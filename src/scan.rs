//! IDX-01: walk the sources, classify, measure, fingerprint, find duplicates and variant groups.

use crate::ids;
use crate::kind::{self, Kind};
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::UNIX_EPOCH;

/// Folders that are never indexed, whatever the ignore files say (IDX-01-R01).
const ALWAYS_SKIP: &[&str] = &["node_modules", "target", ".git", "__pycache__", ".venv", "DerivedData"];
pub const CACHE_NAME: &str = ".opitar-cache.json";

#[derive(Clone, Debug, Serialize)]
pub struct Source {
    /// The path given (canonical): a folder or a single file.
    pub ruta: PathBuf,
    /// Folder that files are served from: the folder itself, or the parent of a single file.
    #[serde(skip)]
    pub raiz: PathBuf,
    pub es_archivo: bool,
    pub nombre: String,
}

impl Source {
    pub fn new(path: &Path) -> std::io::Result<Source> {
        let ruta = path.canonicalize()?;
        let es_archivo = ruta.is_file();
        let raiz = if es_archivo { ruta.parent().unwrap_or(Path::new("/")).to_path_buf() } else { ruta.clone() };
        let nombre = ruta.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| ruta.display().to_string());
        Ok(Source { ruta, raiz, es_archivo, nombre })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Entry {
    pub n: usize,
    pub id: String,
    #[serde(skip)]
    pub digest: [u8; 32],
    pub fuente: usize,
    #[serde(serialize_with = "path_str")]
    pub ruta: PathBuf,
    /// Path relative to the source folder ("sprites/logo.png").
    pub rel: String,
    /// Folder part of `rel` ("sprites"), empty at the top.
    pub carpeta: String,
    pub nombre: String,
    pub ext: String,
    pub tipo: &'static str,
    pub mime: String,
    pub tamano: u64,
    pub modificado: u64,
    pub ancho: Option<u32>,
    pub alto: Option<u32>,
    /// Variant group key (IDX-01-R05), already including the kind.
    pub grupo: String,
    /// How many indexed files share this content (1 = unique).
    pub copias: usize,
    /// Normalised file name and parent-folder name, inputs to `assign_groups` (CAM-002).
    #[serde(skip)]
    pub(crate) base: String,
    #[serde(skip)]
    pub(crate) folder: String,
}

fn path_str<S: serde::Serializer>(p: &Path, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&p.to_string_lossy())
}

#[derive(Serialize, Deserialize, Default)]
struct Cache {
    archivos: HashMap<String, CacheItem>,
}

#[derive(Serialize, Deserialize, Clone)]
struct CacheItem {
    tamano: u64,
    modificado: u64,
    blake3: String,
}

pub struct ScanOptions<'a> {
    pub exclude: Option<&'a Regex>,
    /// Files never indexed (the selection file itself, the cache).
    pub skip: &'a [PathBuf],
    pub cache: Option<&'a Path>,
}

/// Every file of every source, in display order (IDX-01-R08).
pub fn scan(sources: &[Source], opts: &ScanOptions) -> Vec<Entry> {
    let skip: HashSet<&PathBuf> = opts.skip.iter().collect();
    let mut found: Vec<(usize, PathBuf, String)> = Vec::new();
    for (index, source) in sources.iter().enumerate() {
        if source.es_archivo {
            found.push((index, source.ruta.clone(), source.nombre.clone()));
            continue;
        }
        let walker = ignore::WalkBuilder::new(&source.ruta)
            .hidden(true)
            .follow_links(false)
            .require_git(false)
            .git_global(false)
            .add_custom_ignore_filename(".opitarignore")
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                !ALWAYS_SKIP.contains(&name.as_ref()) && name != CACHE_NAME
            })
            .build();
        for item in walker.flatten() {
            if !item.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }
            let path = item.into_path();
            let Ok(canonical) = path.canonicalize() else { continue };
            if skip.contains(&canonical) {
                continue;
            }
            let rel = path.strip_prefix(&source.ruta).unwrap_or(&path).to_string_lossy().replace('\\', "/");
            if opts.exclude.map(|re| re.is_match(&rel)).unwrap_or(false) {
                continue;
            }
            found.push((index, canonical, rel));
        }
    }

    let cache: Cache = opts
        .cache
        .and_then(|p| std::fs::read(p).ok())
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default();

    let mut entries: Vec<Entry> = found
        .par_iter()
        .filter_map(|(fuente, path, rel)| match describe(*fuente, path, rel, &cache) {
            Ok(e) => Some(e),
            Err(err) => {
                eprintln!("opitar: no se pudo leer {}: {}", path.display(), err);
                None
            }
        })
        .collect();

    assign_ids(&mut entries);
    assign_groups(&mut entries);
    entries.sort_by(|a, b| {
        a.fuente.cmp(&b.fuente).then_with(|| natural_cmp(&a.carpeta, &b.carpeta)).then_with(|| natural_cmp(&a.nombre, &b.nombre))
    });
    for (n, e) in entries.iter_mut().enumerate() {
        e.n = n;
    }

    if let Some(path) = opts.cache {
        let mut fresh = Cache::default();
        for e in &entries {
            fresh.archivos.insert(
                e.ruta.to_string_lossy().into_owned(),
                CacheItem { tamano: e.tamano, modificado: e.modificado, blake3: hex(&e.digest) },
            );
        }
        if let Ok(json) = serde_json::to_vec(&fresh) {
            let _ = crate::store::write_atomic(path, &json);
        }
    }
    entries
}

