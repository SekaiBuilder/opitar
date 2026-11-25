# SEL-01 — Selection and saving

Module: MOD-02. Journeys: REC-02, REC-03, REC-05. Contracts: CTR-01..CTR-03.

## Requirements

| ID | Requirement | Status |
|---|---|---|
| SEL-01-R01 | The selection lives in a JSON file (`--output`, default `opitar.json` in the first source's folder, or next to the file if the source is a file). Every change is saved at once, atomically (CTR-03). | Current |
| SEL-01-R02 | Per file (key = ID) it stores `state` (`keep`, `out`, or absent = pending), `note` (up to 2000 characters), `tags` (list, each up to 40 characters, no repeats) and the last seen `path`. An entry without state, note or tags is removed. | Current |
| SEL-01-R03 | **Picking in a group** marks the chosen file `keep` and the rest of the group `out`. Picking it again sets the whole group back to pending. | Current |
| SEL-01-R04 | The JSON carries `version: 2`, `title`, `sources`, `updated` (Unix seconds), `files` and a `summary` with `kept`, `out` and `pending` (each a list of `{id, path}`) for the files present in the session. | Current |
| SEL-01-R05 | On open, existing choices are loaded; choices for IDs not in this session are kept untouched (data from other folders is never lost). | Current |
| SEL-01-R06 | CSV export with columns `id,state,kind,path,size,width,height,tags,note`. | Current |
| SEL-01-R07 | A v1 file (Spanish keys: `archivos`, `estado: queda/fuera`, `nota`, `etiquetas`, `ruta`) is read, mapped to v2 and saved as v2 on the next change (CAM-003). | Current |

## Acceptance

| ID | Given / when / then |
|---|---|
| SEL-01-A01 | When a file is marked keep and Opitar restarts, it is still marked and appears in `summary.kept`. |
| SEL-01-A02 | When `logo-final.png` is picked in the 3-file group `logo`, 1 is in `kept` and 2 in `out`. |
| SEL-01-A03 | Given a JSON with a choice for an ID not in the session, after saving another choice that entry is still in `files`. |
| SEL-01-A04 | Given a v1 file with `"estado": "queda"`, after opening and saving it reads `"state": "keep"` under `files`, and unknown fields are kept. |
