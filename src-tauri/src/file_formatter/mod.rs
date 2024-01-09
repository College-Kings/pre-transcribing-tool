use crate::error::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

lazy_static! {
    static ref DIALOGUE_LINE_REGEX: Regex = Regex::new(r#"^ *(\w+) +".+""#).unwrap();
}

pub fn process_single_file(
    episode: i32,
    speakers_map: &HashMap<String, String>,
    path: &Path,
) -> Result<(), Error> {
    let scene_number = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.replace("scene", ""))
        .unwrap_or("".into());

    let mut lines: Vec<String> = fs::read_to_string(path)?
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut speakers: HashSet<String> = HashSet::new();

    for line in lines.iter_mut() {
        add_speaker(line, &mut speakers)?;
        *line = add_scene_number(line, episode, &scene_number)?;
    }

    lines.insert(0, add_header_data(speakers_map, speakers));

    fs::write(path, lines.join("\n"))?;

    Ok(())
}

fn add_speaker(line: &mut str, speakers: &mut HashSet<String>) -> Result<(), regex::Error> {
    if !DIALOGUE_LINE_REGEX.is_match(line) {
        return Ok(());
    }

    let speaker = DIALOGUE_LINE_REGEX
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    speakers.insert(speaker.into());

    Ok(())
}

fn add_scene_number(
    line: &mut str,
    episode: i32,
    scene_number: &str,
) -> Result<String, regex::Error> {
    let indent_count = line.chars().take_while(|c| c.is_whitespace()).count();

    if !DIALOGUE_LINE_REGEX.is_match(line) {
        return Ok(line.into());
    }

    let mut sb = String::new();
    sb.push_str(&" ".repeat(indent_count));
    sb.push_str(&format!("scene ep{}s{}_\n", episode, scene_number));
    sb.push_str(&" ".repeat(indent_count));
    sb.push_str("with dissolve\n\n");
    sb.push_str(line.trim_end());

    Ok(sb)
}

fn add_header_data(speakers_map: &HashMap<String, String>, speakers: HashSet<String>) -> String {
    let mut sb = String::new();
    sb.push_str("# Writer: \n");
    sb.push_str("# Location: \n");
    sb.push_str("# Outfit: MC, \n");
    for speaker in speakers {
        if speaker == "u" {
            continue;
        }

        sb.push_str(&format!(
            "# Outfit: {}, \n",
            speakers_map.get(&speaker).unwrap_or(&speaker)
        ));
    }
    sb.push_str("# Day: \n");
    sb.push_str("# Time: \n");
    sb.push_str("# Props: \n");
    sb.push_str("# Writer Notes: \n");
    sb.push('\n');

    sb.push_str("# Transcriber: \n");
    sb.push_str("# Transcriber Notes: \n");
    sb.push('\n');

    sb
}
