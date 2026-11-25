//! Opitar — selector de archivos para desarrolladores.
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

/// Opitar: pon cualquier carpeta o archivo en una tabla, con un ID por contenido, buscador por
/// referencia y un visor para imagen, audio, vídeo, 3D, PDF, texto y fuentes. Elige qué se queda;
/// la elección se guarda al momento en un JSON.
#[derive(Parser)]
#[command(name = "opitar", version)]
struct Cli {
    /// Carpetas o archivos (cada uno es una fuente). Por defecto, la carpeta actual.
    rutas: Vec<PathBuf>,
    /// JSON donde se guarda la selección (por defecto opitar.json junto a la primera fuente).
    #[arg(long, short)]
    salida: Option<PathBuf>,
    /// Título de la página (por defecto, el nombre de la primera fuente).
    #[arg(long)]
    titulo: Option<String>,
    /// Puerto local; 0 elige uno libre.
    #[arg(long, default_value_t = 0)]
    puerto: u16,
    /// No abrir el navegador.
    #[arg(long)]
    no_abrir: bool,
    /// Expresión regular: oculta los archivos cuya ruta relativa coincide.
    #[arg(long)]
    excluir: Option<String>,
    /// Imprime la tabla (ID, tipo, tamaño, ruta) y sale.
    #[arg(long)]
    lista: bool,
    /// Busca por referencia (ID completo o parcial, nombre o ruta), imprime y sale.
    #[arg(long, value_name = "REF")]
    buscar: Option<String>,
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
        let prefix = if sources.len() > 1 { format!("{}/", sources[e.fuente].nombre) } else { String::new() };
        let dup = if e.copias > 1 { format!("  (duplicado ×{})", e.copias) } else { String::new() };
        println!("{:<17} {:<10} {:>10}  {}{}{}", e.id, e.tipo, human(e.tamano), prefix, e.rel, dup);
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
    let rutas = if cli.rutas.is_empty() { vec![PathBuf::from(".")] } else { cli.rutas.clone() };
    let mut sources = Vec::new();
    for r in &rutas {
        match Source::new(r) {
            Ok(s) => sources.push(s),
            Err(e) => {
                eprintln!("opitar: {}: {}", r.display(), e);
                return ExitCode::from(2);
            }
        }
    }
    let exclude = match cli.excluir.as_deref().map(Regex::new).transpose() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("opitar: --excluir no es una expresión regular válida: {}", e);
            return ExitCode::from(2);
        }
    };

    let first = &sources[0];
    let salida = match &cli.salida {
        Some(p) => std::env::current_dir().map(|d| d.join(p)).unwrap_or_else(|_| p.clone()),
        None => first.raiz.join("opitar.json"),
    };
    let cache = salida.parent().map(|p| p.join(scan::CACHE_NAME)).unwrap_or_else(|| PathBuf::from(scan::CACHE_NAME));
    let skip = vec![salida.canonicalize().unwrap_or(salida.clone()), cache.canonicalize().unwrap_or(cache.clone())];

    let started = std::time::Instant::now();
    let opts = ScanOptions { exclude: exclude.as_ref(), skip: &skip, cache: Some(&cache) };
    let entries = scan::scan(&sources, &opts);

    if cli.lista {
        print_table(&entries, &sources);
        return ExitCode::SUCCESS;
    }
    if let Some(q) = &cli.buscar {
        let hits: Vec<Entry> = entries.into_iter().filter(|e| matches(e, q)).collect();
        print_table(&hits, &sources);
        return if hits.is_empty() { ExitCode::from(1) } else { ExitCode::SUCCESS };
    }

    let store = match store::Store::open(&salida) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("opitar: {}", e);
            return ExitCode::from(2);
        }
    };
    let titulo = cli.titulo.clone().unwrap_or_else(|| first.nombre.clone());
    let server = match server::bind(cli.puerto) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("opitar: {}", e);
            return ExitCode::from(2);
        }
    };
    let port = server.server_addr().to_ip().map(|a| a.port()).unwrap_or(cli.puerto);
    let token = token();
    let url = format!("http://127.0.0.1:{}/?t={}", port, token);
    let dups = entries.iter().filter(|e| e.copias > 1).count();
    println!(
        "Opitar · {}: {} archivos de {} fuente(s) en {:.1} s{}",
        titulo,
        entries.len(),
        sources.len(),
        started.elapsed().as_secs_f64(),
        if dups > 0 { format!(" · {} duplicados", dups) } else { String::new() }
    );
    println!("Selección en {}", salida.display());
    println!("{}", url);
    println!("Ctrl+C para salir.");

    let app = Arc::new(server::App {
        token,
        titulo,
        sources,
        entries: RwLock::new(entries),
        store: Mutex::new(store),
        exclude,
        cache,
    });
    if !cli.no_abrir {
        open_browser(&url);
    }
    server::run(server, app);
    ExitCode::SUCCESS
}
