# Product and journeys

**Opitar** is a file picker for developers. Give it one or more folders (or single files) from any project and it lays them out in a minimal table, each with its own ID, a preview and a viewer that opens image, audio, video, 3D, PDF, text and fonts. You mark what stays and what goes, with notes and tags, and the decision is written to a JSON file that any person or agent can read.

Requested by the owner on 2026-11-25 as a generalisation of the `selector-multimedia` prototype (boxes for "Atrapa al gato"), to be used in any kind of project.

## Surfaces

| Surface | What it is |
|---|---|
| `opitar` binary (Rust) | CLI that indexes, serves the UI on `127.0.0.1` and opens the browser; also lists and searches in the terminal |
| Local web UI | Single page embedded in the binary: table, gallery, compare, side viewer |
| `opitar.json` | Selection file (format in SEL-01) |

## Persona

A developer (the owner, or an agent working with them) with a folder of project assets: sprites, sounds, models, videos, documents.

## Journeys

| ID | Journey |
|---|---|
| REC-01 | **Open and look**: `opitar file.glb` → the browser opens with that file already in the viewer. |
| REC-02 | **Pick in a big folder**: `opitar assets/` → table with everything; filter by kind or search, mark keep / out with the keyboard, close; `opitar.json` holds the choice. |
| REC-03 | **Compare versions**: `opitar round1/ round2/` → Compare puts "logo_v1", "logo-final", "logo (2)" on one row; click the one that stays and the rest are marked out. |
| REC-04 | **Find by reference**: someone says "use OP-7K3M-Q9XA" → paste it in the search box (or `opitar --find OP-7K3M`) and the file shows up, even if it was moved or renamed. |
| REC-05 | **Hand over the decision**: another person or agent reads `opitar.json` (or the exported CSV) and knows which files to use. |

## Out of scope (v1)

- Editing, converting or renaming files (Opitar only reads; it only writes the selection and its cache).
- Cloud, accounts, team sync.
- Thumbnails generated on disk (the browser lazily loads the original).
- Proprietary 3D formats (`.blend`, `.max`, `.ma`): listed, no 3D view.
