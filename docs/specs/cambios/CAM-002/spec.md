# CAM-002 — Grupos de variantes que entienden carpetas y prefijos

Estado: implementado y verificado con material real. Módulo: MOD-01. Recorridos: REC-03. Contratos: CTR-04 (los IDs no cambian).
Base: commit `2449ba3`, 2026-11-25.

## Problema y resultado

- **Problema (observado con las cajas de «Atrapa al gato»):** en `cajas-aprobadas/12-oro-repujado/cerrada.webp` el nombre del archivo es solo la capa. Con IDX-01-R05, las 51 «cerrada» caen en un único grupo `cerrada`, y `caja-oro-repujado-cerrada.png` (otra fuente) cae en otro grupo distinto. La vista Comparar no sirve.
- **Resultado:** las dos formas dan el mismo grupo `oro-repujado-cerrada`, y Comparar muestra una fila por caja y capa con una celda por fuente.

## Cambios a la línea base

- Modificado IDX-01-R05: además de normalizar el nombre, (a) si el mismo nombre normalizado aparece en más de una carpeta de la misma fuente y tipo, el grupo antepone la carpeta normalizada (`12-oro-repujado/cerrada` → `oro-repujado-cerrada`); (b) si en una fuente con al menos 3 archivos el 60 % o más de los grupos empiezan por la misma palabra (`caja-…`), esa palabra se quita en esa fuente.
- Añadido IDX-01-A05: dados `A/12-oro/cerrada.png`, `A/13-hielo/cerrada.png` y `B/caja-oro-cerrada.png`, `B/caja-hielo-cerrada.png`, `B/caja-pino-cerrada.png`, `A/12-oro/cerrada.png` y `B/caja-oro-cerrada.png` comparten grupo, y `A/13-hielo/cerrada.png` está en otro.

## Fuera de alcance

- Juntar las capas de una caja en una sola fila (como hacía el prototipo con `--capas`): queda como decisión abierta DEC-103.

## Datos

- Solo cambia `grupo` (no se guarda en `opitar.json`); IDs y selecciones intactos.

## Evidencia de cierre (2026-11-25)

Entorno: macOS 27.2 arm64, cargo 1.99.1, navegador integrado. Commit: el de cierre de CAM-002.

- `cargo test`: 15 pruebas, 0 fallos (nuevo: `groups_understand_folders_and_shared_prefixes`).
- **Cajas de «Atrapa al gato»** (4 fuentes del prototipo: `cajas-aprobadas`, `revision-hiperrealista`, `estilos-artisticos`, `resultados`): 412 archivos indexados en 0,0 s con caché, 122 duplicados, 199 grupos, 93 con más de una versión, máximo 3 versiones por grupo. Comparar junta `cajas-aprobadas/01-renacimiento/cerrada.webp` con `resultados/caja-renacimiento-cerrada.png`. Antes del cambio, las 51 «cerrada» caían en un solo grupo.
- **Kanji de Katrainer** (`public/kanji`, `.claude/worktrees/kanji-niveles/public/kanji`, `marketing/campaigns/kanji-flashcards`): 2347 archivos en 0,1 s, 206 duplicados (los 103 kanji de `public` son idénticos a los del worktree). La Galería muestra los SVG; el visor reproduce los mp4 de campaña (1080×1920, 0:25).
- La selección se guardó fuera de los proyectos (`--salida` en una carpeta temporal): no se escribió nada en `atrapa-el-gato` ni en `katrainer` (CTR-01).
