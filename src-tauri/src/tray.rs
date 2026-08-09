use tauri::{
    menu::{IsMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Wry,
};

use crate::{notes, windows};

pub fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let menu = build_menu(app)?;
    let mut builder = TrayIconBuilder::with_id("main")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(handle_tray_event);
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

pub fn refresh_tray(app: &AppHandle) {
    if let Some(tray) = app.tray_by_id("main") {
        if let Ok(menu) = build_menu(app) {
            let _ = tray.set_menu(Some(menu));
        }
    }
}

fn handle_tray_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "new-note" => {
            let _ = super::create_note_impl(app);
        }
        "settings" => {
            let _ = windows::open_settings_window(app);
        }
        "search" => {
            let _ = windows::open_search_window(app);
        }
        "quit" => app.exit(0),
        id => {
            let _ = windows::open_note_window(app, id);
        }
    }
}

fn build_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let mut note_items: Vec<MenuItem<Wry>> = Vec::new();

    let notes: Vec<(String, String)> = notes::note_ids_sorted(&notes::load_meta())
        .iter()
        .filter_map(|id| notes::read_note(id).ok().map(|n| (n.id, n.title)))
        .collect();

    if notes.is_empty() {
        note_items.push(MenuItem::with_id(
            app,
            "no-notes",
            "No notes",
            false,
            None::<&str>,
        )?);
    } else {
        for (id, title) in notes {
            note_items.push(MenuItem::with_id(app, &id, &title, true, None::<&str>)?);
        }
    }

    let separator_1 = PredefinedMenuItem::separator(app)?;
    let new_note = MenuItem::with_id(app, "new-note", "New note", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings…", true, None::<&str>)?;
    let search = MenuItem::with_id(app, "search", "Search…", true, None::<&str>)?;
    let separator_2 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit aVimStickies", true, None::<&str>)?;

    let mut items: Vec<&dyn IsMenuItem<Wry>> =
        note_items.iter().map(|i| i as &dyn IsMenuItem<Wry>).collect();
    items.push(&separator_1);
    items.push(&new_note);
    items.push(&settings);
    items.push(&search);
    items.push(&separator_2);
    items.push(&quit);

    Menu::with_items(app, &items)
}
