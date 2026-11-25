# CAM-001 — First version of Opitar

Status: implemented (see evidence; items without a real walk-through below).
Modules: MOD-01..MOD-05. Journeys: REC-01..REC-05. Contracts: CTR-01..CTR-05, CTR-07.
Base: new project, 2026-11-25. Platform tested: macOS (Apple Silicon).

## Problem and result

- **Persona and task:** a developer who must pick among many project files (sprites, sounds, models, videos).
- **Current problem:** the `selector-multimedia` prototype only understood the cat boxes (`caja-` prefix, fixed layers, image/audio/video only); anything else meant opening each file in a different app, and the choice got lost. See gaps C1–C8 in [research](../../research.md).
- **Result:** `opitar <folder>` shows everything in a table, with a content ID, reference search, a universal viewer and the choice saved to `opitar.json`.

## Experience and look

- Open tool (Apache-2.0, DEC-005); no cost or commercial promise.
- UX-01..UX-05 and EST-01..EST-06 apply in full.

## Scope

- All of IDX-01, SEL-01 and UI-01 goes from target to current.
- Out: the "Out of scope" list in [product](../../product.md); DEC-101 and DEC-102.

## Acceptance criteria

Those of each area: IDX-01-A01..A04, SEL-01-A01..A03, UI-01-A01..A04.

## Data and transition

- New format `opitar.json` v1. It does not read the prototype's format (`filas` by key): the prototype's keys are names, not IDs.
- No contracts for shared calculations or store publishing apply.

## Closing evidence (2026-11-25)

Environment: macOS 27.2 arm64, cargo 1.99.1, the app's built-in browser (Chromium). Commit: the CAM-001 closing commit (`git log --grep CAM-001`). Detail per requirement in [traceability](../../traceability.md).

- `cargo test --release`: 14 tests, 0 failures. `cargo clippy --release`: no warnings.
- Real walk-through with a sample folder (2 sources, 13 files: png, jpg, wav, aiff, ttf, stl, obj, md, rs, pdf, bin, plus `node_modules/`): see traceability.

### Not run / not checked

- (Video and Gallery: checked later, in CAM-002.) PDF seen on screen, "Show in Finder", drag and drop from the desktop (only `/api/huella` was tested with curl), CSV export from the button (only the route with curl), `fbx` and `ply`.
- Performance with a large real project (DEC-102).
- Linux and Windows: compile by design (`cfg`), not tested.
