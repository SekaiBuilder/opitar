# CAM-001 — Plan

1. `Cargo.toml` with `clap`, `serde`, `serde_json`, `blake3`, `ignore`, `imagesize`, `infer`, `tiny_http`, `rayon`, `regex`.
2. `src/kind.rs`: extension → kind and MIME table; signature via `infer`; text sniff.
3. `src/ids.rs`: Crockford base32, `OP-XXXX-XXXX` format, search normalisation.
4. `src/scan.rs`: walk with `ignore::WalkBuilder`, cache, parallel hashing, duplicates, variant groups, natural order.
5. `src/store.rs`: load, per-file and per-group changes, atomic save, summary, CSV.
6. `src/server.rs`: threaded `tiny_http`, token, routes `/`, `/v/*` (embedded assets), `/api/*`, `/f/<token>/<n>/…` with `Range` and sibling resources (glTF textures and `.bin`).
7. `src/main.rs`: CLI, list and find modes, open the browser.
8. `assets/index.html`: full UI; `assets/vendor/three/` with three.js 0.160.0.
9. Unit tests per criterion; real browser walk-through with a sample folder.

Validation: `cargo test`, `cargo build --release`, browser walk-through.