fn describe(fuente: usize, path: &Path, rel: &str, cache: &Cache) -> std::io::Result<Entry> {
    let meta = std::fs::metadata(path)?;
    let tamano = meta.len();
    let modificado = meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_secs()).unwrap_or(0);

    let mut head = vec![0u8; 8192];
    let mut file = File::open(path)?;
    let got = read_up_to(&mut file, &mut head)?;
    head.truncate(got);
    let (k, mime) = kind::classify(path, &head);

    let key = path.to_string_lossy();
    let digest = match cache.archivos.get(key.as_ref()) {
        Some(c) if c.tamano == tamano && c.modificado == modificado => unhex(&c.blake3),
        _ => None,
    };
    let digest = match digest {
        Some(d) => d,
        None => hash_file(path)?,
    };

    let (ancho, alto) = if k == Kind::Image {
        match imagesize::size(path) {
            Ok(s) => (Some(s.width as u32), Some(s.height as u32)),
            Err(_) => (None, None),
        }
    } else {
        (None, None)
    };

    let nombre = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let ext = path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    let carpeta = match rel.rfind('/') {
        Some(i) => rel[..i].to_string(),
        None => String::new(),
    };
    let stem = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    Ok(Entry {
        n: 0,
        id: String::new(),
        digest,
        fuente,
        ruta: path.to_path_buf(),
        rel: rel.to_string(),
        carpeta,
        nombre,
        ext,
        tipo: k.as_str(),
        mime,
        tamano,
        modificado,
        ancho,
        alto,
        grupo: String::new(),
        copias: 1,
        base: variant_key(&stem),
        folder: rel.rsplit('/').nth(1).map(variant_key).unwrap_or_default(),
    })
}

fn read_up_to(file: &mut File, buf: &mut [u8]) -> std::io::Result<usize> {
    let mut total = 0;
    while total < buf.len() {
        let n = file.read(&mut buf[total..])?;
        if n == 0 {
            break;
        }
        total += n;
    }
    Ok(total)
}

pub fn hash_file(path: &Path) -> std::io::Result<[u8; 32]> {
    let mut hasher = blake3::Hasher::new();
    hasher.update_reader(File::open(path)?)?;
    Ok(*hasher.finalize().as_bytes())
}

/// IDX-01-R03 and DEC-003: short IDs, long ones only where two different contents collide; count copies.
pub fn assign_ids(entries: &mut [Entry]) {
    let mut copies: HashMap<[u8; 32], usize> = HashMap::new();
    for e in entries.iter() {
        *copies.entry(e.digest).or_default() += 1;
    }
    let mut by_short: HashMap<String, HashSet<[u8; 32]>> = HashMap::new();
    for d in copies.keys() {
        by_short.entry(ids::format_id(d, false)).or_default().insert(*d);
    }
    for e in entries.iter_mut() {
        let short = ids::format_id(&e.digest, false);
        let collides = by_short.get(&short).map(|s| s.len() > 1).unwrap_or(false);
        e.id = if collides { ids::format_id(&e.digest, true) } else { short };
        e.copias = copies[&e.digest];
    }
}

