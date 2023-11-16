use crate::render_table_creator;
use crate::settings::Settings;
use crate::transcriber::Transcriber;
use std::fs;
use std::path::PathBuf;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn file_dialogue(settings: State<Settings>, select_folder: bool) -> String {
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
pub fn convert_file(settings: State<Settings>) {
    // TODO: Log conversion progress to main window;

    let episode = *settings.episode.lock().unwrap();
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
                Transcriber::new(episode, file.clone()).run();

                println!("Converted file: {}", file.to_str().unwrap())
            }
        }
        _ => {}
    }
}

#[tauri::command]
pub fn create_render_table(settings: State<Settings>) {
    let selected_file = settings.selected_file.lock().unwrap().clone();
    let selected_folder = settings.selected_folder.lock().unwrap().clone();

    match (selected_file, selected_folder) {
        (Some(path), None) => {
            let _ = render_table_creator::process_single_file(path.clone());
            println!("Converted file: {}", path.to_str().unwrap())
        }
        (None, Some(path)) => {
            let files = fs::read_dir(path)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .collect::<Vec<PathBuf>>();

            for file in files {
                let _ = render_table_creator::process_single_file(file.clone());

                println!("Converted file: {}", file.to_str().unwrap())
            }
        }
        _ => {}
    }
}
