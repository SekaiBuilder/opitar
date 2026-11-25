# CAM-001 — Primera versión de Opitar

Estado: implementado (ver evidencia; puntos sin recorrido real abajo).
Módulos: MOD-01..MOD-05. Recorridos: REC-01..REC-05. Contratos: CTR-01..CTR-05, CTR-07.
Base: proyecto nuevo, 2026-11-25. Plataforma probada: macOS (Apple Silicon).

## Problema y resultado

- **Persona y tarea:** desarrollador que debe elegir entre muchos archivos de un proyecto (sprites, sonidos, modelos, vídeos).
- **Problema actual:** el prototipo `selector-multimedia` solo entiende las cajas del gato (prefijo `caja-`, capas fijas, solo imagen/audio/vídeo); fuera de eso, se abre cada archivo en una app distinta y la elección se pierde. Ver carencias C1–C8 en [investigación](../../investigacion.md).
- **Resultado:** `opitar <carpeta>` muestra todo en tabla, con ID por contenido, buscador por referencia, visor universal y la elección guardada en `opitar.json`.

## Negocio, experiencia y estética

- Herramienta abierta (Apache-2.0, DEC-005); sin coste ni promesa comercial.
- UX-01..UX-05 y EST-01..EST-06 aplican completos.

## Alcance

- Todo IDX-01, SEL-01 y UI-01 pasa de objetivo a actual.
- Fuera: lo listado en «Fuera de alcance» de [producto](../../producto.md); DEC-101 y DEC-102.

## Criterios de aceptación

Los de cada área: IDX-01-A01..A04, SEL-01-A01..A03, UI-01-A01..A04.

## Datos y transición

- Formato nuevo `opitar.json` v1 (SEL-01-R04). No lee el formato del prototipo (`filas` por clave): las claves del prototipo son nombres, no IDs.
- No aplican contratos de cálculos compartidos ni de publicación en tiendas.

## Evidencia de cierre (2026-11-25)

Entorno: macOS 27.2 arm64, cargo 1.99.1, navegador integrado de la app (Chromium). Commit: el de cierre de CAM-001 (`git log --grep CAM-001`). Detalle por requisito en [trazabilidad](../../trazabilidad.md).

- `cargo test --release`: 14 pruebas, 0 fallos. `cargo clippy --release`: sin avisos.
- Recorrido real con una carpeta de muestra (2 fuentes, 13 archivos: png, jpg, wav, aiff, ttf, stl, obj, md, rs, pdf, bin, más `node_modules/`): ver trazabilidad.

### No ejecutado / no comprobado

- (Vídeo y Galería: comprobados después, en CAM-002.) PDF visto en pantalla, «Mostrar en Finder», arrastrar y soltar desde el escritorio (solo se probó `/api/huella` con curl), exportar CSV desde el botón (solo la ruta con curl), `fbx` y `ply`.
- Rendimiento con un proyecto grande real (DEC-102).
- Linux y Windows: compila por diseño (`cfg`), no probado.
