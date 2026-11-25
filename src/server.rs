//! UI-01-R02: local HTTP server. 127.0.0.1 only, per-session token, files only from inside a source.

use crate::scan::{self, Entry, ScanOptions, Source};
use crate::store::{Change, Store};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

const PAGE: &str = include_str!("../assets/index.html");

/// three.js and addons, embedded so the page never touches the network (UI-01-R13, DEC-004).
const VENDOR: &[(&str, &[u8])] = &[
    ("three/three.module.min.js", include_bytes!("../assets/vendor/three/three.module.min.js")),
    ("three/addons/controls/OrbitControls.js", include_bytes!("../assets/vendor/three/addons/controls/OrbitControls.js")),
    ("three/addons/loaders/GLTFLoader.js", include_bytes!("../assets/vendor/three/addons/loaders/GLTFLoader.js")),
    ("three/addons/loaders/OBJLoader.js", include_bytes!("../assets/vendor/three/addons/loaders/OBJLoader.js")),
    ("three/addons/loaders/MTLLoader.js", include_bytes!("../assets/vendor/three/addons/loaders/MTLLoader.js")),
    ("three/addons/loaders/STLLoader.js", include_bytes!("../assets/vendor/three/addons/loaders/STLLoader.js")),
    ("three/addons/loaders/FBXLoader.js", include_bytes!("../assets/vendor/three/addons/loaders/FBXLoader.js")),
    ("three/addons/loaders/PLYLoader.js", include_bytes!("../assets/vendor/three/addons/loaders/PLYLoader.js")),
    ("three/addons/libs/fflate.module.js", include_bytes!("../assets/vendor/three/addons/libs/fflate.module.js")),
    ("three/addons/utils/BufferGeometryUtils.js", include_bytes!("../assets/vendor/three/addons/utils/BufferGeometryUtils.js")),
    ("three/addons/curves/NURBSCurve.js", include_bytes!("../assets/vendor/three/addons/curves/NURBSCurve.js")),
    ("three/addons/curves/NURBSUtils.js", include_bytes!("../assets/vendor/three/addons/curves/NURBSUtils.js")),
];

pub struct App {
    pub token: String,
    pub title: String,
    pub sources: Vec<Source>,
    pub entries: RwLock<Vec<Entry>>,
    pub store: Mutex<Store>,
    pub exclude: Option<Regex>,
    pub cache: PathBuf,
}

impl App {
    fn source_paths(&self) -> Vec<String> {
        self.sources.iter().map(|s| s.path.to_string_lossy().into_owned()).collect()
    }

    pub fn rescan(&self) {
        let skip = vec![self.store.lock().unwrap().path.clone(), self.cache.clone()];
        let opts = ScanOptions { exclude: self.exclude.as_ref(), skip: &skip, cache: Some(&self.cache) };
        let fresh = scan::scan(&self.sources, &opts);
        *self.entries.write().unwrap() = fresh;
    }

    fn save(&self, store: &mut Store) -> Result<(), String> {
        let entries = self.entries.read().unwrap();
        store.save(&self.title, &self.source_paths(), &entries)
    }
}

pub fn bind(port: u16) -> Result<Server, String> {
    Server::http(("127.0.0.1", port)).map_err(|e| format!("could not open port {}: {}", port, e))
}

pub fn run(server: Server, app: Arc<App>) {
    let server = Arc::new(server);
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let (server, app) = (server.clone(), app.clone());
            std::thread::spawn(move || {
                while let Ok(request) = server.recv() {
                    handle(&app, request);
                }
            })
        })
        .collect();
    for w in workers {
        let _ = w.join();
    }
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).expect("valid header")
}

fn text(code: u16, body: &str, content_type: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_data(body.as_bytes().to_vec())
        .with_status_code(code)
        .with_header(header("Content-Type", content_type))
        .with_header(header("Cache-Control", "no-store"))
        .with_header(header("X-Content-Type-Options", "nosniff"))
}

fn json_response(code: u16, value: serde_json::Value) -> Response<std::io::Cursor<Vec<u8>>> {
    text(code, &value.to_string(), "application/json; charset=utf-8")
}

fn split_url(url: &str) -> (&str, &str) {
    match url.split_once('?') {
        Some((p, q)) => (p, q),
        None => (url, ""),
    }
}

fn query_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|kv| kv.split_once('=').filter(|(k, _)| *k == key).map(|(_, v)| v))
}

