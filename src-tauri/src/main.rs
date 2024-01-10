// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod error;
mod transcribing_formatter;
mod regexes;
mod render_table_creator;
mod settings;
mod speakers;
mod writing_formatter;
mod utils;

use commands::{run_transcribing_formatter, create_render_table, file_dialogue, greet, run_writing_formatter};
use error::Error;
use settings::{get_episode_number, initialise_settings, set_episode_number};

fn main() -> Result<(), Error> {
    let settings = initialise_settings();

    tauri::Builder::default()
        .manage(settings?)
        .invoke_handler(tauri::generate_handler![
            greet,
            file_dialogue,
            run_writing_formatter,
            run_transcribing_formatter,
            get_episode_number,
            set_episode_number,
            create_render_table
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
