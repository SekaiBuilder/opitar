# Decisiones

## Registradas

| ID | Decisión | Origen | Alternativas descartadas |
|---|---|---|---|
| DEC-001 | Lenguaje Rust. | Titular, 2026-11-25 | — |
| DEC-002 | Un binario que sirve una interfaz web local embebida (no GUI nativa). El navegador ya reproduce imagen, audio, vídeo, PDF y fuentes, y WebGL cubre 3D; el estilo Notion se consigue con CSS. | Propuesta, por cobertura de formatos con poco código | egui (sin vídeo ni PDF nativos), Tauri (más pesado de compilar y distribuir; queda como opción, DEC-101) |
| DEC-003 | ID = `OP-` + 8 caracteres Crockford base32 de los primeros 40 bits del BLAKE3 del contenido, en dos bloques (`OP-7K3M-Q9XA`). Por contenido para que sobreviva a renombrar y mover (C1, C2). 40 bits: colisión improbable por debajo de ~1 millón de archivos; si dos contenidos distintos chocan, el segundo se alarga a 12 caracteres. | Propuesta | UUID aleatorio (no sobrevive a mover sin base de datos), ruta (cambia al renombrar) |
| DEC-004 | three.js 0.160.0 copiado en `assets/vendor/three/` (MIT) y embebido en el binario, para que no haya red (CTR-05). | Propuesta | Cargarlo de un CDN |
| DEC-005 | Licencia Apache-2.0 con `NOTICE` que nombra al autor; quien redistribuya debe conservar el `NOTICE` (sección 4.d de la licencia). | Titular, 2026-11-25 | — |
| DEC-006 | Servidor HTTP con `tiny_http` (síncrono, pocas dependencias) y rangos `Range` implementados a mano para poder saltar en vídeo/audio. | Propuesta | axum + tokio (más dependencias de las necesarias) |

## Abiertas

| ID | Pregunta | De qué depende |
|---|---|---|
| DEC-101 | ¿Empaquetar como app de escritorio (Tauri) además del binario? | Si el navegador molesta en el uso diario |
| DEC-102 | ¿Miniaturas generadas en disco para carpetas de decenas de miles de imágenes? | Medir rendimiento con un proyecto grande real |
| DEC-103 | ¿Mostrar las capas de un mismo elemento (cerrada/fondo/frente) en una sola fila de Comparar? | Uso real con las cajas |
