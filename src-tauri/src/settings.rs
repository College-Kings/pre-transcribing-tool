use crate::{
    error::Error, regexes::CHARACTER_DEFINITION_REGEX, utils::find_game_folder, SETTINGS_PATH,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub episode: Mutex<i32>,
    pub selected_file: Mutex<Option<PathBuf>>,
    pub selected_folder: Mutex<Option<PathBuf>>,
    pub scene_number: Mutex<Option<String>>,

    pub speakers: Mutex<HashMap<String, String>>,
}

impl Settings {
    pub fn new(episode: i32) -> Result<Self, Error> {
        if let Ok(settings) = Self::load() {
            return Ok(settings);
        }

        let settings = Self {
            episode: Mutex::new(episode),
            selected_file: Mutex::new(None),
            selected_folder: Mutex::new(None),
            scene_number: Mutex::new(None),

            speakers: Mutex::new(HashMap::new()),
        };

        settings.save()?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<(), Error> {
        fs::write(SETTINGS_PATH.as_path(), serde_json::to_string(&self)?)?;
        Ok(())
    }

    pub fn load() -> Result<Self, Error> {
        let settings = fs::read_to_string(SETTINGS_PATH.as_path())?;
        Ok(serde_json::from_str(&settings)?)
    }

    pub fn update_speakers(&self, file_path: &Path) -> Result<(), Error> {
        let character_file = find_game_folder(file_path)
            .ok_or("Unable to find game folder")?
            .join("characters.rpy");

        let lines: Vec<String> = std::fs::read_to_string(character_file)?
            .lines()
            .map(|line| line.to_string())
            .collect();

        let mut speakers: HashMap<String, String> = HashMap::new();

        for line in lines {
            let captures = match CHARACTER_DEFINITION_REGEX.captures(&line) {
                Some(captures) => captures,
                None => continue,
            };

            let character_var = captures
                .get(1)
                .unwrap()
                .as_str()
                .trim_start_matches("character.");
            let character_name = captures.get(2).unwrap().as_str();

            speakers.insert(character_var.into(), character_name.into());
        }

        let mut speakers_guard = self.speakers.lock().unwrap();
        *speakers_guard = speakers;

        Ok(())
    }

    pub fn get_speakers(&self, file_path: &Path) -> Result<HashMap<String, String>, Error> {
        let _ = self.update_speakers(file_path);
        let speakers = self.speakers.lock().unwrap();

        if speakers.is_empty() {
            return Err("No speakers found".into());
        }

        Ok(speakers.clone())
    }
}

#[tauri::command]
pub fn get_episode_number(settings: State<Settings>) -> i32 {
    *settings.episode.lock().unwrap()
}

#[tauri::command]
pub fn set_episode_number(settings: State<Settings>, value: i32) {
    *settings.episode.lock().unwrap() = value;
    println!("Episode number set to: {}", value)
}
