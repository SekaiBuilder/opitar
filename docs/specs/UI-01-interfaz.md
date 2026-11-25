# UI-01 — Interfaz, visor y CLI

Módulos: MOD-03, MOD-04, MOD-05. Recorridos: todos. Contratos: CTR-05, CTR-07.

## Requisitos

| ID | Requisito | Estado |
|---|---|---|
| UI-01-R01 | CLI: `opitar [RUTAS]...` (por defecto `.`) con `--salida`, `--titulo`, `--puerto` (por defecto 0 = uno libre), `--no-abrir`, `--excluir`, `--lista` (imprime la tabla `ID  tipo  tamaño  ruta` y sale) y `--buscar REF` (imprime las coincidencias y sale). | Actual |
| UI-01-R02 | El servidor escucha solo en `127.0.0.1`; la URL lleva un token aleatorio por sesión y toda la API y los archivos lo exigen. Solo sirve archivos que están dentro de una fuente (comprobado tras resolver la ruta real), nunca ocultos. Los archivos admiten `Range` (vídeo y audio con salto). | Actual |
| UI-01-R03 | **Vista Tabla**: columnas ID, vista previa, nombre (con su carpeta debajo), tipo, tamaño, medidas, estado, etiquetas y nota; cabecera fija; el ID se copia con un clic. | Actual |
| UI-01-R04 | **Buscador por referencia** (`/`): texto libre que busca en ID (con o sin `OP-` y guiones, también parcial), nombre, ruta y etiquetas; y filtros `tipo:`, `ext:`, `estado:` (`queda`, `fuera`, `pendiente`), `#etiqueta` y `dup` (solo duplicados). Varias palabras se combinan con Y. | Actual |
| UI-01-R05 | Pestañas de estado (Todos, Sin decidir, Se quedan, Fuera, Duplicados) y chips de tipo, cada uno con su contador. | Actual |
| UI-01-R06 | **Visor** lateral según tipo: imagen (ampliable, fondo de cuadros para transparencias), audio y vídeo con controles, 3D (`glb`, `gltf`, `obj`, `stl`, `fbx`, `ply`) con órbita, PDF incrustado, texto/código (primeros 200 KB), fuente con muestra de texto, y para lo demás sus datos y los primeros bytes en hexadecimal. | Actual |
| UI-01-R07 | **Vista Galería** (tarjetas con vista previa) y **vista Comparar** (una fila por grupo de variantes con más de un archivo, una celda por archivo; tocar una aplica SEL-01-R03). | Actual |
| UI-01-R08 | Desde el visor: marcar estado, editar nota y etiquetas, copiar ID, copiar ruta y «Mostrar en Finder» (macOS `open -R`; en otros sistemas, abrir la carpeta). | Actual |
| UI-01-R09 | Teclado según UX-02. | Actual |
| UI-01-R10 | Arrastrar y soltar un archivo en la página lo abre en el visor (local, sin indexarlo); si su contenido coincide con un archivo indexado, se señala su ID. | Actual |
| UI-01-R11 | Si la única fuente es un archivo, la página abre directamente su visor. | Actual |
| UI-01-R12 | Botón «Reescanear» que vuelve a indexar sin reiniciar y conserva las elecciones. | Actual |
| UI-01-R13 | La página no carga nada de internet: CSS, JS y three.js van embebidos en el binario. | Actual |

## Aceptación

| ID | Dado / cuando / entonces |
|---|---|
| UI-01-A01 | Dado `opitar modelo.glb`, se abre el navegador con el modelo girable en el visor. |
| UI-01-A02 | Una petición sin token o con `..` recibe 403/404. |
| UI-01-A03 | Al escribir parte de un ID en el buscador queda solo ese archivo. |
| UI-01-A04 | Con teclado (`j`, `q`, `x`) se recorren y marcan filas sin ratón, y el JSON refleja cada marca. |
