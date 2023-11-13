use super::config::{HEADER_KEYS, VALID_SCENE_STATEMENTS};
use crate::error::Error;
use crate::render_table_creator::docx::create_doc;
use crate::settings::Settings;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct SceneItem {
    pub id: String,
    pub description: String,
    pub occurrences: i32,
}

impl SceneItem {
    pub fn new(id: String, description: String) -> Self {
        Self {
            id,
            description,
            occurrences: 1,
        }
    }
}

impl PartialEq for SceneItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn main(settings: Settings) {
    process_single_file(settings).expect("Failed to process file");
}

fn process_single_file(settings: Settings) -> Result<(), Error> {
    // let selected_file = settings.selected_file.lock().unwrap().clone().ok_or("No file selected")?;
    let selected_file = r"D:\Crimson Sky\College Kings\College-Kings-2\game\ep4\scene1a.rpy";

    let mut header_data: HashMap<String, String> = HashMap::new();
    for key in HEADER_KEYS {
        header_data.insert(key.to_string(), String::new());
    }

    let file = File::open(selected_file)?;
    let reader = BufReader::new(file);

    let header_capture_regex = Regex::new(&format!("# ({}): (.+)", HEADER_KEYS.join("|"))).unwrap();
    let scene_regex =
        Regex::new(&format!(r"^({})\s+(\S+)", VALID_SCENE_STATEMENTS.join("|"))).unwrap();
    let scene_description_regex = Regex::new(r".+#\s*(.+)$").unwrap();

    let mut scene_items: Vec<SceneItem> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        let line_number = index + 1;

        if let Some(captures) = header_capture_regex.captures(line) {
            let key = captures.get(1).unwrap().as_str();
            let value = captures.get(2).unwrap().as_str();

            header_data.insert(key.to_string(), value.to_string());
            continue;
        }

        if let Some(captures) = scene_regex.captures(line) {
            let scene_id = captures.get(2).unwrap().as_str();

            match scene_items.iter_mut().find(|x| x.id == scene_id) {
                Some(item) => item.occurrences += 1,
                None => {
                    let scene_desc = scene_description_regex
                        .captures(line)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str();
                    scene_items.push(SceneItem::new(scene_id.to_string(), scene_desc.to_string()));
                }
            }
            continue;
        }
    }
    create_doc(scene_items);

    Ok(())
}
