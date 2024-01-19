use crate::settings::Settings;
use crate::speakers::get_speakers;
use crate::utils::get_files_from_dir;
use crate::{render_table_creator, transcribing_formatter, writing_formatter};
use std::fs;
use std::path::PathBuf;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

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
pub fn run_writing_formatter(settings: State<Settings>) {
    let selected_file = settings.selected_file.lock().unwrap().clone();
    let selected_folder = settings.selected_folder.lock().unwrap().clone();

    if let Some(path) = selected_file {
        writing_formatter::process_single_file(&get_speakers(&path), &path)
            .expect("Unable to convert file");
        println!("Converted file: {}", path.to_str().unwrap())
    } else if let Some(path) = selected_folder {
        let speakers = get_speakers(&path);
        let files = get_files_from_dir(path);

        for file in files {
            writing_formatter::process_single_file(&speakers, &file)
                .expect("Unable to convert file");
            println!("Converted file: {}", file.to_str().unwrap())
        }
    }
}

#[tauri::command]
pub fn run_transcribing_formatter(settings: State<Settings>) {
    // TODO: Log conversion progress to main window;

    let episode = *settings.episode.lock().unwrap();
    let selected_file = settings.selected_file.lock().unwrap().clone();
    let selected_folder = settings.selected_folder.lock().unwrap().clone();

    if let Some(path) = selected_file {
        transcribing_formatter::process_single_file(episode, &path)
            .expect("Unable to convert file");
        println!("Converted file: {}", path.to_str().unwrap())
    } else if let Some(path) = selected_folder {
        let files = get_files_from_dir(path);

        for file in files {
            transcribing_formatter::process_single_file(episode, &file)
                .expect("Unable to convert file");
            println!("Converted file: {}", file.to_str().unwrap())
        }
    }
}

#[tauri::command]
pub fn create_render_table(settings: State<Settings>) {
    let selected_file = settings.selected_file.lock().unwrap().clone();
    let selected_folder = settings.selected_folder.lock().unwrap().clone();

    match (selected_file, selected_folder) {
        (Some(path), None) => {
            if render_table_creator::process_single_file(path.clone()).is_ok() {
                println!("Created render table: {}", path.to_str().unwrap())
            }
        }
        (None, Some(path)) => {
            let files = fs::read_dir(path)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .collect::<Vec<PathBuf>>();

            for file in files {
                if render_table_creator::process_single_file(file.clone()).is_ok() {
                    println!("Created render table: {}", file.to_str().unwrap())
                }
            }
        }
        _ => {}
    }
}
