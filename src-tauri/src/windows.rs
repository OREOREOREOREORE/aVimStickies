use std::sync::atomic::{AtomicUsize, Ordering};

use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder};

use crate::notes;

const DEFAULT_WIDTH: f64 = 420.0;
const DEFAULT_HEIGHT: f64 = 320.0;
const MIN_WIDTH: f64 = 220.0;
const MIN_HEIGHT: f64 = 160.0;

static CASCADE: AtomicUsize = AtomicUsize::new(0);

fn cascade_position() -> (i32, i32) {
    let n = CASCADE.fetch_add(1, Ordering::Relaxed) % 8;
    (180 + n as i32 * 26, 120 + n as i32 * 26)
}

pub fn open_note_window(app: &AppHandle, id: &str) -> tauri::Result<()> {
    if let Some(win) = app.get_webview_window(id) {
        if win.is_visible().unwrap_or(true) {
            return Ok(());
        }
        let _ = win.show();
        let _ = win.set_focus();
        return Ok(());
    }

    let meta = notes::load_meta().get(id).cloned();

    let window = WebviewWindowBuilder::new(
        app,
        id,
        WebviewUrl::App(format!("?note={id}").into()),
    )
    .title("aVimStickies")
    .decorations(false)
    .inner_size(DEFAULT_WIDTH, DEFAULT_HEIGHT)
    .min_inner_size(MIN_WIDTH, MIN_HEIGHT)
    .build()?;

    if let Some(m) = meta {
        let has_position = m.x.is_some() && m.y.is_some();
        if has_position {
            if let (Some(x), Some(y)) = (m.x, m.y) {
                let _ = window.set_position(PhysicalPosition::new(x, y));
            }
        } else {
            let (cx, cy) = cascade_position();
            let _ = window.set_position(PhysicalPosition::new(cx, cy));
        }
        if let (Some(w), Some(h)) = (m.width, m.height) {
            let _ = window.set_size(PhysicalSize::new(w, h));
        }
        if m.always_on_top {
            let _ = window.set_always_on_top(true);
        }
    } else {
        let (cx, cy) = cascade_position();
        let _ = window.set_position(PhysicalPosition::new(cx, cy));
    }

    let app = app.clone();
    let id = id.to_string();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            if let Some(win) = app.get_webview_window(&id) {
                if let (Ok(pos), Ok(size)) = (win.inner_position(), win.inner_size()) {
                    let mut meta = notes::load_meta();
                    if let Some(m) = meta.get_mut(&id) {
                        m.x = Some(pos.x);
                        m.y = Some(pos.y);
                        if size.height >= 40 {
                            m.width = Some(size.width);
                            m.height = Some(size.height);
                        }
                        notes::save_meta(&meta);
                    }
                }
                let _ = win.hide();
            }
        }
    });

    Ok(())
}

pub fn open_settings_window(app: &AppHandle) -> tauri::Result<()> {
    if let Some(win) = app.get_webview_window("settings") {
        let _ = win.show();
        let _ = win.set_focus();
        return Ok(());
    }
    WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("settings".into()))
        .title("aVimStickies Settings")
        .inner_size(380.0, 480.0)
        .build()?;
    Ok(())
}

pub fn open_search_window(app: &AppHandle) -> tauri::Result<()> {
    if let Some(win) = app.get_webview_window("search") {
        let _ = win.show();
        let _ = win.set_focus();
        return Ok(());
    }
    WebviewWindowBuilder::new(app, "search", WebviewUrl::App("search".into()))
        .title("Search Notes")
        .inner_size(420.0, 480.0)
        .build()?;
    Ok(())
}
