# Mapa de módulos

| ID | Módulo | Responsabilidad | Archivos | Spec |
|---|---|---|---|---|
| MOD-01 | Índice | Recorrer rutas, clasificar por tipo, medir, huella y ID, agrupar variantes, caché | `src/scan.rs`, `src/kind.rs`, `src/ids.rs` | [IDX-01](IDX-01-indice.md) |
| MOD-02 | Selección | Estado por archivo, guardado atómico en JSON, resumen, CSV | `src/store.rs` | [SEL-01](SEL-01-seleccion.md) |
| MOD-03 | Servidor | HTTP local con token, rutas de API y archivos con rangos | `src/server.rs` | [UI-01](UI-01-interfaz.md) |
| MOD-04 | Interfaz | Página única: tabla, galería, comparar, buscador, visor | `assets/index.html`, `assets/vendor/three/` | [UI-01](UI-01-interfaz.md) |
| MOD-05 | CLI | Argumentos, modo lista y búsqueda por terminal, abrir navegador | `src/main.rs` | [UI-01](UI-01-interfaz.md) |
