//! Opitar — a file picker for developers.
//! Copyright 2026 SekaiBuilder. Licensed under the Apache License, Version 2.0 (see LICENSE and NOTICE).

mod ids;
mod kind;
mod scan;
mod server;
mod store;

use clap::Parser;
use regex::Regex;
use scan::{Entry, ScanOptions, Source};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Opitar: put any folder or file in a table, with a content ID, reference search and a viewer
/// for image, audio, video, 3D, PDF, text and fonts. Pick what stays; the choice is saved at once
/// to a JSON file.
#[derive(Parser)]
#[command(name = "opitar", version)]
struct Cli {
    /// Folders or files (each one is a source). Defaults to the current folder.
    paths: Vec<PathBuf>,
    /// JSON file where the selection is saved (default: opitar.json next to the first source).
    #[arg(long, short, alias = "salida")]
    output: Option<PathBuf>,
    /// Page title (default: the name of the first source).
    #[arg(long, alias = "titulo")]
    title: Option<String>,
    /// Local port; 0 picks a free one.
    #[arg(long, default_value_t = 0, alias = "puerto")]
    port: u16,
    /// Do not open the browser.
    #[arg(long, alias = "no-abrir")]
    no_open: bool,
    /// Regular expression: hides files whose relative path matches.
    #[arg(long, alias = "excluir")]
    exclude: Option<String>,
    /// Print the table (ID, kind, size, path) and exit.
    #[arg(long, alias = "lista")]
    list: bool,
    /// Find by reference (full or partial ID, name or path), print and exit.
    #[arg(long, value_name = "REF", alias = "buscar")]
    find: Option<String>,
}

fn human(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut v = bytes as f64;
    let mut u = 0;
    while v >= 1024.0 && u < units.len() - 1 {
        v /= 1024.0;
        u += 1;
    }
    if u == 0 {
        format!("{} B", bytes)
    } else {
        format!("{:.1} {}", v, units[u])
    }
}

fn matches(e: &Entry, query: &str) -> bool {
    let id_q = ids::normalize_query(query);
    let low = query.to_lowercase();
    (!id_q.is_empty() && ids::compact(&e.id).contains(&id_q)) || e.rel.to_lowercase().contains(&low)
}

fn print_table(entries: &[Entry], sources: &[Source]) {
    for e in entries {
        let prefix = if sources.len() > 1 { format!("{}/", sources[e.source].name) } else { String::new() };
        let dup = if e.copies > 1 { format!("  (duplicate ×{})", e.copies) } else { String::new() };
        println!("{:<17} {:<9} {:>10}  {}{}{}", e.id, e.kind, human(e.size), prefix, e.rel, dup);
    }
}

fn token() -> String {
    let mut h = blake3::Hasher::new();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    h.update(&now.to_le_bytes());
    h.update(&std::process::id().to_le_bytes());
    let stack = 0u8;
    h.update(&(&stack as *const u8 as usize).to_le_bytes());
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        let mut buf = [0u8; 32];
        if std::io::Read::read_exact(&mut f, &mut buf).is_ok() {
            h.update(&buf);
        }
    }
    scan::hex(&h.finalize().as_bytes()[..16])
}

fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(url).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd").args(["/C", "start", "", url]).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let paths = if cli.paths.is_empty() { vec![PathBuf::from(".")] } else { cli.paths.clone() };
    let mut sources = Vec::new();
    for p in &paths {
        match Source::new(p) {
            Ok(s) => sources.push(s),
            Err(e) => {
                eprintln!("opitar: {}: {}", p.display(), e);
                return ExitCode::from(2);
            }
        }
    }
    let exclude = match cli.exclude.as_deref().map(Regex::new).transpose() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("opitar: --exclude is not a valid regular expression: {}", e);
            return ExitCode::from(2);
        }
    };

    let first = &sources[0];
    let output = match &cli.output {
        Some(p) => std::env::current_dir().map(|d| d.join(p)).unwrap_or_else(|_| p.clone()),
        None => first.root.join("opitar.json"),
    };
    let cache = output.parent().map(|p| p.join(scan::CACHE_NAME)).unwrap_or_else(|| PathBuf::from(scan::CACHE_NAME));
    let skip = vec![output.canonicalize().unwrap_or(output.clone()), cache.canonicalize().unwrap_or(cache.clone())];

    let started = std::time::Instant::now();
    let opts = ScanOptions { exclude: exclude.as_ref(), skip: &skip, cache: Some(&cache) };
    let entries = scan::scan(&sources, &opts);

    if cli.list {
        print_table(&entries, &sources);
        return ExitCode::SUCCESS;
    }
    if let Some(q) = &cli.find {
        let hits: Vec<Entry> = entries.into_iter().filter(|e| matches(e, q)).collect();
        print_table(&hits, &sources);
        return if hits.is_empty() { ExitCode::from(1) } else { ExitCode::SUCCESS };
    }

    let store = match store::Store::open(&output) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("opitar: {}", e);
            return ExitCode::from(2);
        }
    };
    let title = cli.title.clone().unwrap_or_else(|| first.name.clone());
    let server = match server::bind(cli.port) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("opitar: {}", e);
            return ExitCode::from(2);
        }
    };
    let port = server.server_addr().to_ip().map(|a| a.port()).unwrap_or(cli.port);
    let token = token();
    let url = format!("http://127.0.0.1:{}/?t={}", port, token);
    let dups = entries.iter().filter(|e| e.copies > 1).count();
    println!(
        "Opitar · {}: {} files from {} source(s) in {:.1} s{}",
        title,
        entries.len(),
        sources.len(),
        started.elapsed().as_secs_f64(),
        if dups > 0 { format!(" · {} duplicates", dups) } else { String::new() }
    );
    println!("Selection saved to {}", output.display());
    println!("{}", url);
    println!("Ctrl+C to quit.");

    let app = Arc::new(server::App {
        token,
        title,
        sources,
        entries: RwLock::new(entries),
        store: Mutex::new(store),
        exclude,
        cache,
    });
    if !cli.no_open {
        open_browser(&url);
    }
    server::run(server, app);
    ExitCode::SUCCESS
}
