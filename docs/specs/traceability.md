# Traceability and evidence

Levels: **auto** = automated check (`cargo test`), **real** = real walk-through in the browser or terminal, **code** = code read only.
Environment for everything below: macOS 27.2 arm64, cargo 1.99.1, 2026-11-25. CAM-001/CAM-002 rows were run on their own commits; CAM-003 rows on the CAM-003 closing commit.

## Recorded runs

| Suite | Commit | Environment | Result |
|---|---|---|---|
| `cargo test --release` | CAM-003 closing commit | macOS 27.2 arm64 | 16 passed, 0 failed |
| `cargo clippy --release` | CAM-003 closing commit | macOS 27.2 arm64 | 0 warnings |

## Per requirement

| Test | Requirement / acceptance | Level | Result |
|---|---|---|---|
| `scan::tests::walk_respects_ignores_and_finds_duplicates`; in the sample, `node_modules/no.png` does not appear | IDX-01-R01, A03 | auto + real | Pass |
| `kind::tests::*` (English kind names since CAM-003) | IDX-01-R02 | auto | Pass |
| `ids::tests::id_is_stable_for_fixed_content` (`hola\n` → `OP-D3CV-TA7N`); sample: 3 copies of `logo` with the same ID `×3` | IDX-01-R03, A01, A04 | auto + real | Pass |
| Sample: `logo_v1.png` 96×64 via the data API | IDX-01-R04 | real | Pass |
| `scan::tests::variant_groups`, `groups_understand_folders_and_shared_prefixes`; Compare groups `logo_v1.png`, `Logo (2).jpg`, `logo-final.png` and the "Atrapa al gato" boxes by box and layer | IDX-01-R05, A02, A05 | auto + real | Pass |
| — | IDX-01-R06 | code | No test |
| A second scan with the cache gives the same IDs (test) | IDX-01-R07 | auto | Pass |
| `scan::tests::natural_order` | IDX-01-R08 | auto | Pass |
| `store::tests::mark_survives_restart`; sample: `q` writes `"state": "keep"` to `opitar.json` | SEL-01-R01..R04, A01 | auto + real | Pass |
| `store::tests::choose_in_group_and_toggle`; click in Compare | SEL-01-R03, A02 | auto + real | Pass |
| `store::tests::foreign_ids_and_unknown_fields_are_kept` | SEL-01-R05, A03, CTR-02 | auto | Pass |
| `store::tests::tags_and_notes_*` (CSV quoting); export route with curl (CAM-001) | SEL-01-R06 | auto + real | Pass |
| `store::tests::v1_file_is_migrated_to_v2`; real: the sample's v0.1 `opitar.json` opened in v0.2, choices, note and tag shown, rewritten as v2 on the next mark | SEL-01-R07, A04 | auto + real | Pass |
| `write_atomic` (temp + rename), used by every save | CTR-03 | code | — |
| `--list`, `--find`, `--help` in English; Spanish aliases (`--lista`) still work | UI-01-R01 | real | Pass |
| curl (CAM-001): no token 403, `/` without token 403, `..` encoded and raw 404, `Range: bytes=0-9` → 206 with `Content-Range` | UI-01-R02, A02, CTR-05 | real | Pass |
| Table screenshot with counts and chips (English since CAM-003) | UI-01-R03, R05, CTR-07 | real | Pass |
| Search "cubo" leaves one row (CAM-001); `state:keep` leaves the 4 kept files (CAM-003) | UI-01-R04, A03 | real | Pass (partial ID only via `--find`) |
| Viewer: obj and gltf (3D with orbit and stats), wav (waveform), ttf (sample and glyphs), mp4 (Katrainer) | UI-01-R06 | real | Pass; PDF, fbx, ply and hex not walked |
| Compare with the boxes; Gallery with 2347 Katrainer kanji | UI-01-R07 | real | Pass |
| State buttons; Finder not tested | UI-01-R08 | real (partial) | — |
| `j`, `k`, `Space`, `q`, `Esc` in the browser | UI-01-R09, A04 | real | Pass |
| Fingerprint API with `cubo.obj` → its ID and index (CAM-001) | UI-01-R10 | real (API) | Desktop drop not tested |
| `opitar solo/tri.gltf` opens the viewer full screen | UI-01-R11, A01 | real | Pass (gltf; glb not tested) |
| — | UI-01-R12 | code | No test |
| Page and import map point to the embedded `/v/…` | UI-01-R13 | code + real | three.js served 200 |
