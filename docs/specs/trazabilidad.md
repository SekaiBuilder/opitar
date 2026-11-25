# Trazabilidad y evidencia

Niveles: **auto** = check automatizado (`cargo test`), **real** = recorrido real en navegador o terminal, **código** = solo código leído.
Entorno de todo lo de abajo: macOS 27.2 arm64, cargo 1.99.1, 2026-11-25, commit de cierre de CAM-001.

| Prueba | Requisito / aceptación | Nivel | Resultado |
|---|---|---|---|
| `scan::tests::walk_respects_ignores_and_finds_duplicates`; en la muestra, `node_modules/no.png` no aparece | IDX-01-R01, A03 | auto + real | Pasa |
| `kind::tests::*` | IDX-01-R02 | auto | Pasa |
| `ids::tests::id_is_stable_for_fixed_content` (`hola\n` → `OP-D3CV-TA7N`); muestra: 3 copias de `logo` con el mismo ID `×3` | IDX-01-R03, A01, A04 | auto + real | Pasa |
| Muestra: `logo_v1.png` 96×64 vía `/api/datos` | IDX-01-R04 | real | Pasa |
| `scan::tests::variant_groups`; vista Comparar agrupa `logo_v1.png`, `Logo (2).jpg`, `logo-final.png` | IDX-01-R05, A02 | auto + real | Pasa |
| — | IDX-01-R06 | código | Sin prueba |
| Segundo escaneo con caché da los mismos IDs (test) | IDX-01-R07 | auto | Pasa |
| `scan::tests::natural_order` | IDX-01-R08 | auto | Pasa |
| `store::tests::mark_survives_restart`; muestra: `q` en `glass.wav` aparece en `opitar.json` | SEL-01-R01..R04, A01 | auto + real | Pasa |
| `store::tests::choose_in_group_and_toggle`; `/api/grupo` con curl y clic en Comparar | SEL-01-R03, A02 | auto + real | Pasa |
| `store::tests::foreign_ids_and_unknown_fields_are_kept` | SEL-01-R05, A03, CTR-02 | auto | Pasa |
| `store::tests::tags_and_notes_*` (comillas CSV); `/api/exportar.csv` con curl | SEL-01-R06 | auto + real | Pasa |
| `write_atomic` (temporal + rename), usado en todos los guardados | CTR-03 | código | — |
| `--lista` y `--buscar logo` en la muestra | UI-01-R01 | real | Pasa |
| curl: sin token 403, `/` sin token 403, `..` codificado y sin codificar 404, `Range: bytes=0-9` → 206 con `Content-Range` | UI-01-R02, A02, CTR-05 | real | Pasa |
| Captura de la tabla con contadores y chips | UI-01-R03, R05 | real | Pasa (cabecera fija corregida durante la prueba) |
| Buscar «cubo» deja una fila; Enter abre el visor | UI-01-R04, A03 | real | Pasa (ID parcial: solo con `--buscar` en terminal) |
| Visor: obj y gltf (3D con órbita y estadísticas), wav (forma de onda), ttf (muestra y glifos) | UI-01-R06 | real | Pasa; vídeo, PDF, fbx, ply, hex sin recorrer |
| Comparar recorrido; Galería no | UI-01-R07 | real (parcial) | Comparar pasa |
| Botones de estado; Finder no probado | UI-01-R08 | real (parcial) | — |
| `j`, `k`, `Espacio`, `q`, `Esc` en el navegador | UI-01-R09, A04 | real | Pasa |
| `/api/huella` con `cubo.obj` → su ID e índice | UI-01-R10 | real (API) | Soltar desde el escritorio sin probar |
| `opitar solo/tri.gltf` abre el visor a pantalla completa | UI-01-R11, A01 | real | Pasa (con gltf; glb no probado) |
| — | UI-01-R12 | código | Sin prueba |
| Página e importmap apuntan a `/v/…` embebido | UI-01-R13 | código + real | three.js servido 200 |
