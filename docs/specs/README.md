# Desarrollo guiado por especificaciones (SDD) — Opitar

Cada cambio empieza con un problema observable y criterios de aceptación.
Después se documenta el plan, se implementa el alcance acordado y se registra
la evidencia. Un test local no acredita una instalación real ni un despliegue.

Nivel: **estándar** (interfaz, guarda la selección del usuario, se publica como
código abierto). Norma común: `../METODOLOGIA-SDD.md` de `activos/`.

## Índice

- [Investigación previa](investigacion.md)
- [Producto y recorridos](producto.md)
- [Experiencia y estética](experiencia.md)
- [Mapa de módulos](modulos.md)
- [Contratos transversales](contratos.md)
- [Decisiones](decisiones.md)
- [Plantilla de cambio](plantilla-cambio.md)
- Specs de área: [IDX-01 — Índice de archivos](IDX-01-indice.md), [SEL-01 — Selección y guardado](SEL-01-seleccion.md), [UI-01 — Interfaz y visor](UI-01-interfaz.md)
- [Trazabilidad](trazabilidad.md)

Cambio activo: [CAM-001 — Primera versión de Opitar](cambios/CAM-001/spec.md).

## Cómo leer los requisitos

- **Actual:** lo que hace el código del commit indicado, con evidencia.
- **Objetivo:** lo que se exige al cambiar; no dice que hoy se cumpla.
- **Brecha:** diferencia o límite detectado. No se borra para que la spec parezca cumplida.
- **Decisión pendiente:** elección de producto que falta.

## Niveles de evidencia

| Nivel | Qué acredita | Qué no |
|---|---|---|
| Documento | Intención | Que esté implementado |
| Código leído | Comportamiento visible en el código | Que funcione al ejecutarlo |
| Check automatizado | Ese caso, en ese commit y entorno | Pantallas, otros entornos |
| Recorrido real | Lo observado en esa configuración | Otras plataformas |

## Comprobación de deriva

`python3 ~/.claude/skills/sdd/scripts/sdd-deriva.py .`
