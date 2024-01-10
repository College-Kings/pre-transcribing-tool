use crate::error::Error;
use crate::regexes::DIALOGUE_LINE_REGEX;
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

    for line in lines.iter_mut() {
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
