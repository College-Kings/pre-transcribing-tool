// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod error;
mod regexes;
mod render_table_creator;
mod settings;
mod transcribing_formatter;
mod utils;
mod writing_formatter;

use commands::{
    create_render_table, file_dialogue, run_transcribing_formatter, run_writing_formatter,
};
use error::Error;
use lazy_static::lazy_static;
use settings::{get_episode_number, set_episode_number, Settings};
use std::{fs, path::PathBuf};

lazy_static! {
    pub static ref DATA_ROOT: PathBuf = dirs::config_local_dir().unwrap().join("CollegeKingsTool");
    pub static ref SETTINGS_PATH: PathBuf = DATA_ROOT.join("settings.json");
}

fn main() -> Result<(), Error> {
    fs::create_dir_all(DATA_ROOT.as_path())?;

    tauri::Builder::default()
        .manage(Settings::new(1)?)
        .invoke_handler(tauri::generate_handler![
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
