# Investigación previa (2026-11-25)

Qué falla hoy cuando un desarrollador tiene que elegir archivos multimedia, qué
herramientas existen y qué adopta Opitar. Fuentes: foros y páginas de producto
(no son encuestas: muestran quejas repetidas, no su frecuencia).

## Carencias observadas

| # | Carencia | Ejemplo / fuente | Respuesta en Opitar |
|---|---|---|---|
| C1 | **Versiones sin fin** («final_final_v2b») y nadie sabe cuál vale | Hilo de equipo de 10 personas en los foros de Unreal | Agrupar variantes por nombre normalizado y elegir una en la vista Comparar (IDX-01-R05, SEL-01-R03) |
| C2 | **Duplicados** que parecen distintos (copias de seguridad, reimportaciones) | Blender asset browser muestra 4–5 copias; Datasmith crea miles de mallas iguales | ID por contenido: dos archivos idénticos comparten ID y se marcan «duplicado» (IDX-01-R03) |
| C3 | **Nombres ambiguos** y búsqueda por nombre que no encuentra | Guía de nombres de Epic: riesgo de «nombres demasiado parecidos» | Buscador por referencia (ID), nombre, ruta, tipo y etiqueta (UI-01-R04) |
| C4 | **Cada herramienta ve un tipo**: visor de imágenes, reproductor, visor 3D, editor… | Gestores de assets de itch.io/Godot cubren imagen+audio+3D, pero ligados a un motor | Un visor para imagen, audio, vídeo, 3D, PDF, texto/código y fuentes (UI-01-R06) |
| C5 | **La elección se queda en la cabeza o en un chat**; el siguiente (persona o agente) no la ve | Sugerencia en foros de mantener una hoja con nombre/autor/descripción | La selección se escribe al momento en un JSON legible y exportable a CSV (SEL-01-R01) |
| C6 | **Proyectos grandes** lentos de recorrer; carpetas de compilación y dependencias meten ruido | `node_modules`, `target`, copias | Respeta `.gitignore`, ignora ocultos y carpetas de compilación; caché de huellas (IDX-01-R01, R07) |
| C7 | **Gestores pesados o cerrados** (Electron, nube, de pago) | Unity Asset Manager es nube; Eagle es de pago; Allusion solo imágenes | Un binario Rust local, sin cuentas ni red (CTR-05) |
| C8 | **El prototipo propio** solo servía para las cajas del gato | `selector-multimedia/` (Python): reglas `caja-`, capas fijas | Opitar generaliza: cualquier carpeta, cualquier tipo, reglas de variantes genéricas |

## Herramientas parecidas

| Herramienta | Qué tiene | Qué falta para este uso | Se adopta |
|---|---|---|---|
| Global Asset Manager (Godot, abierto) | Escanea carpetas, etiqueta por carpeta, previsualiza 3D/imagen/audio | Solo dentro de Godot | Etiquetas automáticas por carpeta (objetivo, ver brechas) |
| Assets Manager (vailor1, abierto) | GLTF/GLB/FBX/OBJ, WAV/FLAC/OGG/MP3 | Windows/Linux, sin elección ni IDs | Lista de formatos 3D y audio |
| Orlop, Sharp Stock Lite | Etiquetas, búsqueda, «mostrar en el explorador» | Licencia cerrada o no indicada | «Mostrar en Finder» (UI-01-R08) |
| Plugins de duplicados por hash (Unreal) | Duplicados por contenido, no por nombre | Solo Unreal | Huella BLAKE3 del contenido |
| Prototipo `selector-multimedia` | Tabla estilo Notion, columnas por fuente, elección guardada al momento en JSON | Específico de cajas; solo imagen/audio/vídeo | Formato de tabla, estados «se queda / fuera / sin decidir», guardado inmediato |

## Fuentes

- [Global Asset Manager](https://godotengine.org/asset-library/asset/4829) · [fork de kivylius](https://store.godotengine.org/asset/kivylius/asset-manager/)
- [Assets Manager (vailor1)](https://vailor1.itch.io/assets-manager) · [Orlop](https://tirch.itch.io/orlop) · [Sharp Stock Lite](https://sharpknifegames.itch.io/sharp-stock-lite)
- [«Final_Final» en equipos (foros Unreal)](https://forums.unrealengine.com/t/why-do-my-final-final-folders-keep-multiplying-how-do-you-enforce-one-true-asset-version-on-a-10-person-team/2512267)
- [Duplicados en el asset browser de Blender](https://blenderartists.org/t/asset-browser-making-many-duplicates/1464081)
- [Datasmith y mallas repetidas](https://forums.unrealengine.com/t/datasmith-and-cleaning-the-project-from-unnecessary-mesh/2421624)
- [Convenciones de nombres de Epic](https://dev.epicgames.com/documentation/de-de/unreal-engine/recommended-asset-naming-conventions-in-unreal-engine-projects)
- [Bibliotecas grandes (foros Unreal)](https://forums.unrealengine.com/t/how-to-manage-big-assets-libraries/74972)
- [Duplicados en AEM](https://experienceleaguecommunities.adobe.com/adobe-experience-manager-sites-8/resolve-duplicates-unreferenced-assets-in-aem-13428)
- [Unity Asset Manager](https://unity.com/en/products/asset-manager)
