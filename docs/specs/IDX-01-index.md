# IDX-01 — File index

Module: MOD-01. Journeys: REC-01..REC-04. Contracts: CTR-01, CTR-04.

## Requirements

| ID | Requirement | Status |
|---|---|---|
| IDX-01-R01 | Each given path is a **source**: a folder (walked fully) or a single file. `.gitignore`, `.ignore` and `.opitarignore` are honoured; hidden files, symlinks and always `node_modules`, `target`, `.git`, `__pycache__`, `.venv`, `DerivedData`, `.opitar-cache.json` and the output file itself are skipped. | Current |
| IDX-01-R02 | Each file gets a **kind**: `image`, `audio`, `video`, `3d`, `document`, `font`, `code`, `archive` or `other`. First by extension (table in `src/kind.rs`); if unknown, by byte signature; otherwise `code` when the first 8 KB are UTF-8 without NUL bytes, else `other`. | Current |
| IDX-01-R03 | Each file gets an **ID** per DEC-003 from the BLAKE3 of its full content. Files with the same content share the ID and are flagged as **duplicates** with the number of copies. | Current |
| IDX-01-R04 | Size in bytes, modification time and, for images, width × height read from the header are recorded. | Current |
| IDX-01-R05 | Each file belongs to a **variant group**: name without extension, lowercased, without a leading number (`12-`), without trailing version suffixes (`_v2`, `-v03`, `final`, `copia`, `copy`, `(2)`, `@2x`, `old`, `new`, `wip`, `draft`, `bak`), with `-`, `_`, `.` and spaces unified to `-`, and of the same kind. If the name ends up empty, the original is used. Also (CAM-002): if the same normalised name appears in more than one folder of the same source and kind, the normalised folder is prepended (`12-oro-repujado/cerrada` → `oro-repujado-cerrada`); and if in a source with at least 3 files 60 % or more of the groups start with the same word (`caja-…`), that word is dropped for that source. | Current |
| IDX-01-R06 | `--exclude REGEX` drops files whose relative path matches. | Current |
| IDX-01-R07 | Hashes are kept in `.opitar-cache.json` next to the output, keyed by path + size + mtime; if those don't change the file is not read again. Hashes are computed in parallel. | Current |
| IDX-01-R08 | Default order is source, folder and name (natural order: `2` before `10`). | Current |

## Acceptance

| ID | Given / when / then |
|---|---|
| IDX-01-A01 | Given a file with content `hola\n`, its ID is always the same (pinned in a test). |
| IDX-01-A02 | Given `logo_v1.png`, `logo-final.png` and `Logo (2).png`, all three are in group `logo`. |
| IDX-01-A03 | Given a folder with `node_modules/x.png` and a `.gitignore` ignoring `tmp/`, neither shows up. |
| IDX-01-A04 | Two copies of the same file at different paths have the same ID and show as duplicates ×2. |
| IDX-01-A05 | Given `A/12-oro/cerrada.png`, `A/13-hielo/cerrada.png` and `B/caja-oro-cerrada.png`, `B/caja-hielo-cerrada.png`, `B/caja-pino-cerrada.png`, the "oro" files share a group and `A/13-hielo/cerrada.png` is in another. |
