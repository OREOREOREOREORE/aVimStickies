mod notes;
mod tray;
mod windows;

use notes::{NoteContent, NoteMeta, NoteSummary};
use tauri::Manager;

pub(crate) fn create_note_impl(app: &tauri::AppHandle) -> Result<String, String> {
    let id = notes::new_id();
    std::fs::write(notes::note_path(&id), "").map_err(|e| e.to_string())?;
    let mut meta = notes::load_meta();
    meta.insert(id.clone(), NoteMeta::default());
    notes::save_meta(&meta);
    windows::open_note_window(app, &id).map_err(|e| e.to_string())?;
    tray::refresh_tray(app);
    Ok(id)
}

#[tauri::command]
fn create_note(app: tauri::AppHandle) -> Result<String, String> {
    create_note_impl(&app)
}

#[tauri::command]
fn list_notes() -> Vec<NoteSummary> {
    notes::load_meta()
        .keys()
        .filter_map(|id| notes::read_note(id).ok().map(|n| NoteSummary { id: n.id, title: n.title }))
        .collect()
}

#[tauri::command]
fn get_note(id: String) -> Result<NoteContent, String> {
    notes::read_note(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_note(id: String, content: String) -> Result<(), String> {
    notes::write_note(&id, &content).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_note_meta(id: String, meta: NoteMeta) -> Result<(), String> {
    let mut all = notes::load_meta();
    all.insert(id, meta);
    notes::save_meta(&all);
    Ok(())
}

#[tauri::command]
fn delete_note(app: tauri::AppHandle, id: String) -> Result<(), String> {
    trash::delete(notes::note_path(&id)).map_err(|e| e.to_string())?;
    let mut all = notes::load_meta();
    all.remove(&id);
    notes::save_meta(&all);
    if let Some(win) = app.get_webview_window(&id) {
        let _ = win.close();
    }
    tray::refresh_tray(&app);
    Ok(())
}

fn open_all(app: &tauri::AppHandle) {
    let meta = notes::load_meta();
    if meta.is_empty() {
        let _ = create_note_impl(app);
        return;
    }
    for id in notes::note_ids_sorted(&meta) {
        let _ = windows::open_note_window(app, &id);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_note,
            list_notes,
            get_note,
            save_note,
            update_note_meta,
            delete_note
        ])
        .setup(|app| {
            let handle = app.handle();
            open_all(handle);
            tray::build_tray(handle)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