pub fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn authorized(app: &App, request: &Request, query: &str) -> bool {
    let from_header = request.headers().iter().find(|h| h.field.equiv("X-Opitar-Token")).map(|h| h.value.as_str().to_string());
    let given = from_header.or_else(|| query_param(query, "t").map(String::from));
    given.as_deref() == Some(app.token.as_str())
}

fn read_body(request: &mut Request, limit: usize) -> Option<Vec<u8>> {
    let mut body = Vec::new();
    request.as_reader().take(limit as u64 + 1).read_to_end(&mut body).ok()?;
    (body.len() <= limit).then_some(body)
}

fn handle(app: &App, mut request: Request) {
    let url = request.url().to_string();
    let (path, query) = split_url(&url);
    let method = request.method().clone();

    // The page and the vendored libraries carry no user data and are served without the token.
    if method == Method::Get && path == "/" {
        let resp = if query_param(query, "t") == Some(app.token.as_str()) {
            text(200, PAGE, "text/html; charset=utf-8")
        } else {
            text(403, "Opitar: open the full URL printed in the terminal (it carries the session token).", "text/plain; charset=utf-8")
        };
        let _ = request.respond(resp);
        return;
    }
    if method == Method::Get && path.starts_with("/v/") {
        let name = &path[3..];
        let resp = match VENDOR.iter().find(|(n, _)| *n == name) {
            Some((_, bytes)) => Response::from_data(bytes.to_vec())
                .with_header(header("Content-Type", "text/javascript; charset=utf-8"))
                .with_header(header("Cache-Control", "max-age=86400")),
            None => text(404, "no", "text/plain"),
        };
        let _ = request.respond(resp);
        return;
    }
    if method == Method::Get && path == "/favicon.ico" {
        let _ = request.respond(text(404, "", "text/plain"));
        return;
    }

    // Files: /f/<token>/<n>/<name or sibling path>. The token sits in the path so relative URLs
    // (glTF buffers and textures, OBJ materials) keep it.
    if method == Method::Get && path.starts_with("/f/") {
        let rest = &path[3..];
        let mut parts = rest.splitn(3, '/');
        let (tok, n, sub) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""), parts.next().unwrap_or(""));
        if tok != app.token {
            let _ = request.respond(text(403, "token", "text/plain"));
            return;
        }
        let target = n.parse::<usize>().ok().and_then(|n| resolve_file(app, n, &percent_decode(sub)));
        match target {
            Some((file, mime)) => serve_file(request, &file, &mime),
            None => {
                let _ = request.respond(text(404, "no", "text/plain"));
            }
        }
        return;
    }

    if !authorized(app, &request, query) {
        let _ = request.respond(text(403, "token", "text/plain"));
        return;
    }

    let resp = match (method, path) {
        (Method::Get, "/api/data") => {
            let entries = app.entries.read().unwrap();
            let store = app.store.lock().unwrap();
            json_response(
                200,
                json!({
                    "title": app.title,
                    "sources": app.sources,
                    "files": *entries,
                    "marks": store.data.files,
                    "output": store.path.to_string_lossy(),
                }),
            )
        }
        (Method::Get, "/api/export.csv") => {
            let entries = app.entries.read().unwrap();
            let store = app.store.lock().unwrap();
            text(200, &store.csv(&entries), "text/csv; charset=utf-8")
                .with_header(header("Content-Disposition", "attachment; filename=\"opitar.csv\""))
        }
        (Method::Get, p) if p.starts_with("/api/text/") => match entry_path(app, &p[10..]) {
            Some(file) => {
                let mut buf = Vec::new();
                let ok = File::open(&file).and_then(|f| f.take(200_000).read_to_end(&mut buf)).is_ok();
                if ok {
                    text(200, &String::from_utf8_lossy(&buf), "text/plain; charset=utf-8")
                } else {
                    text(500, "could not read", "text/plain")
                }
            }
            None => text(404, "no", "text/plain"),
        },
        (Method::Get, p) if p.starts_with("/api/hex/") => match entry_path(app, &p[9..]) {
            Some(file) => {
                let mut buf = Vec::new();
                let _ = File::open(&file).and_then(|f| f.take(512).read_to_end(&mut buf));
                text(200, &hexdump(&buf), "text/plain; charset=utf-8")
            }
            None => text(404, "no", "text/plain"),
        },
        (Method::Post, "/api/mark") => match read_body(&mut request, 64 * 1024).and_then(|b| serde_json::from_slice::<Change>(&b).ok()) {
            Some(change) => {
                let path = route_of(app, &change.id);
                let mut store = app.store.lock().unwrap();
                match store.apply(change, &path).and_then(|_| app.save(&mut store)) {
                    Ok(()) => json_response(200, json!({ "ok": true, "marks": store.data.files })),
                    Err(e) => json_response(400, json!({ "ok": false, "error": e })),
                }
            }
            None => json_response(400, json!({ "ok": false, "error": "malformed request" })),
        },
        (Method::Post, "/api/group") => {
            #[derive(Deserialize)]
            struct Pick {
                id: String,
            }
            match read_body(&mut request, 4096).and_then(|b| serde_json::from_slice::<Pick>(&b).ok()) {
                Some(pick) => {
                    let group: Vec<(String, String)> = {
                        let entries = app.entries.read().unwrap();
                        let key = entries.iter().find(|e| e.id == pick.id).map(|e| e.group.clone());
                        let mut seen = std::collections::HashSet::new();
                        entries
                            .iter()
                            .filter(|e| Some(&e.group) == key.as_ref() && seen.insert(e.id.clone()))
                            .map(|e| (e.id.clone(), e.path.to_string_lossy().into_owned()))
                            .collect()
                    };
                    if group.is_empty() {
                        json_response(404, json!({ "ok": false, "error": "unknown ID" }))
                    } else {
                        let mut store = app.store.lock().unwrap();
                        store.choose_in_group(&pick.id, &group);
                        match app.save(&mut store) {
                            Ok(()) => json_response(200, json!({ "ok": true, "marks": store.data.files })),
                            Err(e) => json_response(500, json!({ "ok": false, "error": e })),
                        }
                    }
                }
                None => json_response(400, json!({ "ok": false, "error": "malformed request" })),
            }
        }
        (Method::Post, "/api/rescan") => {
            app.rescan();
            let mut store = app.store.lock().unwrap();
            let _ = app.save(&mut store);
            json_response(200, json!({ "ok": true }))
        }
        (Method::Post, "/api/reveal") => {
            #[derive(Deserialize)]
            struct Reveal {
                n: usize,
            }
            match read_body(&mut request, 1024).and_then(|b| serde_json::from_slice::<Reveal>(&b).ok()) {
                Some(r) => match entry_path(app, &r.n.to_string()) {
                    Some(file) => {
                        reveal(&file);
                        json_response(200, json!({ "ok": true }))
                    }
                    None => json_response(404, json!({ "ok": false })),
                },
                None => json_response(400, json!({ "ok": false })),
            }
        }
        // UI-01-R10: a dropped file is hashed here (streamed, never stored) to find it in the index.
        (Method::Post, "/api/fingerprint") => {
            let mut hasher = blake3::Hasher::new();
            match hasher.update_reader(request.as_reader()) {
                Ok(_) => {
                    let digest = *hasher.finalize().as_bytes();
                    let entries = app.entries.read().unwrap();
                    let found: Vec<usize> = entries.iter().filter(|e| e.digest == digest).map(|e| e.n).collect();
                    let id = entries
                        .iter()
                        .find(|e| e.digest == digest)
                        .map(|e| e.id.clone())
                        .unwrap_or_else(|| crate::ids::format_id(&digest, false));
                    json_response(200, json!({ "id": id, "indexed": found }))
                }
                Err(_) => json_response(400, json!({ "ok": false })),
            }
        }
        _ => text(404, "no", "text/plain"),
    };
    let _ = request.respond(resp);
}

