# Module map

| ID | Module | Responsibility | Files | Spec |
|---|---|---|---|---|
| MOD-01 | Index | Walk paths, classify by kind, measure, hash and ID, group variants, cache | `src/scan.rs`, `src/kind.rs`, `src/ids.rs` | [IDX-01](IDX-01-index.md) |
| MOD-02 | Selection | Per-file state, atomic JSON save, summary, CSV, v1 migration | `src/store.rs` | [SEL-01](SEL-01-selection.md) |
| MOD-03 | Server | Local HTTP with token, API routes and files with ranges | `src/server.rs` | [UI-01](UI-01-interface.md) |
| MOD-04 | Interface | Single page: table, gallery, compare, search, viewer | `assets/index.html`, `assets/vendor/three/` | [UI-01](UI-01-interface.md) |
| MOD-05 | CLI | Arguments, list and find modes, opening the browser | `src/main.rs` | [UI-01](UI-01-interface.md) |
