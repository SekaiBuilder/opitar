//! SEL-01: the selection file. Keyed by content ID, saved atomically on every change.
//! Format v2 (English keys); v1 files (Spanish keys) are migrated on read (SEL-01-R07, CAM-003).

use crate::scan::Entry;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const NOTE_MAX: usize = 2000;
pub const TAG_MAX: usize = 40;
pub const VERSION: u32 = 2;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct Mark {
    #[serde(default, alias = "estado", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, alias = "nota", skip_serializing_if = "String::is_empty")]
    pub note: String,
    #[serde(default, alias = "etiquetas", skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, alias = "ruta", skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// Fields written by a newer Opitar are kept as they are (CTR-02).
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

impl Mark {
    fn is_empty(&self) -> bool {
        self.state.is_none() && self.note.is_empty() && self.tags.is_empty()
    }
}

/// v1 state names → v2.
fn migrate_state(s: Option<String>) -> Option<String> {
    match s.as_deref() {
        Some("queda") => Some("keep".into()),
        Some("fuera") => Some("out".into()),
        _ => s,
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SelectionFile {
    #[serde(default = "v1")]
    pub version: u32,
    #[serde(default, alias = "titulo")]
    pub title: String,
    #[serde(default, alias = "fuentes")]
    pub sources: Vec<String>,
    #[serde(default, alias = "actualizado")]
    pub updated: u64,
    #[serde(default, alias = "archivos")]
    pub files: BTreeMap<String, Mark>,
    /// Always recomputed on save; whatever was there on load (including v1's `resumen`) is dropped.
    #[serde(default, alias = "resumen")]
    pub summary: Value,
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

fn v1() -> u32 {
    1
}

#[derive(Serialize, Default, Debug)]
pub struct Summary {
    pub kept: Vec<Ref>,
    pub out: Vec<Ref>,
    pub pending: Vec<Ref>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Ref {
    pub id: String,
    pub path: String,
}

/// A change to one file; `None` leaves that field as it is. `state: Some(None)` clears the state.
#[derive(Deserialize, Default, Debug)]
pub struct Change {
    pub id: String,
    #[serde(default, deserialize_with = "double_option")]
    pub state: Option<Option<String>>,
    pub note: Option<String>,
    pub tags: Option<Vec<String>>,
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
        let mut data = match std::fs::read(path) {
            Ok(bytes) => serde_json::from_slice::<SelectionFile>(&bytes)
                .map_err(|e| format!("{} is not a valid Opitar JSON ({}); left untouched. Use another --output.", path.display(), e))?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => SelectionFile { version: VERSION, ..Default::default() },
            Err(e) => return Err(format!("cannot read {}: {}", path.display(), e)),
        };
        if data.version > VERSION {
            return Err(format!("{} was written by a newer Opitar (v{}).", path.display(), data.version));
        }
        if data.version < VERSION {
            for mark in data.files.values_mut() {
                mark.state = migrate_state(mark.state.take());
            }
        }
        Ok(Store { path: path.to_path_buf(), data })
    }

    pub fn apply(&mut self, change: Change, path: &str) -> Result<(), String> {
        let mut mark = self.data.files.remove(&change.id).unwrap_or_default();
        if let Some(state) = change.state {
            match state.as_deref() {
                None | Some("keep") | Some("out") => mark.state = state,
                Some(other) => {
                    self.data.files.insert(change.id, mark);
                    return Err(format!("unknown state: {}", other));
                }
            }
        }
        if let Some(note) = change.note {
            mark.note = note.chars().take(NOTE_MAX).collect();
        }
        if let Some(tags) = change.tags {
            let mut clean: Vec<String> = Vec::new();
            for t in tags {
                let t: String = t.trim().trim_start_matches('#').chars().take(TAG_MAX).collect();
                if !t.is_empty() && !clean.contains(&t) {
                    clean.push(t);
                }
            }
            mark.tags = clean;
        }
        if !path.is_empty() {
            mark.path = path.to_string();
        }
        if !mark.is_empty() {
            self.data.files.insert(change.id, mark);
        }
        Ok(())
    }

    /// SEL-01-R03: `chosen` stays, the rest of its group goes; choosing it again clears the group.
    pub fn choose_in_group(&mut self, chosen: &str, group: &[(String, String)]) {
        let state = |s: &Store, id: &str| s.data.files.get(id).and_then(|m| m.state.clone());
        let others: Vec<&(String, String)> = group.iter().filter(|(id, _)| id != chosen).collect();
        let already = state(self, chosen).as_deref() == Some("keep")
            && others.iter().all(|(id, _)| state(self, id).as_deref() == Some("out"));
        let chosen_path = group.iter().find(|(id, _)| id == chosen).map(|(_, p)| p.clone()).unwrap_or_default();
        let set = |s: &mut Store, id: &str, path: &str, state: Option<&str>| {
            let change = Change { id: id.to_string(), state: Some(state.map(String::from)), ..Default::default() };
            let _ = s.apply(change, path);
        };
        let (mine, theirs) = if already { (None, None) } else { (Some("keep"), Some("out")) };
        set(self, chosen, &chosen_path, mine);
        for (id, path) in others {
            set(self, id, path, theirs);
        }
    }

    /// SEL-01-R04: rewrite the summary for the files of this session and save atomically as v2.
    pub fn save(&mut self, title: &str, sources: &[String], entries: &[Entry]) -> Result<(), String> {
        let mut summary = Summary::default();
        let mut seen = std::collections::HashSet::new();
        for e in entries {
            if !seen.insert(e.id.clone()) {
                continue;
            }
            let r = Ref { id: e.id.clone(), path: e.path.to_string_lossy().into_owned() };
            match self.data.files.get(&e.id).and_then(|m| m.state.as_deref()) {
                Some("keep") => summary.kept.push(r),
                Some("out") => summary.out.push(r),
                _ => summary.pending.push(r),
            }
        }
        self.data.version = VERSION;
        self.data.title = title.to_string();
        self.data.sources = sources.to_vec();
        self.data.updated = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        self.data.summary = serde_json::to_value(summary).map_err(|e| e.to_string())?;
        let json = serde_json::to_vec_pretty(&self.data).map_err(|e| e.to_string())?;
        write_atomic(&self.path, &json).map_err(|e| format!("could not save {}: {}", self.path.display(), e))
    }

    /// SEL-01-R06
    pub fn csv(&self, entries: &[Entry]) -> String {
        let mut out = String::from("id,state,kind,path,size,width,height,tags,note\n");
        for e in entries {
            let m = self.data.files.get(&e.id);
            let fields = [
                e.id.clone(),
                m.and_then(|m| m.state.clone()).unwrap_or_else(|| "pending".into()),
                e.kind.to_string(),
                e.path.to_string_lossy().into_owned(),
                e.size.to_string(),
                e.width.map(|v| v.to_string()).unwrap_or_default(),
                e.height.map(|v| v.to_string()).unwrap_or_default(),
                m.map(|m| m.tags.join(" ")).unwrap_or_default(),
                m.map(|m| m.note.clone()).unwrap_or_default(),
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

    fn entry(id: &str, path: &str) -> Entry {
        Entry {
            n: 0,
            id: id.into(),
            digest: [0; 32],
            source: 0,
            path: PathBuf::from(path),
            rel: path.into(),
            folder: String::new(),
            name: path.into(),
            ext: "png".into(),
            kind: "image",
            mime: "image/png".into(),
            size: 1,
            modified: 0,
            width: None,
            height: None,
            group: "image:logo".into(),
            copies: 1,
            norm_name: String::new(),
            norm_folder: String::new(),
        }
    }

    fn set(store: &mut Store, id: &str, state: Option<&str>) {
        let c = Change { id: id.into(), state: Some(state.map(String::from)), ..Default::default() };
        store.apply(c, "x").unwrap();
    }

    #[test]
    fn mark_survives_restart() {
        // SEL-01-A01
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("opitar.json");
        let entries = vec![entry("OP-AAAA-AAAA", "a.png")];
        let mut s = Store::open(&path).unwrap();
        set(&mut s, "OP-AAAA-AAAA", Some("keep"));
        s.save("t", &[], &entries).unwrap();
        let s2 = Store::open(&path).unwrap();
        assert_eq!(s2.data.files["OP-AAAA-AAAA"].state.as_deref(), Some("keep"));
        assert_eq!(s2.data.summary["kept"][0]["id"], "OP-AAAA-AAAA");
    }

    #[test]
    fn choose_in_group_and_toggle() {
        // SEL-01-A02
        let dir = tempfile::tempdir().unwrap();
        let mut s = Store::open(&dir.path().join("o.json")).unwrap();
        let g: Vec<(String, String)> = ["A", "B", "C"].iter().map(|i| (i.to_string(), format!("{}.png", i))).collect();
        s.choose_in_group("B", &g);
        let entries: Vec<Entry> = g.iter().map(|(i, p)| entry(i, p)).collect();
        s.save("t", &[], &entries).unwrap();
        assert_eq!(s.data.summary["kept"].as_array().unwrap().len(), 1);
        assert_eq!(s.data.summary["out"].as_array().unwrap().len(), 2);
        s.choose_in_group("B", &g);
        assert!(s.data.files.is_empty());
    }

    #[test]
    fn foreign_ids_and_unknown_fields_are_kept() {
        // SEL-01-A03, CTR-02
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("o.json");
        std::fs::write(&path, r#"{"version":2,"files":{"OP-ZZZZ-ZZZZ":{"state":"out","future":7}},"new_field":true}"#).unwrap();
        let mut s = Store::open(&path).unwrap();
        set(&mut s, "OP-AAAA-AAAA", Some("keep"));
        s.save("t", &[], &[entry("OP-AAAA-AAAA", "a.png")]).unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(v["files"]["OP-ZZZZ-ZZZZ"]["state"], "out");
        assert_eq!(v["files"]["OP-ZZZZ-ZZZZ"]["future"], 7);
        assert_eq!(v["new_field"], true);
    }

    #[test]
    fn v1_file_is_migrated_to_v2() {
        // SEL-01-A04, SEL-01-R07: a file written by Opitar v0.1.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("o.json");
        std::fs::write(
            &path,
            r#"{"version":1,"titulo":"Cajas","fuentes":["/x"],"actualizado":1,
                "archivos":{"OP-AAAA-AAAA":{"estado":"queda","nota":"la buena","etiquetas":["ui"],"ruta":"/x/a.png","futuro":1},
                            "OP-BBBB-BBBB":{"estado":"fuera"}},
                "resumen":{"se_quedan":[],"fuera":[],"sin_decidir":[]},"extra":true}"#,
        )
        .unwrap();
        let mut s = Store::open(&path).unwrap();
        assert_eq!(s.data.files["OP-AAAA-AAAA"].state.as_deref(), Some("keep"));
        assert_eq!(s.data.files["OP-BBBB-BBBB"].state.as_deref(), Some("out"));
        s.save("Boxes", &[], &[entry("OP-AAAA-AAAA", "/x/a.png")]).unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(v["version"], 2);
        let a = &v["files"]["OP-AAAA-AAAA"];
        assert_eq!((a["state"].as_str(), a["note"].as_str(), a["tags"][0].as_str(), a["path"].as_str()), (Some("keep"), Some("la buena"), Some("ui"), Some("/x/a.png")));
        assert_eq!(a["futuro"], 1);
        assert_eq!(v["extra"], true);
        assert!(v.get("archivos").is_none() && v.get("resumen").is_none() && v.get("titulo").is_none());
        assert_eq!(v["summary"]["kept"][0]["id"], "OP-AAAA-AAAA");
    }

    #[test]
    fn invalid_json_is_never_overwritten() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("o.json");
        std::fs::write(&path, "not json").unwrap();
        assert!(Store::open(&path).is_err());
    }

    #[test]
    fn tags_and_notes_are_cleaned_and_empty_marks_removed() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = Store::open(&dir.path().join("o.json")).unwrap();
        let c = Change { id: "A".into(), tags: Some(vec!["#ui".into(), "ui".into(), " ".into()]), ..Default::default() };
        s.apply(c, "a").unwrap();
        assert_eq!(s.data.files["A"].tags, vec!["ui"]);
        s.apply(Change { id: "A".into(), tags: Some(vec![]), ..Default::default() }, "a").unwrap();
        assert!(s.data.files.is_empty());
        assert!(s.apply(Change { id: "A".into(), state: Some(Some("x".into())), ..Default::default() }, "").is_err());
        let csv = s.csv(&[entry("A", "a,b.png")]);
        assert!(csv.contains("\"a,b.png\""));
    }
}