/// IDX-01-R05 (CAM-002): a name used in several folders of a source takes its folder
/// ("12-oro/cerrada" → "oro-cerrada"); a first word shared by most of a source ("caja-…") is dropped.
pub fn assign_groups(entries: &mut [Entry]) {
    let mut folders: HashMap<(usize, &'static str, String), HashSet<String>> = HashMap::new();
    for e in entries.iter() {
        folders.entry((e.fuente, e.tipo, e.base.clone())).or_default().insert(e.folder.clone());
    }
    let keys: Vec<String> = entries
        .iter()
        .map(|e| {
            let shared = folders[&(e.fuente, e.tipo, e.base.clone())].len() > 1;
            if shared && !e.folder.is_empty() {
                format!("{}-{}", e.folder, e.base)
            } else {
                e.base.clone()
            }
        })
        .collect();
    let mut by_source: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, e) in entries.iter().enumerate() {
        by_source.entry(e.fuente).or_default().push(i);
    }
    let mut final_keys = keys.clone();
    for idx in by_source.values() {
        if idx.len() < 3 {
            continue;
        }
        let mut firsts: HashMap<&str, usize> = HashMap::new();
        for &i in idx {
            if let Some((first, _)) = keys[i].split_once('-') {
                *firsts.entry(first).or_default() += 1;
            }
        }
        if let Some((word, n)) = firsts.into_iter().max_by_key(|(_, n)| *n) {
            if n * 10 >= idx.len() * 6 {
                let prefix = format!("{}-", word);
                for &i in idx {
                    if let Some(rest) = keys[i].strip_prefix(&prefix) {
                        final_keys[i] = rest.to_string();
                    }
                }
            }
        }
    }
    for (e, k) in entries.iter_mut().zip(final_keys) {
        e.grupo = format!("{}:{}", e.tipo, k);
    }
}

fn re(pattern: &'static str, cell: &'static OnceLock<Regex>) -> &'static Regex {
    cell.get_or_init(|| Regex::new(pattern).expect("valid regex"))
}

/// IDX-01-R05: "Logo (2)", "logo_v1", "12-logo-final@2x" → "logo".
pub fn variant_key(stem: &str) -> String {
    static PAREN: OnceLock<Regex> = OnceLock::new();
    static SCALE: OnceLock<Regex> = OnceLock::new();
    static SEP: OnceLock<Regex> = OnceLock::new();
    static LEAD: OnceLock<Regex> = OnceLock::new();
    static TAIL: OnceLock<Regex> = OnceLock::new();
    let mut s = stem.to_lowercase();
    s = re(r"\s*\(\d+\)", &PAREN).replace_all(&s, "").into_owned();
    s = re(r"@\d+(\.\d+)?x$", &SCALE).replace(&s, "").into_owned();
    s = re(r"[^\p{L}\p{N}]+", &SEP).replace_all(&s, "-").trim_matches('-').to_string();
    s = re(r"^\d+-", &LEAD).replace(&s, "").into_owned();
    let tail = re(r"-(?:v\d+|ver\d+|version-?\d*|final|copia|copy|old|new|wip|draft|bak|backup)(?:-?\d+)?$", &TAIL);
    loop {
        let next = tail.replace(&s, "").into_owned();
        if next == s {
            break;
        }
        s = next;
    }
    if s.is_empty() {
        stem.to_lowercase()
    } else {
        s
    }
}

/// Natural order: "2" before "10", case-insensitive.
pub fn natural_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    let (mut x, mut y) = (a.chars().peekable(), b.chars().peekable());
    loop {
        match (x.peek().copied(), y.peek().copied()) {
            (None, None) => return a.cmp(b),
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(c), Some(d)) if c.is_ascii_digit() && d.is_ascii_digit() => {
                let mut n1 = String::new();
                while let Some(c) = x.peek().copied().filter(|c| c.is_ascii_digit()) {
                    n1.push(c);
                    x.next();
                }
                let mut n2 = String::new();
                while let Some(d) = y.peek().copied().filter(|d| d.is_ascii_digit()) {
                    n2.push(d);
                    y.next();
                }
                let (t1, t2) = (n1.trim_start_matches('0'), n2.trim_start_matches('0'));
                let ord = t1.len().cmp(&t2.len()).then_with(|| t1.cmp(t2));
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            (Some(c), Some(d)) => {
                let ord = c.to_lowercase().cmp(d.to_lowercase());
                if ord != Ordering::Equal {
                    return ord;
                }
                x.next();
                y.next();
            }
        }
    }
}

