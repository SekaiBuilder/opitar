# Opitar

**Selector de archivos para desarrolladores.** Le das una carpeta (o un archivo) de cualquier proyecto y te lo pone todo en una tabla minimalista, estilo Notion: cada archivo con su **ID por contenido**, vista previa y un **visor universal** (imagen, audio con forma de onda, vídeo, **3D**, PDF, texto/código y fuentes). Eliges qué se queda y qué no, con teclado, y la decisión se guarda al momento en un JSON que cualquier persona o agente puede leer.

Un binario en Rust, local, sin cuentas ni red.

## Por qué

Elegir assets en un proyecto grande se rompe siempre por lo mismo: versiones sin fin (`logo_final_final_v2`), duplicados que parecen distintos, nombres ambiguos, una app distinta para cada tipo de archivo y la elección que se pierde en un chat. Opitar ataca cada uno:

| Problema | Opitar |
|---|---|
| ¿Cuál es la versión buena? | Vista **Comparar**: agrupa `logo_v1`, `logo-final`, `Logo (2)`; tocas la que se queda y las demás quedan fuera |
| Duplicados | El ID sale del contenido (BLAKE3): copias idénticas comparten ID y se marcan `×N` |
| «¿Cuál era ese archivo?» | **Buscador por referencia**: `OP-7K3M-Q9XA`, parte del ID, nombre, ruta, `tipo:audio`, `ext:png`, `estado:pendiente`, `#etiqueta`, `dup` |
| Un visor por formato | Un visor para todo; 3D con `glb`, `gltf`, `obj`, `stl`, `fbx`, `ply` |
| La elección se pierde | `opitar.json` (y CSV) con lo que se queda, lo que no y lo pendiente |
| Proyectos enormes | Respeta `.gitignore`, ignora `node_modules`, `target`…; caché de huellas |

## Instalar

```bash
cargo install --git https://github.com/SekaiBuilder/opitar
```

O desde el código: `cargo build --release` → `target/release/opitar`.

## Usar

```bash
opitar assets/                 # tabla con todo lo de assets/
opitar ronda1/ ronda2/         # varias fuentes: comparar versiones
opitar modelo.glb              # abre ese archivo directamente en el visor
opitar . --lista               # tabla en la terminal: ID, tipo, tamaño, ruta
opitar . --buscar OP-7K3M      # buscar por referencia desde la terminal
```

Opciones: `--salida archivo.json`, `--titulo`, `--puerto`, `--no-abrir`, `--excluir REGEX`. Archivos a ignorar: `.opitarignore` (misma sintaxis que `.gitignore`).

### Teclado

`j`/`k` moverse · `Espacio` visor · `q` se queda · `x` fuera · `u` sin decidir · `p` escuchar · `c` copiar ID · `f` pantalla completa · `/` buscar · `Esc` cerrar · `?` ayuda.

Suelta un archivo del escritorio en la página para abrirlo; si ya está en el índice, te dice su ID.

## El archivo de selección

```json
{
  "version": 1,
  "archivos": {
    "OP-398C-VKB1": { "estado": "queda", "nota": "la buena", "etiquetas": ["ui"], "ruta": "/…/logo_v1.png" }
  },
  "resumen": { "se_quedan": [{ "id": "OP-398C-VKB1", "ruta": "/…/logo_v1.png" }], "fuera": [], "sin_decidir": [] }
}
```

Opitar **nunca modifica tus archivos**: solo escribe este JSON y su caché `.opitar-cache.json`. El servidor escucha solo en `127.0.0.1` con un token por sesión.

## Specs

Proyecto con desarrollo guiado por especificaciones: [`docs/specs/`](docs/specs/README.md) (investigación previa, producto, contratos, decisiones y requisitos por área con su evidencia).

## Licencia y autoría

Apache License 2.0 — ver [LICENSE](LICENSE). Puedes usarlo, modificarlo y redistribuirlo, también comercialmente, **siempre que conserves el archivo [NOTICE](NOTICE) y nombres al autor original, SekaiBuilder**.

Incluye three.js (MIT) para el visor 3D.
