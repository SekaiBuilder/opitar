# Experiencia y estética

## Principios de uso

| ID | Principio |
|---|---|
| UX-01 | Un comando, cero configuración: `opitar <ruta>` y listo. Todo lo demás tiene valor por defecto. |
| UX-02 | Todo con teclado: `j/k` o flechas para moverse, `Espacio` abre el visor, `q` se queda, `x` fuera, `u` sin decidir, `/` busca, `Esc` cierra. |
| UX-03 | Nada se pierde: cada cambio se guarda al momento; al reabrir se recupera. |
| UX-04 | La referencia manda: cada archivo muestra su ID y se copia con un clic. |
| UX-05 | Estados vacío, cargando y error escritos en la interfaz, nunca una pantalla en blanco. |

## Estética (estilo Notion, minimalista)

| ID | Regla |
|---|---|
| EST-01 | Fondo blanco (oscuro `#191919` en modo oscuro), tinta `#37352f`, líneas `rgba(55,53,47,.09)`. Paleta heredada del prototipo. |
| EST-02 | Tipografía del sistema (`ui-sans-serif, -apple-system…`), 14 px; título de página 40 px en negrita; IDs en monoespaciada. |
| EST-03 | Estados como etiquetas planas: verde «Se queda», rojo «Fuera», gris «Sin decidir». |
| EST-04 | Sin sombras pesadas ni bordes gruesos; el visor es un panel lateral («peek») como el de Notion. |
| EST-05 | Iconos de tipo con emoji sobrios (🖼 imagen, 🎵 audio, 🎬 vídeo, 🧊 3D, 📄 documento, 🔤 fuente, ⌨ código, 📦 comprimido, · otro). |
| EST-06 | Funciona en claro y oscuro (`prefers-color-scheme`). |
