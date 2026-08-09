use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn notes_dir() -> PathBuf {
    let home = dirs::home_dir().expect("no home directory");
    let dir = home.join("Stickies");
    fs::create_dir_all(&dir).expect("create notes dir");
    dir
}

fn meta_path() -> PathBuf {
    notes_dir().join("meta.json")
}

pub type Meta = HashMap<String, NoteMeta>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NoteMeta {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct NoteContent {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct NoteSummary {
    pub id: String,
    pub title: String,
}

pub fn load_meta() -> Meta {
    let path = meta_path();
    if !path.exists() {
        return Meta::new();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_meta(meta: &Meta) {
    let json = serde_json::to_string_pretty(meta).expect("serialize meta");
    fs::write(meta_path(), json).expect("write meta");
}

pub fn new_id() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("n{nanos}")
}

pub fn note_path(id: &str) -> PathBuf {
    notes_dir().join(format!("{id}.md"))
}

fn title_of(content: &str, id: &str) -> String {
    content
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| id.to_string())
}

pub fn read_note(id: &str) -> Result<NoteContent, std::io::Error> {
    let path = note_path(id);
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "note not found",
        ));
    }
    let content = fs::read_to_string(&path)?;
    let title = title_of(&content, id);
    Ok(NoteContent {
        id: id.to_string(),
        title,
        content,
    })
}

pub fn note_ids_sorted(meta: &Meta) -> Vec<String> {
    let mut ids: Vec<String> = meta.keys().cloned().collect();
    ids.sort();
    ids
}
