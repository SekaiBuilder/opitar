//! SEL-01: the selection file. Keyed by content ID, saved atomically on every change.

use crate::scan::Entry;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const NOTE_MAX: usize = 2000;
pub const TAG_MAX: usize = 40;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct Mark {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estado: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub nota: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub etiquetas: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ruta: String,
    /// Fields written by a newer Opitar are kept as they are (CTR-02).
    #[serde(flatten)]
    pub otros: Map<String, Value>,
}

impl Mark {
    fn is_empty(&self) -> bool {
        self.estado.is_none() && self.nota.is_empty() && self.etiquetas.is_empty()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SelectionFile {
    #[serde(default = "one")]
    pub version: u32,
    #[serde(default)]
    pub titulo: String,
    #[serde(default)]
    pub fuentes: Vec<String>,
    #[serde(default)]
    pub actualizado: u64,
    #[serde(default)]
    pub archivos: BTreeMap<String, Mark>,
    #[serde(default)]
    pub resumen: Summary,
    #[serde(flatten)]
    pub otros: Map<String, Value>,
}

fn one() -> u32 {
    1
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Summary {
    pub se_quedan: Vec<Ref>,
    pub fuera: Vec<Ref>,
    pub sin_decidir: Vec<Ref>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ref {
    pub id: String,
    pub ruta: String,
}

/// A change to one file; `None` leaves that field as it is. `estado: Some(None)` clears the state.
#[derive(Deserialize, Default, Debug)]
pub struct Change {
    pub id: String,
    #[serde(default, deserialize_with = "double_option")]
    pub estado: Option<Option<String>>,
    pub nota: Option<String>,
    pub etiquetas: Option<Vec<String>>,
}

fn double_option<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<Option<String>>, D::Error> {
    Ok(Some(Option::deserialize(d)?))
}

pub struct Store {
    pub path: PathBuf,
    pub data: SelectionFile,
}

impl Store {
    /// A missing file is an empty selection; an unreadable or invalid one is an error, so it is never overwritten.
    pub fn open(path: &Path) -> Result<Store, String> {
        let data = match std::fs::read(path) {
            Ok(bytes) => serde_json::from_slice::<SelectionFile>(&bytes)
                .map_err(|e| format!("{} no es un JSON de Opitar válido ({}); no se toca. Usa otro --salida.", path.display(), e))?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => SelectionFile { version: 1, ..Default::default() },
            Err(e) => return Err(format!("no se puede leer {}: {}", path.display(), e)),
        };
        if data.version > 1 {
            return Err(format!("{} es de una versión más nueva de Opitar (v{}).", path.display(), data.version));
        }
        Ok(Store { path: path.to_path_buf(), data })
    }

    pub fn apply(&mut self, change: Change, ruta: &str) -> Result<(), String> {
        let mut mark = self.data.archivos.remove(&change.id).unwrap_or_default();
        if let Some(estado) = change.estado {
            match estado.as_deref() {
                None | Some("queda") | Some("fuera") => mark.estado = estado,
                Some(other) => return Err(format!("estado desconocido: {}", other)),
            }
        }
        if let Some(nota) = change.nota {
            mark.nota = nota.chars().take(NOTE_MAX).collect();
        }
        if let Some(tags) = change.etiquetas {
            let mut clean: Vec<String> = Vec::new();
            for t in tags {
                let t: String = t.trim().trim_start_matches('#').chars().take(TAG_MAX).collect();
                if !t.is_empty() && !clean.contains(&t) {
                    clean.push(t);
                }
            }
            mark.etiquetas = clean;
        }
        if !ruta.is_empty() {
            mark.ruta = ruta.to_string();
        }
        if !mark.is_empty() {
            self.data.archivos.insert(change.id, mark);
        }
        Ok(())
    }

    /// SEL-01-R03: `chosen` stays, the rest of its group goes; choosing it again clears the group.
    pub fn choose_in_group(&mut self, chosen: &str, group: &[(String, String)]) {
        let state = |s: &Store, id: &str| s.data.archivos.get(id).and_then(|m| m.estado.clone());
        let others: Vec<&(String, String)> = group.iter().filter(|(id, _)| id != chosen).collect();
        let already = state(self, chosen).as_deref() == Some("queda")
            && others.iter().all(|(id, _)| state(self, id).as_deref() == Some("fuera"));
        let chosen_path = group.iter().find(|(id, _)| id == chosen).map(|(_, r)| r.clone()).unwrap_or_default();
        let set = |s: &mut Store, id: &str, ruta: &str, estado: Option<&str>| {
            let change = Change { id: id.to_string(), estado: Some(estado.map(String::from)), ..Default::default() };
            let _ = s.apply(change, ruta);
        };
        if already {
            set(self, chosen, &chosen_path, None);
            for (id, ruta) in others {
                set(self, id, ruta, None);
            }
        } else {
            set(self, chosen, &chosen_path, Some("queda"));
            for (id, ruta) in others {
                set(self, id, ruta, Some("fuera"));
            }
        }
    }

    /// SEL-01-R04: rewrite the summary for the files of this session and save atomically.
    pub fn save(&mut self, titulo: &str, fuentes: &[String], entries: &[Entry]) -> Result<(), String> {
        let mut summary = Summary::default();
        let mut seen = std::collections::HashSet::new();
        for e in entries {
            if !seen.insert(e.id.clone()) {
                continue;
            }
            let r = Ref { id: e.id.clone(), ruta: e.ruta.to_string_lossy().into_owned() };
            match self.data.archivos.get(&e.id).and_then(|m| m.estado.as_deref()) {
                Some("queda") => summary.se_quedan.push(r),
                Some("fuera") => summary.fuera.push(r),
                _ => summary.sin_decidir.push(r),
            }
        }
        self.data.version = 1;
        self.data.titulo = titulo.to_string();
        self.data.fuentes = fuentes.to_vec();
        self.data.actualizado = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        self.data.resumen = summary;
        let json = serde_json::to_vec_pretty(&self.data).map_err(|e| e.to_string())?;
        write_atomic(&self.path, &json).map_err(|e| format!("no se pudo guardar {}: {}", self.path.display(), e))
    }

    /// SEL-01-R06
    pub fn csv(&self, entries: &[Entry]) -> String {
        let mut out = String::from("id,estado,tipo,ruta,tamano,ancho,alto,etiquetas,nota\n");
        for e in entries {
            let m = self.data.archivos.get(&e.id);
            let estado = m.and_then(|m| m.estado.clone()).unwrap_or_else(|| "pendiente".into());
            let tags = m.map(|m| m.etiquetas.join(" ")).unwrap_or_default();
            let nota = m.map(|m| m.nota.clone()).unwrap_or_default();
            let fields = [
                e.id.clone(),
                estado,
                e.tipo.to_string(),
                e.ruta.to_string_lossy().into_owned(),
                e.tamano.to_string(),
                e.ancho.map(|v| v.to_string()).unwrap_or_default(),
                e.alto.map(|v| v.to_string()).unwrap_or_default(),
                tags,
                nota,
            ];
            let line: Vec<String> = fields.iter().map(|f| csv_field(f)).collect();
            out.push_str(&line.join(","));
            out.push('\n');
        }
        out
    }
}

fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// CTR-03: write to a sibling temp file, then rename over the target.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, ruta: &str) -> Entry {
        Entry {
            n: 0,
            id: id.into(),
            digest: [0; 32],
            fuente: 0,
            ruta: PathBuf::from(ruta),
            rel: ruta.into(),
            carpeta: String::new(),
            nombre: ruta.into(),
            ext: "png".into(),
            tipo: "imagen",
            mime: "image/png".into(),
            tamano: 1,
            modificado: 0,
            ancho: None,
            alto: None,
            grupo: "imagen:logo".into(),
            copias: 1,
        }
    }

    fn set(store: &mut Store, id: &str, estado: Option<&str>) {
        let c = Change { id: id.into(), estado: Some(estado.map(String::from)), ..Default::default() };
        store.apply(c, "x").unwrap();
    }

    #[test]
    fn mark_survives_restart() {
        // SEL-01-A01
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("opitar.json");
        let entries = vec![entry("OP-AAAA-AAAA", "a.png")];
        let mut s = Store::open(&path).unwrap();
        set(&mut s, "OP-AAAA-AAAA", Some("queda"));
        s.save("t", &[], &entries).unwrap();
        let s2 = Store::open(&path).unwrap();
        assert_eq!(s2.data.archivos["OP-AAAA-AAAA"].estado.as_deref(), Some("queda"));
        assert_eq!(s2.data.resumen.se_quedan[0].id, "OP-AAAA-AAAA");
    }

    #[test]
    fn choose_in_group_and_toggle() {
        // SEL-01-A02
        let dir = tempfile::tempdir().unwrap();
        let mut s = Store::open(&dir.path().join("o.json")).unwrap();
        let g: Vec<(String, String)> = ["A", "B", "C"].iter().map(|i| (i.to_string(), format!("{}.png", i))).collect();
        s.choose_in_group("B", &g);
        let entries: Vec<Entry> = g.iter().map(|(i, r)| entry(i, r)).collect();
        s.save("t", &[], &entries).unwrap();
        assert_eq!(s.data.resumen.se_quedan.len(), 1);
        assert_eq!(s.data.resumen.fuera.len(), 2);
        s.choose_in_group("B", &g);
        assert!(s.data.archivos.is_empty());
    }

    #[test]
    fn foreign_ids_and_unknown_fields_are_kept() {
        // SEL-01-A03, CTR-02
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("o.json");
        std::fs::write(
            &path,
            r#"{"version":1,"archivos":{"OP-ZZZZ-ZZZZ":{"estado":"fuera","futuro":7}},"campo_nuevo":true}"#,
        )
        .unwrap();
        let mut s = Store::open(&path).unwrap();
        set(&mut s, "OP-AAAA-AAAA", Some("queda"));
        s.save("t", &[], &[entry("OP-AAAA-AAAA", "a.png")]).unwrap();
        let text = std::fs::read_to_string(&path).unwrap();
        let v: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["archivos"]["OP-ZZZZ-ZZZZ"]["estado"], "fuera");
        assert_eq!(v["archivos"]["OP-ZZZZ-ZZZZ"]["futuro"], 7);
        assert_eq!(v["campo_nuevo"], true);
    }

    #[test]
    fn invalid_json_is_never_overwritten() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("o.json");
        std::fs::write(&path, "no es json").unwrap();
        assert!(Store::open(&path).is_err());
    }

    #[test]
    fn tags_and_notes_are_cleaned_and_empty_marks_removed() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = Store::open(&dir.path().join("o.json")).unwrap();
        let c = Change { id: "A".into(), etiquetas: Some(vec!["#ui".into(), "ui".into(), " ".into()]), ..Default::default() };
        s.apply(c, "a").unwrap();
        assert_eq!(s.data.archivos["A"].etiquetas, vec!["ui"]);
        s.apply(Change { id: "A".into(), etiquetas: Some(vec![]), ..Default::default() }, "a").unwrap();
        assert!(s.data.archivos.is_empty());
        assert!(s.apply(Change { id: "A".into(), estado: Some(Some("x".into())), ..Default::default() }, "").is_err());
        let csv = s.csv(&[entry("A", "a,b.png")]);
        assert!(csv.contains("\"a,b.png\""));
    }
}
