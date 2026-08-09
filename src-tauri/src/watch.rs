use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use tauri::{AppHandle, Emitter};

use crate::notes;

pub fn start_watcher(app: AppHandle) {
    std::thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
            Ok(w) => w,
            Err(_) => return,
        };
        let dir = notes::notes_dir();
        if watcher.watch(&dir, RecursiveMode::NonRecursive).is_err() {
            return;
        }
        for event in rx {
            if let Ok(event) = event {
                for path in event.paths {
                    if let Some(id) = note_id_from_path(&path) {
                        let _ = app.emit("note-changed", id);
                    }
                }
            }
        }
    });
}

fn note_id_from_path(path: &std::path::Path) -> Option<String> {
    let name = path.file_name()?.to_str()?;
    name.strip_prefix('n').and_then(|rest| {
        rest.strip_suffix(".md").map(|id| format!("n{id}"))
    })
}
