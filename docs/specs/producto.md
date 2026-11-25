# Producto y recorridos

**Opitar** es un selector de archivos para desarrolladores: le das una o varias
carpetas (o archivos sueltos) de cualquier proyecto y te los pone en una tabla
minimalista, cada uno con un ID propio, una vista previa y un visor que abre
imagen, audio, vídeo, 3D, PDF, texto y fuentes. Eliges qué se queda y qué
fuera, con notas y etiquetas, y la decisión queda escrita en un JSON que
cualquier persona o agente puede leer.

Pedido por el titular el 2026-11-25 como generalización del prototipo
`selector-multimedia` (cajas de «Atrapa al gato»), para usarlo en todo tipo de
proyecto.

## Superficies

| Superficie | Qué es |
|---|---|
| Binario `opitar` (Rust) | CLI que indexa, sirve la interfaz en `127.0.0.1` y abre el navegador; también lista y busca por terminal |
| Interfaz web local | Página única embebida en el binario: tabla, galería, comparar, visor lateral |
| `opitar.json` | Archivo de selección (formato en SEL-01) |

## Persona

Desarrollador (el titular, o un agente que trabaja con él) con una carpeta de
assets de un proyecto: sprites, sonidos, modelos, vídeos, documentos.

## Recorridos

| ID | Recorrido |
|---|---|
| REC-01 | **Abrir y ver**: `opitar archivo.glb` → se abre el navegador con ese archivo ya en el visor. |
| REC-02 | **Elegir en una carpeta grande**: `opitar assets/` → tabla con todo, filtra por tipo o busca, marca «se queda» / «fuera» con teclado, cierra; `opitar.json` tiene la elección. |
| REC-03 | **Comparar versiones**: `opitar ronda1/ ronda2/` → vista Comparar agrupa «logo_v1», «logo-final», «logo (2)» en una fila; tocas la que se queda y las demás quedan fuera. |
| REC-04 | **Buscar por referencia**: alguien dice «usa OP-7K3M-Q9XA» → se pega en el buscador (o `opitar --buscar OP-7K3M`) y aparece el archivo, aunque se haya movido o renombrado. |
| REC-05 | **Pasar la decisión**: otra persona o agente lee `opitar.json` (o el CSV exportado) y sabe qué archivos usar. |

## Fuera de alcance (v1)

- Editar, convertir o renombrar archivos (Opitar solo lee; lo único que escribe es la selección y su caché).
- Nube, cuentas, sincronización entre equipos.
- Miniaturas pregeneradas en disco (el navegador carga el original con carga diferida).
- Formatos 3D propietarios (`.blend`, `.max`, `.ma`): se listan, sin vista 3D.