fn route_of(app: &App, id: &str) -> String {
    let entries = app.entries.read().unwrap();
    entries.iter().find(|e| e.id == id).map(|e| e.path.to_string_lossy().into_owned()).unwrap_or_default()
}

fn entry_path(app: &App, n: &str) -> Option<PathBuf> {
    let n: usize = n.parse().ok()?;
    app.entries.read().unwrap().get(n).map(|e| e.path.clone())
}

/// The indexed file `n`, or a sibling path relative to its folder that stays inside the source (CTR-05).
fn resolve_file(app: &App, n: usize, sub: &str) -> Option<(PathBuf, String)> {
    let entries = app.entries.read().unwrap();
    let entry = entries.get(n)?;
    if sub.is_empty() || sub == entry.name {
        return Some((entry.path.clone(), entry.mime.clone()));
    }
    let rel = Path::new(sub);
    let safe = rel.components().all(|c| matches!(c, Component::Normal(s) if !s.to_string_lossy().starts_with('.')));
    if !safe {
        return None;
    }
    let base = entry.path.parent()?;
    let target = base.join(rel).canonicalize().ok()?;
    let root = &app.sources.get(entry.source)?.root;
    if !target.starts_with(root) || !target.is_file() {
        return None;
    }
    let mime = crate::kind::classify(&target, &[]).1;
    Some((target, mime))
}

