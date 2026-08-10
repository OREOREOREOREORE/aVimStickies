mod notes;
mod preview;
mod settings;
mod tray;
mod watch;
mod windows;

use notes::{NoteContent, NoteMeta, NoteSummary};
use serde::Serialize;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
use tauri_plugin_updater::UpdaterExt;

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
fn render_markdown(content: String) -> String {
    preview::render(&content)
}

#[tauri::command]
fn set_pinned(app: tauri::AppHandle, id: String, pinned: bool) -> Result<(), String> {
    let mut all = notes::load_meta();
    if let Some(m) = all.get_mut(&id) {
        m.always_on_top = pinned;
        notes::save_meta(&all);
    }
    if let Some(win) = app.get_webview_window(&id) {
        win.set_always_on_top(pinned).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_note_color(id: String, color: String) -> Result<(), String> {
    let mut all = notes::load_meta();
    if let Some(m) = all.get_mut(&id) {
        m.color = Some(color);
        notes::save_meta(&all);
    }
    Ok(())
}

#[tauri::command]
fn get_settings() -> settings::Settings {
    settings::load()
}

#[tauri::command]
fn save_settings(app: tauri::AppHandle, new_settings: settings::Settings) -> Result<(), String> {
    settings::save(&new_settings);
    let _ = app.emit("settings-changed", new_settings);
    Ok(())
}

#[derive(Serialize)]
struct SearchResult {
    id: String,
    title: String,
    snippet: String,
}

#[tauri::command]
fn search_notes(query: String) -> Vec<SearchResult> {
    let q = query.to_lowercase();
    let mut results = Vec::new();
    for id in notes::note_ids_sorted(&notes::load_meta()) {
        if let Ok(n) = notes::read_note(&id) {
            if n.content.to_lowercase().contains(&q) || n.title.to_lowercase().contains(&q) {
                let snippet = snippet_of(&n.content, &q);
                results.push(SearchResult {
                    id: n.id,
                    title: n.title,
                    snippet,
                });
            }
        }
    }
    results
}

fn snippet_of(content: &str, query: &str) -> String {
    let lower = content.to_lowercase();
    if let Some(idx) = lower.find(query) {
        let start = idx.saturating_sub(40);
        let end = (idx + query.len() + 40).min(content.len());
        let slice = content[start..end].trim();
        let prefix = if start > 0 { "…" } else { "" };
        let suffix = if end < content.len() { "…" } else { "" };
        format!("{prefix}{slice}{suffix}")
    } else {
        content.chars().take(80).collect()
    }
}

#[tauri::command]
fn open_settings(app: tauri::AppHandle) -> Result<(), String> {
    windows::open_settings_window(&app).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_search(app: tauri::AppHandle) -> Result<(), String> {
    windows::open_search_window(&app).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_note(app: tauri::AppHandle, id: String) -> Result<(), String> {
    windows::open_note_window(&app, &id).map_err(|e| e.to_string())
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
        let _ = win.destroy();
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

fn check_for_updates(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let updater = match app.updater() {
            Ok(u) => u,
            Err(e) => {
                eprintln!("updater init failed: {e}");
                return;
            }
        };
        match updater.check().await {
            Ok(Some(update)) => {
                let version = update.version.clone();
                let _ = app.emit("update-available", version);
            }
            Ok(None) => {}
            Err(e) => eprintln!("update check failed: {e}"),
        }
    });
}

#[tauri::command]
async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater
        .check()
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "no update available".to_string())?;
    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| e.to_string())?;
    tauri::process::restart(&app.env());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            create_note,
            list_notes,
            get_note,
            save_note,
            render_markdown,
            set_pinned,
            set_note_color,
            get_settings,
            save_settings,
            search_notes,
            open_settings,
            open_search,
            open_note,
            install_update,
            update_note_meta,
            delete_note
        ])
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts(["cmd+n", "cmd+shift+f"])
                .expect("register shortcuts")
                .with_handler(|app, shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        if shortcut.matches(Modifiers::SUPER, Code::KeyN) {
                            let _ = create_note_impl(app);
                        } else if shortcut.matches(Modifiers::SUPER | Modifiers::SHIFT, Code::KeyF) {
                            let _ = windows::open_search_window(app);
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            let handle = app.handle();
            open_all(handle);
            tray::build_tray(handle)?;
            watch::start_watcher(handle.clone());
            check_for_updates(handle.clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
