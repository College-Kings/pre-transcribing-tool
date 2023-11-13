// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod render_table_creator;
mod settings;
mod transcriber;
mod commands;
mod file_formatter;


use settings::{get_episode_number, set_episode_number, initialise_settings};
use commands::{greet, file_dialogue, convert_file};

const TESTING: bool = true;

fn main() {
    if TESTING {
        render_table_creator::main::main();
    } else {
        let settings = initialise_settings();

        tauri::Builder::default()
            .manage(settings)
            .invoke_handler(tauri::generate_handler![
            greet,
            file_dialogue,
            convert_file,
            get_episode_number,
            set_episode_number
        ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application"); 
    }

}
