use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder};

use crate::notes;

const DEFAULT_WIDTH: f64 = 240.0;
const DEFAULT_HEIGHT: f64 = 280.0;

pub fn open_note_window(app: &AppHandle, id: &str) -> tauri::Result<()> {
    if app.get_webview_window(id).is_some() {
        return Ok(());
    }

    let meta = notes::load_meta().get(id).cloned();

    let window = WebviewWindowBuilder::new(
        app,
        id,
        WebviewUrl::App(format!("?note={id}").into()),
    )
    .title("vStickier")
    .inner_size(DEFAULT_WIDTH, DEFAULT_HEIGHT)
    .build()?;

    if let Some(m) = meta {
        if let (Some(x), Some(y)) = (m.x, m.y) {
            let _ = window.set_position(PhysicalPosition::new(x, y));
        }
        if let (Some(w), Some(h)) = (m.width, m.height) {
            let _ = window.set_size(PhysicalSize::new(w, h));
        }
    }

    let app = app.clone();
    let id = id.to_string();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { .. } = event {
            if let Some(win) = app.get_webview_window(&id) {
                if let (Ok(pos), Ok(size)) = (win.inner_position(), win.inner_size()) {
                    let mut meta = notes::load_meta();
                    if let Some(m) = meta.get_mut(&id) {
                        m.x = Some(pos.x);
                        m.y = Some(pos.y);
                        m.width = Some(size.width);
                        m.height = Some(size.height);
                        notes::save_meta(&meta);
                    }
                }
            }
        }
    });

    Ok(())
}
