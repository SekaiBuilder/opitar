# CAM-001 — Primera versión de Opitar

Estado: listo para implementar.
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
- Contratos no aplicables: CTR-06 y CTR-08 de la plantilla (sin cálculos compartidos ni publicación en tiendas).
