use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::notes;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub font_family: String,
    pub font_size: u32,
    pub theme: String,
    pub opacity: f64,
    pub show_preview_button: bool,
    pub show_action_buttons: bool,
    pub enable_color_cycle: bool,
    pub show_status_bar: bool,
    pub show_line_numbers: bool,
    #[serde(default = "default_true")]
    pub wrap_text: bool,
}

fn default_true() -> bool {
    true
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            font_family: "ui-monospace, SFMono-Regular, Menlo, monospace".into(),
            font_size: 13,
            theme: "light".into(),
            opacity: 1.0,
            show_preview_button: false,
            show_action_buttons: false,
            enable_color_cycle: false,
            show_status_bar: true,
            show_line_numbers: true,
            wrap_text: true,
        }
    }
}

fn settings_path() -> PathBuf {
    notes::notes_dir().join("settings.json")
}

pub fn load() -> Settings {
    let path = settings_path();
    if !path.exists() {
        return Settings::default();
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(settings: &Settings) {
    let json = serde_json::to_string_pretty(settings).expect("serialize settings");
    fs::write(settings_path(), json).expect("write settings");
}
