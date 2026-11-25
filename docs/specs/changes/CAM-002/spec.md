# CAM-002 — Variant groups that understand folders and prefixes

Status: implemented and verified with real material. Module: MOD-01. Journeys: REC-03. Contracts: CTR-04 (IDs do not change).
Base: commit `2449ba3`, 2026-11-25.

## Problem and result

- **Problem (seen with the "Atrapa al gato" boxes):** in `cajas-aprobadas/12-oro-repujado/cerrada.webp` the file name is only the layer. Under IDX-01-R05, all 51 "cerrada" files fell into a single `cerrada` group, and `caja-oro-repujado-cerrada.png` (another source) into a different one. The Compare view was useless.
- **Result:** both forms give the same group `oro-repujado-cerrada`, and Compare shows one row per box and layer with one cell per source.

## Baseline changes

- Modified IDX-01-R05: besides normalising the name, (a) if the same normalised name appears in more than one folder of the same source and kind, the group prepends the normalised folder (`12-oro-repujado/cerrada` → `oro-repujado-cerrada`); (b) if in a source with at least 3 files 60 % or more of the groups start with the same word (`caja-…`), that word is dropped for that source.
- Added IDX-01-A05.

## Out of scope

- Merging the layers of one box into a single row (as the prototype did with `--capas`): open decision DEC-103.

## Data

- Only `group` changes (it is not stored in `opitar.json`); IDs and selections untouched.

## Closing evidence (2026-11-25)

Environment: macOS 27.2 arm64, cargo 1.99.1, built-in browser. Commit `76e6022`.

- `cargo test`: 15 tests, 0 failures (new: `groups_understand_folders_and_shared_prefixes`).
- **"Atrapa al gato" boxes** (the prototype's 4 sources: `cajas-aprobadas`, `revision-hiperrealista`, `estilos-artisticos`, `resultados`): 412 files indexed in 0.0 s with cache, 122 duplicates, 199 groups, 93 with more than one version, at most 3 versions per group. Compare pairs `cajas-aprobadas/01-renacimiento/cerrada.webp` with `resultados/caja-renacimiento-cerrada.png`. Before the change, the 51 "cerrada" files fell into one group.
- **Katrainer kanji** (`public/kanji`, `.claude/worktrees/kanji-niveles/public/kanji`, `marketing/campaigns/kanji-flashcards`): 2347 files in 0.1 s, 206 duplicates (the 103 kanji in `public` are identical to the worktree's). The Gallery shows the SVGs; the viewer plays the campaign mp4s (1080×1920, 0:25).
- The selection was saved outside both projects (`--output` in a temp folder): nothing was written to `atrapa-el-gato` or `katrainer` (CTR-01).
