use std::fs;
use std::path::PathBuf;
use regex::Regex;

pub struct Transcriber {
    episode: String,
    file: PathBuf,
    scene_number: String,
    lines: Vec<String>,
}

impl Transcriber {
    pub fn new(episode: String, file: PathBuf) -> Transcriber {
        let scene_number = file.file_stem().and_then(|stem| stem.to_str()).map(|stem| stem.replace("scene", "")).unwrap_or("".into());

        Transcriber {
            episode,
            file: file.clone(),
            scene_number,
            lines: fs::read_to_string(file).unwrap().lines().map(|line| line.to_string()).collect(),
        }
    }

    fn add_scene_numbers(&mut self) {
        for i in 0..self.lines.len() {
            let mut line = self.lines[i].as_str();
            let indent_count = line.chars().take_while(|c| c.is_whitespace()).count();

            line = line.trim();

            let re = Regex::new(r#"[a-z]+ +".+""#).unwrap();
            if !re.is_match(line) {
                continue;
            }

            let mut sb = String::new();
            sb.push_str(&" ".repeat(indent_count));
            sb.push_str(&format!("scene {}s{}_n\n", self.episode, self.scene_number));
            sb.push_str(&" ".repeat(indent_count));
            sb.push_str("with dissolve\n\n");
            sb.push_str(&" ".repeat(indent_count));
            sb.push_str(line);

            self.lines[i] = sb;
        }
    }

    pub fn run(&mut self) {
        self.add_scene_numbers();

        fs::write(&self.file, self.lines.join("\n")).unwrap();
    }
}
