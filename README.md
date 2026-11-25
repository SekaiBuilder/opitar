# Opitar

**A file picker for developers.** Point it at any folder (or file) in any project and it lays everything out in a minimal, Notion-style table: every file with a **content ID**, a preview and a **universal viewer** (image, audio with waveform, video, **3D**, PDF, text/code and fonts). Mark what stays and what goes from the keyboard, and the decision is saved at once to a JSON file any person or agent can read.

One local Rust binary. No accounts, no network.

## Why

Picking assets in a large project always breaks the same way: endless versions (`logo_final_final_v2`), duplicates that look different, ambiguous names, a different app for every file type, and a choice that gets lost in a chat. Opitar tackles each one:

| Problem | Opitar |
|---|---|
| Which version is the right one? | **Compare** view groups `logo_v1`, `logo-final`, `Logo (2)` (also `12-oro/cerrada.png` with `caja-oro-cerrada.png`); click the one that stays and the rest are marked out |
| Duplicates | The ID comes from the content (BLAKE3): identical copies share an ID and are flagged `×N` |
| "Which file was that?" | **Reference search**: `OP-7K3M-Q9XA`, part of an ID, name, path, `kind:audio`, `ext:png`, `state:pending`, `#tag`, `dup` |
| One viewer per format | One viewer for everything; 3D for `glb`, `gltf`, `obj`, `stl`, `fbx`, `ply` |
| The choice gets lost | `opitar.json` (and CSV) with what stays, what goes and what's pending |
| Huge projects | Honours `.gitignore`, skips `node_modules`, `target`…; hash cache |

## Install

```bash
cargo install --git https://github.com/SekaiBuilder/opitar
```

Or from source: `cargo build --release` → `target/release/opitar`.

## Use

```bash
opitar assets/                 # table with everything under assets/
opitar round1/ round2/         # several sources: compare versions
opitar model.glb               # open that file straight in the viewer
opitar . --list                # table in the terminal: ID, kind, size, path
opitar . --find OP-7K3M        # find by reference from the terminal
```

Options: `--output file.json`, `--title`, `--port`, `--no-open`, `--exclude REGEX`. Files to ignore: `.opitarignore` (same syntax as `.gitignore`).

### Keyboard

`j`/`k` move · `Space` viewer · `q` keep · `x` out · `u` pending · `p` listen · `c` copy ID · `f` full screen · `/` search · `Esc` close · `?` help.

Drop a file from your desktop onto the page to open it; if it is already indexed, Opitar tells you its ID.

## The selection file

```json
{
  "version": 2,
  "files": {
    "OP-398C-VKB1": { "state": "keep", "note": "the good one", "tags": ["ui"], "path": "/…/logo_v1.png" }
  },
  "summary": { "kept": [{ "id": "OP-398C-VKB1", "path": "/…/logo_v1.png" }], "out": [], "pending": [] }
}
```

Files written by Opitar 0.1 (Spanish keys, `version: 1`) are read and upgraded automatically.

Opitar **never modifies your files**: it only writes this JSON and its cache `.opitar-cache.json`. The server listens only on `127.0.0.1` with a per-session token.

## Specs

Built with spec-driven development: [`docs/specs/`](docs/specs/README.md) (prior research, product, contracts, decisions and per-area requirements with their evidence).

## Licence and attribution

Apache License 2.0 — see [LICENSE](LICENSE). You may use, modify and redistribute it, commercially too, **as long as you keep the [NOTICE](NOTICE) file and credit the original author, SekaiBuilder**.

Includes three.js (MIT) for the 3D viewer.
