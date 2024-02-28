use crate::error::Result;
use crate::regexes::DIALOGUE_LINE_REGEX;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub fn process_single_file(speakers_map: &HashMap<String, String>, path: &Path) -> Result<()> {
    let mut lines: Vec<String> = fs::read_to_string(path)?
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut speakers: HashSet<String> = HashSet::new();

    for line in lines.iter_mut() {
        add_speaker(line, &mut speakers)?;
    }

    lines.insert(0, add_header_data(speakers_map, speakers));

    fs::write(path, lines.join("\n"))?;

    Ok(())
}

fn add_speaker(line: &mut str, speakers: &mut HashSet<String>) -> Result<()> {
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

fn add_header_data(speakers_map: &HashMap<String, String>, speakers: HashSet<String>) -> String {
    let mut sb = String::new();
    sb.push_str("# Writer: \n");
    sb.push_str("# Location: \n");
    for speaker in speakers {
        if speaker == "u" {
            sb.push_str("# Outfit: MC, \n");
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
