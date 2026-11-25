# Contratos transversales

Lo que ningún cambio puede romper.

| ID | Contrato | Evidencia exigida cuando se toca |
|---|---|---|
| CTR-01 | **Solo lectura de los archivos del usuario.** Opitar no modifica, mueve ni borra ningún archivo indexado. Solo escribe `opitar.json` (o el `--salida`) y su caché `.opitar-cache.json` junto a él. | Revisión de toda escritura a disco en el diff |
| CTR-02 | **Formato de selección estable.** `opitar.json` lleva `version`; los campos nuevos son opcionales; una versión nueva lee los archivos de la anterior sin perder elecciones. | Fixture de la versión anterior cargado y guardado |
| CTR-03 | **Guardado atómico.** Se escribe en un temporal y se renombra; un corte no deja un JSON a medias. | Test de guardado |
| CTR-04 | **IDs estables.** El ID de un archivo depende solo de su contenido: mismo contenido → mismo ID, en cualquier ruta, máquina o versión de Opitar. Cambiar el algoritmo exige versión nueva del formato y migración. | Test con contenido fijo y su ID esperado |
| CTR-05 | **Nada sale del ordenador.** El servidor escucha solo en `127.0.0.1`, exige un token aleatorio por sesión, no sirve nada fuera de las rutas dadas y la página no carga recursos de internet. | Test de ruta fuera de la raíz y sin token |
| CTR-07 | **Experiencia.** Interfaz en español; teclado completo; estados vacío/carga/error. | Recorrido en el navegador |
