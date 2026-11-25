# Cross-cutting contracts

What no change may break.

| ID | Contract | Evidence required when touched |
|---|---|---|
| CTR-01 | **Read-only on the user's files.** Opitar never modifies, moves or deletes an indexed file. It only writes `opitar.json` (or `--output`) and its cache `.opitar-cache.json` next to it. | Review of every disk write in the diff |
| CTR-02 | **Stable selection format.** `opitar.json` carries `version`; new fields are optional; a new version reads the previous version's files without losing choices. | Fixture of the previous version loaded and saved |
| CTR-03 | **Atomic save.** Written to a temp file and renamed; a crash never leaves half a JSON. | Save test |
| CTR-04 | **Stable IDs.** A file's ID depends only on its content: same content → same ID, on any path, machine or Opitar version. Changing the algorithm needs a new format version and a migration. | Test with fixed content and its expected ID |
| CTR-05 | **Nothing leaves the computer.** The server listens only on `127.0.0.1`, requires a random per-session token, serves nothing outside the given paths, and the page loads nothing from the internet. | Test of a path outside the root and of a missing token |
| CTR-07 | **Experience.** UI in English; full keyboard; empty/loading/error states. | Browser walk-through |
