use crate::error::Error;
use regex::Regex;
use std::path::PathBuf;
use std::{fs, io};

pub struct Transcriber {
    episode: i32,
    file: PathBuf,
    scene_number: String,
    lines: Vec<String>,
}

impl Transcriber {
    pub fn new(episode: i32, file: PathBuf) -> Result<Transcriber, io::Error> {
        let scene_number = file
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|stem| stem.replace("scene", ""))
            .unwrap_or("".into());

        let lines = fs::read_to_string(&file)?
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(Transcriber {
            episode,
            file,
            scene_number,
            lines,
        })
    }

    fn add_scene_numbers(&mut self) -> Result<(), regex::Error> {
        for i in 0..self.lines.len() {
            let mut line = self.lines[i].as_str();
            let indent_count = line.chars().take_while(|c| c.is_whitespace()).count();

            line = line.trim();

            let re = Regex::new(r#"\w+ +".+""#)?;
            if !re.is_match(line) {
                continue;
            }

            let mut sb = String::new();
            sb.push_str(&" ".repeat(indent_count));
            sb.push_str(&format!(
                "scene ep{}s{}_\n",
                self.episode, self.scene_number
            ));
            sb.push_str(&" ".repeat(indent_count));
            sb.push_str("with dissolve\n\n");
            sb.push_str(&" ".repeat(indent_count));
            sb.push_str(line);

            self.lines[i] = sb;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.add_scene_numbers()?;

        fs::write(&self.file, self.lines.join("\n"))?;
        Ok(())
    }
}