pub fn hex(d: &[u8]) -> String {
    d.iter().map(|b| format!("{:02x}", b)).collect()
}

fn unhex(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, chunk) in out.iter_mut().enumerate() {
        *chunk = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn variant_groups() {
        // IDX-01-A02
        assert_eq!(variant_key("logo_v1"), "logo");
        assert_eq!(variant_key("logo-final"), "logo");
        assert_eq!(variant_key("Logo (2)"), "logo");
        assert_eq!(variant_key("12-logo-final_final@2x"), "logo");
        assert_eq!(variant_key("sprite-01"), "sprite-01");
        assert_eq!(variant_key("final"), "final");
    }

    #[test]
    fn groups_understand_folders_and_shared_prefixes() {
        // IDX-01-A05
        let dir = tempfile::tempdir().unwrap();
        let (a, b) = (dir.path().join("A"), dir.path().join("B"));
        for d in ["12-oro", "13-hielo"] {
            fs::create_dir_all(a.join(d)).unwrap();
            fs::write(a.join(d).join("cerrada.png"), d).unwrap();
        }
        fs::create_dir_all(&b).unwrap();
        for n in ["oro", "hielo", "pino"] {
            fs::write(b.join(format!("caja-{}-cerrada.png", n)), format!("b{}", n)).unwrap();
        }
        let sources = vec![Source::new(&a).unwrap(), Source::new(&b).unwrap()];
        let entries = scan(&sources, &ScanOptions { exclude: None, skip: &[], cache: None });
        let g = |rel: &str, f: usize| entries.iter().find(|e| e.rel == rel && e.fuente == f).unwrap().grupo.clone();
        assert_eq!(g("12-oro/cerrada.png", 0), "imagen:oro-cerrada");
        assert_eq!(g("12-oro/cerrada.png", 0), g("caja-oro-cerrada.png", 1));
        assert_ne!(g("12-oro/cerrada.png", 0), g("13-hielo/cerrada.png", 0));
    }

    #[test]
    fn natural_order() {
        let mut v = vec!["f10", "f2", "F1"];
        v.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(v, vec!["F1", "f2", "f10"]);
    }

    #[test]
    fn walk_respects_ignores_and_finds_duplicates() {
        // IDX-01-A03, IDX-01-A04
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("node_modules")).unwrap();
        fs::create_dir_all(root.join("tmp")).unwrap();
        fs::create_dir_all(root.join("a")).unwrap();
        fs::write(root.join("node_modules/x.png"), b"x").unwrap();
        fs::write(root.join("tmp/y.png"), b"y").unwrap();
        fs::write(root.join(".gitignore"), b"tmp/\n").unwrap();
        fs::write(root.join("uno.txt"), b"igual").unwrap();
        fs::write(root.join("a/dos.txt"), b"igual").unwrap();
        fs::write(root.join("otro.wav"), b"RIFF....").unwrap();
        let sources = vec![Source::new(root).unwrap()];
        let cache = root.join(CACHE_NAME);
        let opts = ScanOptions { exclude: None, skip: &[], cache: Some(&cache) };
        let entries = scan(&sources, &opts);
        let rels: Vec<&str> = entries.iter().map(|e| e.rel.as_str()).collect();
        assert_eq!(rels, vec!["otro.wav", "uno.txt", "a/dos.txt"]);
        let uno = entries.iter().find(|e| e.nombre == "uno.txt").unwrap();
        let dos = entries.iter().find(|e| e.nombre == "dos.txt").unwrap();
        assert_eq!(uno.id, dos.id);
        assert_eq!(uno.copias, 2);
        assert!(cache.exists());
        // Second scan uses the cache and gives the same IDs.
        let again = scan(&sources, &opts);
        assert_eq!(again.iter().map(|e| &e.id).collect::<Vec<_>>(), entries.iter().map(|e| &e.id).collect::<Vec<_>>());
    }
}
