use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Mutex};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub episode: Mutex<i32>,
    pub selected_file: Mutex<Option<PathBuf>>,
    pub selected_folder: Mutex<Option<PathBuf>>,
    pub scene_number: Mutex<Option<String>>,
}

impl Settings {
    pub fn new(episode: i32) -> Self {
        Self {
            episode: Mutex::new(episode),
            selected_file: Mutex::new(None),
            selected_folder: Mutex::new(None),
            scene_number: Mutex::new(None),
        }
    }
}

pub fn initalise_settings() -> Settings {
    let settings_file = dirs::data_dir()
        .unwrap()
        .join("Viridian")
        .join("pre-transcriber")
        .join("settings.json");

    if settings_file.exists() {
        let settings = fs::read_to_string(&settings_file).unwrap();
        let settings: Settings = serde_json::from_str(&settings).unwrap_or(Settings::new(1));

        return settings;
    } else {
        fs::create_dir_all(&settings_file).unwrap();
    }

    Settings::new(1)
}
