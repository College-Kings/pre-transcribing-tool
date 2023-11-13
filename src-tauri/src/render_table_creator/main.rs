use regex::Regex;
use serde_json::Value::String;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::Mutex;

struct State {
    pub episode: Mutex<i32>,
    pub selected_file: Mutex<Option<PathBuf>>,
    pub selected_folder: Mutex<Option<PathBuf>>,
    pub scene_number: Mutex<Option<String>>,
}

impl State {
    pub fn new<T>(episode: T) -> Self
    where
        T: Into<i32>,
    {
        Self {
            episode: Mutex::new(episode.into()),
            selected_file: Mutex::new(None),
            selected_folder: Mutex::new(None),
            scene_number: Mutex::new(None),
        }
    }
}

fn main() {
    let state = State {
        episode: Mutex::new(4),
        selected_file: Mutex::new(None),
        selected_folder: Mutex::new(None),
        scene_number: Mutex::new(Some("12b".to_string())),
    };

    let _ = process_single_file(state);
}

fn process_single_file(state: State) -> Result<(), String> {
    let selected_file = match state.selected_file.lock().unwrap().clone() {
        Some(path) => path,
        None => return Err("No file selected".to_string()),
    };

    let mut header_data: HashMap<String, String> = HashMap::new();
    header_data.insert("Writer".to_string(), String::new());
    header_data.insert("Scene Number".to_string(), String::new());
    header_data.insert("Location".to_string(), String::new());
    header_data.insert("MC Outfit".to_string(), String::new());
    header_data.insert("Outfit".to_string(), String::new());
    header_data.insert("Day".to_string(), String::new());
    header_data.insert("Time".to_string(), String::new());
    header_data.insert("TR".to_string(), String::new());

    let file = File::open(selected_file)?;
    let reader = BufReader::new(file);

    let header_capture_regex =
        Regex::new(&format!("# ({}): (.+)", header_data.keys().join("|"))).unwrap();

    for (index, line) in reader.lines().enumerate() {
        let line = line?.trim();
        let line_number = index + 1;

        if let Some(captures) = header_capture_regex.captures(line) {
            let key = captures.get(1).unwrap().as_str();
            let value = captures.get(2).unwrap().as_str();

            header_data.insert(key, value);
        }
    }

    Ok(())
}
