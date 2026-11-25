# IDX-01 — Índice de archivos

Módulo: MOD-01. Recorridos: REC-01..REC-04. Contratos: CTR-01, CTR-04.

## Requisitos

| ID | Requisito | Estado |
|---|---|---|
| IDX-01-R01 | Cada ruta dada es una **fuente**: una carpeta (se recorre entera) o un archivo suelto. Se respetan `.gitignore`, `.ignore` y `.opitarignore`; se saltan ocultos, enlaces simbólicos y siempre `node_modules`, `target`, `.git`, `__pycache__`, `.venv`, `DerivedData`, `.opitar-cache.json` y el propio archivo de salida. | Actual |
| IDX-01-R02 | Cada archivo recibe un **tipo**: imagen, audio, vídeo, 3d, documento, fuente, código, comprimido u otro. Primero por extensión (tabla en `src/kind.rs`); si no la reconoce, por firma de bytes; si no, «código» cuando los primeros 8 KB son UTF-8 sin bytes nulos, y si no «otro». | Actual |
| IDX-01-R03 | Cada archivo recibe un **ID** según DEC-003 a partir del BLAKE3 de su contenido completo. Archivos con el mismo contenido comparten ID y se marcan como **duplicados** con el número de copias. | Actual |
| IDX-01-R04 | Se registran tamaño en bytes, fecha de modificación y, para imágenes, ancho × alto leídos de la cabecera. | Actual |
| IDX-01-R05 | Cada archivo pertenece a un **grupo de variantes**: nombre sin extensión, en minúsculas, sin número inicial (`12-`), sin sufijos de versión al final (`_v2`, `-v03`, `final`, `copia`, `copy`, `(2)`, `@2x`, `old`, `new`, `wip`, `draft`, `bak`), con `-`, `_`, `.` y espacios unificados en `-`, y del mismo tipo. Si el nombre queda vacío se usa el original. Además (CAM-002): si el mismo nombre normalizado aparece en más de una carpeta de la misma fuente y tipo, se antepone la carpeta normalizada (`12-oro-repujado/cerrada` → `oro-repujado-cerrada`); y si en una fuente con al menos 3 archivos el 60 % o más de los grupos empiezan por la misma palabra (`caja-…`), esa palabra se quita en esa fuente. | Actual |
| IDX-01-R06 | `--excluir REGEX` quita los archivos cuya ruta relativa coincide. | Actual |
| IDX-01-R07 | Las huellas se guardan en `.opitar-cache.json` junto a la salida, por ruta + tamaño + fecha; si no cambian, no se vuelve a leer el archivo. Las huellas se calculan en paralelo. | Actual |
| IDX-01-R08 | El orden por defecto es fuente, carpeta y nombre (orden natural: `2` antes que `10`). | Actual |

## Aceptación

| ID | Dado / cuando / entonces |
|---|---|
| IDX-01-A01 | Dado un archivo con contenido `hola\n`, su ID es siempre el mismo (fijado en test). |
| IDX-01-A02 | Dados `logo_v1.png`, `logo-final.png` y `Logo (2).png`, los tres están en el grupo `logo`. |
| IDX-01-A03 | Dada una carpeta con `node_modules/x.png` y un `.gitignore` que ignora `tmp/`, ninguno de los dos aparece. |
| IDX-01-A04 | Dos copias del mismo archivo en rutas distintas tienen el mismo ID y figuran como duplicados ×2. |
| IDX-01-A05 | Dados `A/12-oro/cerrada.png`, `A/13-hielo/cerrada.png` y `B/caja-oro-cerrada.png`, `B/caja-hielo-cerrada.png`, `B/caja-pino-cerrada.png`, los de «oro» comparten grupo y `A/13-hielo/cerrada.png` está en otro. |
