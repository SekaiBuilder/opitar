# Decisions

## Recorded

| ID | Decision | Origin | Discarded alternatives |
|---|---|---|---|
| DEC-001 | Rust. | Owner, 2026-11-25 | — |
| DEC-002 | One binary serving an embedded local web UI (not a native GUI). The browser already plays image, audio, video, PDF and fonts, and WebGL covers 3D; the Notion look is just CSS. | Proposal, for format coverage with little code | egui (no native video or PDF), Tauri (heavier to build and ship; kept as an option, DEC-101) |
| DEC-003 | ID = `OP-` + 8 Crockford base32 chars of the first 40 bits of the content's BLAKE3, in two blocks (`OP-7K3M-Q9XA`). Content-based so it survives renames and moves (C1, C2). 40 bits: collisions unlikely below ~1 million files; if two different contents collide, both get 12 chars. | Proposal | Random UUID (does not survive moves without a database), path (changes on rename) |
| DEC-004 | three.js 0.160.0 copied into `assets/vendor/three/` (MIT) and embedded in the binary, so there is no network (CTR-05). | Proposal | Loading it from a CDN |
| DEC-005 | Apache-2.0 licence with a `NOTICE` naming the author; redistributors must keep the `NOTICE` (section 4(d)). | Owner, 2026-11-25 | — |
| DEC-006 | HTTP server with `tiny_http` (synchronous, few dependencies) and hand-written `Range` support so video/audio can seek. | Proposal | axum + tokio (more dependencies than needed) |
| DEC-007 | Everything in English: UI, CLI, docs, specs and data format (v2). Spanish CLI flags kept as hidden aliases; v1 files migrated on read. | Owner, 2026-11-25 (CAM-003) | Keeping Spanish data keys |

## Open

| ID | Question | Depends on |
|---|---|---|
| DEC-101 | Package as a desktop app (Tauri) besides the binary? | Whether the browser gets in the way day to day |
| DEC-102 | Thumbnails generated on disk for folders with tens of thousands of images? | Measuring a large real project |
| DEC-103 | Show the layers of one item (closed/back/front) on a single Compare row? | Real use with the boxes |
