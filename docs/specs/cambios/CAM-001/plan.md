# CAM-001 — Plan

1. `Cargo.toml` con `clap`, `serde`, `serde_json`, `blake3`, `ignore`, `imagesize`, `infer`, `tiny_http`, `rayon`, `regex`.
2. `src/kind.rs`: tabla extensión → tipo y MIME; firma con `infer`; prueba de texto.
3. `src/ids.rs`: Crockford base32, formato `OP-XXXX-XXXX`, normalización de búsqueda.
4. `src/scan.rs`: recorrido con `ignore::WalkBuilder`, caché, huellas en paralelo, duplicados, grupos de variantes, orden natural.
5. `src/store.rs`: carga, cambios por archivo y por grupo, guardado atómico, resumen, CSV.
6. `src/server.rs`: `tiny_http` en hilos, token, rutas `/`, `/v/*` (assets embebidos), `/api/*`, `/f/<n>` con `Range`, `/r/<fuente>/<ruta>` para recursos hermanos (texturas y `.bin` de glTF).
7. `src/main.rs`: CLI, modos `--lista` y `--buscar`, abrir navegador.
8. `assets/index.html`: interfaz completa; `assets/vendor/three/` con three.js 0.160.0.
9. Tests unitarios por criterio; recorrido real en el navegador con una carpeta de muestra (imagen, audio, vídeo, glb, pdf, txt, ttf).

Validación: `cargo test`, `cargo build --release`, recorrido en navegador.