/// Byte ranges so audio and video can seek (UI-01-R02).
fn parse_range(value: &str, len: u64) -> Option<(u64, u64)> {
    let spec = value.strip_prefix("bytes=")?.split(',').next()?.trim();
    let (a, b) = spec.split_once('-')?;
    let (start, end) = if a.is_empty() {
        let n: u64 = b.parse().ok()?;
        (len.saturating_sub(n), len.checked_sub(1)?)
    } else {
        let start: u64 = a.parse().ok()?;
        let end = if b.is_empty() { len.checked_sub(1)? } else { b.parse::<u64>().ok()?.min(len.checked_sub(1)?) };
        (start, end)
    };
    (start <= end && end < len).then_some((start, end))
}

fn serve_file(request: Request, path: &Path, mime: &str) {
    let Ok(mut file) = File::open(path) else {
        let _ = request.respond(text(404, "no", "text/plain"));
        return;
    };
    let len = file.metadata().map(|m| m.len()).unwrap_or(0);
    let range = request.headers().iter().find(|h| h.field.equiv("Range")).map(|h| h.value.as_str().to_string());
    match range.as_deref().map(|r| parse_range(r, len)) {
        Some(Some((start, end))) => {
            if file.seek(SeekFrom::Start(start)).is_err() {
                let _ = request.respond(text(500, "seek", "text/plain"));
                return;
            }
            let size = end - start + 1;
            let resp = Response::new(StatusCode(206), vec![], file.take(size), Some(size as usize), None)
                .with_header(header("Content-Range", &format!("bytes {}-{}/{}", start, end, len)));
            let _ = request.respond(common(resp, mime));
        }
        Some(None) => {
            let resp = text(416, "", "text/plain").with_header(header("Content-Range", &format!("bytes */{}", len)));
            let _ = request.respond(resp);
        }
        None => {
            let resp = Response::new(StatusCode(200), vec![], file, Some(len as usize), None);
            let _ = request.respond(common(resp, mime));
        }
    }
}

fn common<R: Read>(r: Response<R>, mime: &str) -> Response<R> {
    r.with_header(header("Content-Type", mime))
        .with_header(header("Accept-Ranges", "bytes"))
        .with_header(header("Cache-Control", "private, max-age=60"))
        .with_header(header("X-Content-Type-Options", "nosniff"))
}

fn hexdump(bytes: &[u8]) -> String {
    let mut out = String::new();
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let hex: Vec<String> = chunk.iter().map(|b| format!("{:02x}", b)).collect();
        let ascii: String = chunk.iter().map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' }).collect();
        out.push_str(&format!("{:08x}  {:<48}  {}\n", i * 16, hex.join(" "), ascii));
    }
    out
}

/// UI-01-R08: show the file in Finder (macOS) or open its folder elsewhere.
fn reveal(path: &Path) {
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg("-R").arg(path).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("explorer").arg(format!("/select,{}", path.display())).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let _ = std::process::Command::new("xdg-open").arg(path.parent().unwrap_or(path)).spawn();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges() {
        assert_eq!(parse_range("bytes=0-99", 1000), Some((0, 99)));
        assert_eq!(parse_range("bytes=900-", 1000), Some((900, 999)));
        assert_eq!(parse_range("bytes=-100", 1000), Some((900, 999)));
        assert_eq!(parse_range("bytes=0-5000", 1000), Some((0, 999)));
        assert_eq!(parse_range("bytes=2000-", 1000), None);
        assert_eq!(parse_range("bytes=0-0", 0), None);
    }

    #[test]
    fn decoding() {
        assert_eq!(percent_decode("a%20b%C3%B1"), "a bñ");
        assert_eq!(percent_decode("100%"), "100%");
    }
}
