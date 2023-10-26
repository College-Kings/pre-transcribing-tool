// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod settings;
mod transcriber;

use std::fs;
use std::path::PathBuf;

use crate::transcriber::Transcriber;
use settings::{initalise_settings, Settings};
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn file_dialogue(settings: State<Settings>, select_folder: bool) -> String {
    if select_folder {
        *settings.selected_file.lock().unwrap() = None;
        *settings.selected_folder.lock().unwrap() = FileDialogBuilder::new().pick_folder()
    } else {
        *settings.selected_folder.lock().unwrap() = None;

        *settings.selected_file.lock().unwrap() = FileDialogBuilder::new()
            .add_filter("Renpy Files (*.rpy)", &["rpy"])
            .add_filter("All Files", &["*"])
            .pick_file()
    }

    if select_folder {
        match *settings.selected_folder.lock().unwrap() {
            Some(ref path) => format!("Selected folder: {}", path.to_str().unwrap()),
            None => "No folder selected".into(),
        }
    } else {
        match *settings.selected_file.lock().unwrap() {
            Some(ref path) => format!("Selected file: {}", path.to_str().unwrap()),
            None => "No file selected".into(),
        }
    }
    // TODO: Toggle "Covert File" button to visible
}

#[tauri::command]
fn convert_file(settings: State<Settings>) {
    // TODO: Log conversion progress to main window;

    let episode = settings.episode.lock().unwrap().clone();
    let selected_file = settings.selected_file.lock().unwrap().clone();
    let selected_folder = settings.selected_folder.lock().unwrap().clone();

    match (selected_file, selected_folder) {
        (Some(path), None) => {
            Transcriber::new(episode, path.clone()).run();
            println!("Converted file: {}", path.to_str().unwrap())
        }
        (None, Some(path)) => {
            let files = fs::read_dir(path)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .collect::<Vec<PathBuf>>();

            for file in files {
                Transcriber::new(episode.clone(), file.clone()).run();

                println!("Converted file: {}", file.to_str().unwrap())
            }
        }
        _ => {}
    }
}

fn main() {
    let settings = initalise_settings();

    tauri::Builder::default()
        .manage(settings)
        .invoke_handler(tauri::generate_handler![greet, file_dialogue, convert_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
