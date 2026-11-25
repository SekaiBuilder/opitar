# UI-01 — Interface, viewer and CLI

Modules: MOD-03, MOD-04, MOD-05. Journeys: all. Contracts: CTR-05, CTR-07.

## Requirements

| ID | Requirement | Status |
|---|---|---|
| UI-01-R01 | CLI: `opitar [PATHS]...` (default `.`) with `--output/-o`, `--title`, `--port` (default 0 = any free port), `--no-open`, `--exclude`, `--list` (prints `ID  kind  size  path` and exits) and `--find REF` (prints matches and exits). The Spanish flags of v0.1 remain as hidden aliases. | Current |
| UI-01-R02 | The server listens only on `127.0.0.1`; the URL carries a random per-session token that the whole API and every file require. It only serves files inside a source (checked after resolving the real path), never hidden ones. Files support `Range` (video and audio can seek). | Current |
| UI-01-R03 | **Table view**: columns ID, preview, name (with its folder underneath), kind, size, dimensions, state, tags and note; sticky header; the ID copies with one click. | Current |
| UI-01-R04 | **Reference search** (`/`): free text matches the ID (with or without `OP-` and dashes, also partial), name, path, tags and note; plus filters `kind:`, `ext:`, `state:` (`keep`, `out`, `pending`), `#tag` and `dup` (duplicates only). Several words combine with AND. | Current |
| UI-01-R05 | State tabs (All, Pending, Keep, Out, Duplicates) and kind chips, each with its count. | Current |
| UI-01-R06 | Side **viewer** per kind: image (zoomable, checkerboard for transparency), audio with waveform and video with controls, 3D (`glb`, `gltf`, `obj`, `stl`, `fbx`, `ply`) with orbit, embedded PDF, text/code (first 200 KB), font with sample text, and for everything else its data and the first bytes in hex. | Current |
| UI-01-R07 | **Gallery view** (cards with preview) and **Compare view** (one row per variant group with more than one file, one cell per file; clicking one applies SEL-01-R03). | Current |
| UI-01-R08 | From the viewer: set state, edit note and tags, copy ID, copy path and "Show in Finder" (macOS `open -R`; elsewhere, open the folder). | Current |
| UI-01-R09 | Keyboard per UX-02. | Current |
| UI-01-R10 | Dropping a file on the page opens it in the viewer (locally, not indexed); if its content matches an indexed file, its ID is shown. | Current |
| UI-01-R11 | If the only source is a file, the page opens its viewer straight away. | Current |
| UI-01-R12 | A "Rescan" button re-indexes without restarting and keeps the choices. | Current |
| UI-01-R13 | The page loads nothing from the internet: CSS, JS and three.js are embedded in the binary. | Current |

## Acceptance

| ID | Given / when / then |
|---|---|
| UI-01-A01 | Given `opitar model.glb`, the browser opens with the model orbitable in the viewer. |
| UI-01-A02 | A request without the token or with `..` gets 403/404. |
| UI-01-A03 | Typing part of an ID in the search box leaves only that file. |
| UI-01-A04 | With the keyboard (`j`, `q`, `x`) rows are walked and marked without the mouse, and the JSON reflects each mark. |
