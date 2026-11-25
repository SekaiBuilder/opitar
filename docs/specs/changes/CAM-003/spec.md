# CAM-003 — Everything in English, selection format v2

Status: implemented and verified. Modules: MOD-01..MOD-05. Journeys: all. Contracts: CTR-02, CTR-04, CTR-07.
Base: commit `76e6022`, 2026-11-25.

## Problem and result

- **Problem:** the owner works in English by default (stated 2026-11-25). Opitar was written in Spanish: UI, CLI flags and messages, specs, README, and the keys and values of `opitar.json` (`archivos`, `estado: "queda"`…). An English-language open-source tool with Spanish data keys is hard to adopt.
- **Result:** UI, CLI, messages, docs, specs and data format are in English. Existing `opitar.json` files (v1, Spanish keys) still open and keep every choice.

## Baseline changes

- Modified UI-01-R01: flags are `--output/-o`, `--title`, `--port`, `--no-open`, `--exclude`, `--list`, `--find REF`. The Spanish flags (`--salida`, `--titulo`, `--puerto`, `--no-abrir`, `--excluir`, `--lista`, `--buscar`) stay as hidden aliases.
- Modified UI-01-R04: filters are `kind:`, `ext:`, `state:` (`keep`, `out`, `pending`), `#tag` and `dup`.
- Modified IDX-01-R02: kind names are `image`, `audio`, `video`, `3d`, `document`, `font`, `code`, `archive`, `other`.
- Modified SEL-01-R02, R04, R06: format v2 (below). CSV columns `id,state,kind,path,size,width,height,tags,note`.
- Modified CTR-07: the interface is in English.
- Added SEL-01-R07: a v1 file is read, its keys and states are mapped to v2, and it is saved as v2 on the next change.
- Added SEL-01-A04: given a v1 file with `"estado": "queda"`, after opening and saving it reads `"state": "keep"` under `files`, and unknown fields are kept.

## Format v2

```json
{ "version": 2, "title": "…", "sources": ["…"], "updated": 0,
  "files": { "OP-XXXX-XXXX": { "state": "keep|out", "note": "…", "tags": ["…"], "path": "…" } },
  "summary": { "kept": [{"id": "…", "path": "…"}], "out": [], "pending": [] } }
```

v1 → v2: `titulo→title`, `fuentes→sources`, `actualizado→updated`, `archivos→files`, `estado→state` (`queda→keep`, `fuera→out`), `nota→note`, `etiquetas→tags`, `ruta→path`; `resumen` is dropped and recomputed as `summary`.

## Data and transition

- IDs are unchanged (CTR-04). The hash cache changes keys; an old cache is ignored and rebuilt (one extra hashing pass).
- Rollback: an Opitar v0.1 cannot read v2 (it rejects `version > 1` without touching the file).
- Spec files are renamed to English names; requirement IDs do not change.

## Closing evidence (2026-11-25)

Environment: macOS 27.2 arm64, cargo 1.99.1, built-in browser. Commit: the CAM-003 closing commit.

- `cargo test --release`: 16 passed (new: `v1_file_is_migrated_to_v2`). `cargo clippy --release`: 0 warnings.
- Real: the sample folder's `opitar.json` written by v0.1 (Spanish keys) opened in v0.2; the UI showed its choices, note and tag; one mark rewrote it as v2 with keys `version, title, sources, updated, files, summary` and states `keep`/`out`.
- UI walked in English: table, viewer, Compare, search `state:keep`. A bug found in the walk (the `state:` filter cut one character too many) was fixed before closing.
- `opitar --help` in English.

### Not run / not checked

- Spanish flag aliases other than `--lista`.
- Everything listed as not checked in CAM-001 and still open in traceability.
