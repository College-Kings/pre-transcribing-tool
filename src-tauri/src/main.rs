// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod transcriber;
mod settings;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::State;
use crate::settings::Settings;
use crate::transcriber::Transcriber;

struct Storage {
    selected_file: Mutex<Option<PathBuf>>,
    selected_folder: Mutex<Option<PathBuf>>,
    settings: Mutex<HashMap<Settings, String>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn file_dialogue(state: State<Storage>, select_folder: bool) -> String {
    if select_folder {
        *state.selected_file.lock().unwrap() = None;
        *state.selected_folder.lock().unwrap() = FileDialogBuilder::new().pick_folder()
    } else {
        *state.selected_folder.lock().unwrap() = None;

        *state.selected_file.lock().unwrap() = FileDialogBuilder::new()
            .add_filter("Renpy Files (*.rpy)", &["rpy"])
            .add_filter("All Files", &["*"])
            .pick_file()
    }

    if select_folder {
        match *state.selected_folder.lock().unwrap() {
            Some(ref path) => format!("Selected folder: {}", path.to_str().unwrap()),
            None => "No folder selected".into(),
        }
    } else {
        match *state.selected_file.lock().unwrap() {
            Some(ref path) => format!("Selected file: {}", path.to_str().unwrap()),
            None => "No file selected".into(),
        }
    }

    // TODO: Toggle "Covert File" button to visible
}

#[tauri::command]
fn convert_file(state: State<Storage>) {
    // TODO: Log conversion progress to main window

    let settings_mutex = state.settings.lock();
    let settings = settings_mutex.as_deref().unwrap();
    let episode = settings.get(&Settings::Episode).unwrap().clone();

    if let Some(ref path) = *state.selected_file.lock().unwrap() {
        Transcriber::new(episode, path.clone()).run();

        println!("Converted file: {}", path.to_str().unwrap())
    } else if let Some(ref path) = *state.selected_folder.lock().unwrap() {
        let files = fs::read_dir(path).unwrap().filter_map(|entry| entry.ok()).map(|entry| entry.path()).collect::<Vec<PathBuf>>();

        for file in files {
            Transcriber::new(episode.clone(), file.clone()).run();

            println!("Converted file: {}", file.to_str().unwrap())
        }
    }
}

fn main() {
    let root_path = dirs::data_dir().unwrap().join("Viridian").join("PreTranscriber");
    let settings_file = root_path.join("settings.json");

    if !root_path.exists() {
        fs::create_dir_all(&root_path).unwrap();
    }

    let mut default_settings: HashMap<Settings, String> = HashMap::new();
    default_settings.insert(Settings::Episode, String::new());
    default_settings.insert(Settings::SceneNumber, String::new());
    default_settings.insert(Settings::SceneName, String::new());

    let json = serde_json::to_string_pretty(&default_settings).unwrap();
    if !settings_file.exists() {
        fs::write(&settings_file, json).unwrap();
    }

    tauri::Builder::default()
        .manage(Storage {
            selected_file: Mutex::new(None),
            selected_folder: Mutex::new(None),
            settings: Mutex::new(default_settings),
        })
        .invoke_handler(tauri::generate_handler![greet, file_dialogue, convert_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
