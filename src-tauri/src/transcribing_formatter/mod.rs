mod outfit_map;

use crate::error::Error;
use crate::regexes::{DIALOGUE_LINE_REGEX, OUTFIT_HEADER_REGEX};
use crate::transcribing_formatter::outfit_map::OUTFIT_MAP;
use std::fs;
use std::path::Path;

pub fn process_single_file(episode: i32, path: &Path) -> Result<(), Error> {
    let scene_number = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.replace("scene", ""))
        .unwrap_or("".into());

    let mut lines: Vec<String> = fs::read_to_string(path)?
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut day = String::new();
    for line in &lines {
        if line.starts_with("# Day: ") {
            day = line.replace("# Day: ", "");
            break;
        }

        if line.starts_with("label ") {
            break;
        }
    }

    for line in lines.iter_mut() {
        if line.starts_with("# Outfit: ") {
            *line = fill_in_outfit_number(&day, line);
            continue;
        }

        *line = add_scene_number(line, episode, &scene_number)?;
    }

    fs::write(path, lines.join("\n"))?;

    Ok(())
}

fn add_scene_number(
    line: &mut str,
    episode: i32,
    scene_number: &str,
) -> Result<String, regex::Error> {
    let indent_count = line.chars().take_while(|c| c.is_whitespace()).count();

    if !DIALOGUE_LINE_REGEX.is_match(line) {
        return Ok(line.to_string());
    }

    let mut sb = String::new();
    sb.push_str(&" ".repeat(indent_count));
    sb.push_str(&format!("scene ep{}s{}_\n\n", episode, scene_number));
    sb.push_str(line.trim_end());

    Ok(sb)
}

fn fill_in_outfit_number(day: &str, line: &mut str) -> String {
    let line = line.trim();

    if !OUTFIT_HEADER_REGEX.is_match(line) {
        return line.to_string();
    }

    let character = OUTFIT_HEADER_REGEX
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_lowercase();

    let outfit_number = OUTFIT_MAP
        .get(character.as_str())
        .and_then(|map| map.get(day.to_lowercase().as_str()))
        .copied();

    match outfit_number {
        Some(outfit_number) => format!("{} {}", line, outfit_number),
        None => line.to_string(),
    }
}
