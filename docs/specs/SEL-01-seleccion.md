# SEL-01 — Selección y guardado

Módulo: MOD-02. Recorridos: REC-02, REC-03, REC-05. Contratos: CTR-01..CTR-03.

## Requisitos

| ID | Requisito | Estado |
|---|---|---|
| SEL-01-R01 | La selección vive en un JSON (`--salida`, por defecto `opitar.json` en la carpeta de la primera fuente, o junto al archivo si la fuente es un archivo). Cada cambio se guarda al momento, de forma atómica (CTR-03). | Objetivo |
| SEL-01-R02 | Por archivo (clave = ID) se guarda `estado` (`queda`, `fuera` o ausente = sin decidir), `nota` (hasta 2000 caracteres), `etiquetas` (lista, cada una hasta 40 caracteres, sin repetir) y la última `ruta` vista. Una entrada sin estado, nota ni etiquetas se borra. | Objetivo |
| SEL-01-R03 | **Elegir en un grupo** marca el archivo elegido como `queda` y los demás del grupo como `fuera`. Volver a elegirlo deja todo el grupo sin decidir. | Objetivo |
| SEL-01-R04 | El JSON lleva `version: 1`, `titulo`, `fuentes`, `actualizado` (segundos Unix), `archivos` y un `resumen` con `se_quedan`, `fuera` y `sin_decidir` (cada uno lista de `{id, ruta}`) de los archivos presentes en la sesión. | Objetivo |
| SEL-01-R05 | Al abrir, se cargan las elecciones existentes; las de IDs que no están en esta sesión se conservan sin tocar (no se pierden datos de otras carpetas). | Objetivo |
| SEL-01-R06 | Exportar CSV con columnas `id,estado,tipo,ruta,tamano,ancho,alto,etiquetas,nota`. | Objetivo |

## Aceptación

| ID | Dado / cuando / entonces |
|---|---|
| SEL-01-A01 | Cuando se marca un archivo «queda» y se reinicia Opitar, sigue marcado y aparece en `resumen.se_quedan`. |
| SEL-01-A02 | Cuando se elige `logo-final.png` en el grupo `logo` de 3 archivos, queda 1 en `se_quedan` y 2 en `fuera`. |
| SEL-01-A03 | Dado un JSON con una elección de un ID que no está en la sesión, tras guardar otra elección esa entrada sigue en `archivos`. |
