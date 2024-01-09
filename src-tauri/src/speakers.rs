use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref CHARACTER_DEFINITION_REGEX: Regex =
        Regex::new(r#"^ *define ([\w.]+) *= *Character\(_?\(?"([^"]+)""#).unwrap();
}

pub fn get_speakers(file_path: &Path) -> HashMap<String, String> {
    let mut character_file = find_game_folder(file_path).expect("Unable to find game folder");
    character_file.push("characters.rpy");

    let lines: Vec<String> = std::fs::read_to_string(character_file)
        .expect("Unable to read characters.rpy")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut speakers: HashMap<String, String> = HashMap::new();

    for line in lines {
        if !CHARACTER_DEFINITION_REGEX.is_match(&line) {
            continue;
        }

        let captures = CHARACTER_DEFINITION_REGEX.captures(&line).unwrap();
        let character_var = captures
            .get(1)
            .unwrap()
            .as_str()
            .trim_start_matches("character.");
        let character_name = captures.get(2).unwrap().as_str();

        speakers.insert(character_var.into(), character_name.into());
    }

    speakers
}

fn find_game_folder(mut current_path: &Path) -> Option<PathBuf> {
    while let Some(parent) = current_path.parent() {
        if parent.ends_with("game") {
            return Some(parent.to_path_buf());
        }

        current_path = parent;
    }

    None
}
