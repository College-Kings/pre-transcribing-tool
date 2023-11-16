mod animation_item;
mod config;
mod docx;
mod scene_item;

use crate::error::Error;
use animation_item::AnimationItem;
use config::{HEADER_KEYS, VALID_SCENE_STATEMENTS};
use docx::create_doc;
use regex::{Captures, Regex};
use scene_item::SceneItem;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn process_single_file(selected_file: PathBuf) -> Result<(), Error> {
    let mut header_data: HashMap<String, String> = HashMap::new();

    let file = File::open(selected_file)?;
    let reader = BufReader::new(file);

    let header_regex = Regex::new(&format!(r"^#\s*({}):\s*(.+)", HEADER_KEYS.join("|"))).unwrap();
    let scene_regex = Regex::new(&format!(
        r"^({})\s+(\S+)(.*)",
        VALID_SCENE_STATEMENTS.join("|")
    ))
    .unwrap();
    let scene_description_regex = Regex::new(r".+#\s*(.+)$").unwrap();
    let image_regex = Regex::new(r"^image\s+(\S+)\s*=(.+)").unwrap();
    let image_description_regex = Regex::new(r#""\s*(.+)"$"#).unwrap();
    let animation_regex = Regex::new(
        r#"\s*Movie\((?:play="([^"]+)",\s*)?(?:clothing="([^"]+)",\s*)?(?:angle="([^"]+)",\s*)?(?:speed="([^"]+)")?\)"#,
    )
        .unwrap();

    let mut scene_items: Vec<SceneItem> = Vec::new();
    let mut animation_items: Vec<AnimationItem> = Vec::new();

    let mut total_render_count = 0;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if let Some(captures) = header_regex.captures(line) {
            process_header_data(&mut header_data, captures)?;
            continue;
        }

        // scene|show image_name
        if let Some(captures) = scene_regex.captures(line) {
            process_scene(
                &mut scene_items,
                &mut total_render_count,
                captures,
                &scene_description_regex,
            )?;
            continue;
        }

        // image image_name =
        if let Some(captures) = image_regex.captures(line) {
            process_image(
                &mut animation_items,
                captures,
                &animation_regex,
                &mut total_render_count,
                &mut scene_items,
                &image_description_regex,
            )?;
            continue;
        }
    }

    create_doc(header_data, scene_items, total_render_count);

    Ok(())
}

fn process_header_data(
    header_data: &mut HashMap<String, String>,
    captures: Captures,
) -> Result<(), Error> {
    let key = captures
        .get(1)
        .ok_or_else(|| Error::SyntaxError(format!("Invalid header capture: {:?}", captures)))?
        .as_str();
    let value = captures
        .get(2)
        .ok_or_else(|| Error::SyntaxError(format!("Header key {} has no value", key)))?
        .as_str();

    header_data
        .entry(key.to_string())
        .and_modify(|x| *x = format!("{}; {}", x, value))
        .or_insert(value.to_string());

    Ok(())
}

fn process_scene(
    scene_items: &mut Vec<SceneItem>,
    render_count: &mut i32,
    captures: Captures,
    description_regex: &Regex,
) -> Result<(), Error> {
    let id = captures
        .get(2)
        .ok_or_else(|| Error::SyntaxError(format!("Invalid scene capture: {:?}", captures)))?
        .as_str();

    if id.contains("_ani") {
        return Ok(());
    }

    *render_count += 1;

    if let Some(rest) = captures.get(3) {
        match description_regex.captures(rest.as_str()) {
            Some(captures) => {
                let desc = captures
                    .get(1)
                    .ok_or_else(|| Error::SyntaxError(format!("Scene {} has no description", id)))?
                    .as_str();
                push_scene_to_vec(scene_items, id, desc)?;
            }
            None => {
                let scene = scene_items.iter_mut().find(|x| x.id == id).ok_or_else(|| {
                    Error::SyntaxError(format!("Scene {} has no description", id,))
                })?;
                scene.occurrences += 1;
            }
        }
    }
    Ok(())
}

fn process_animation(
    animation_items: &mut Vec<AnimationItem>,
    captures: Captures,
    id: &str,
) -> Result<(), Error> {
    let desc = captures
        .get(1)
        .ok_or_else(|| Error::SyntaxError(format!("Animation {} has no description", id)))?
        .as_str();
    let clothing = captures
        .get(2)
        .ok_or_else(|| Error::SyntaxError(format!("Animation {} has no clothing description", id)))?
        .as_str();
    let angle = captures
        .get(3)
        .ok_or_else(|| Error::SyntaxError(format!("Animation {} has no angle description", id)))?
        .as_str();
    let speed = captures
        .get(4)
        .ok_or_else(|| Error::SyntaxError(format!("Animation {} has no speed description", id)))?
        .as_str();

    animation_items.push(AnimationItem::new(
        id.to_string(),
        desc.to_string(),
        clothing.to_string(),
        angle.to_string(),
        speed.to_string(),
    ));

    Ok(())
}

fn process_image(
    animation_items: &mut Vec<AnimationItem>,
    captures: Captures,
    animation_regex: &Regex,
    render_count: &mut i32,
    scene_items: &mut Vec<SceneItem>,
    image_description_regex: &Regex,
) -> Result<(), Error> {
    let id = captures.get(1).unwrap().as_str();
    let rest = captures.get(2).unwrap().as_str();

    // image image_name = Movie(play="animation description", clothing="clothing description", angle="angle description", speed="speed description")
    if let Some(captures) = animation_regex.captures(rest) {
        process_animation(animation_items, captures, id)?;
        return Ok(());
    }

    *render_count += 1;

    match image_description_regex.captures(rest) {
        Some(captures) => {
            let desc = captures
                .get(1)
                .ok_or_else(|| Error::SyntaxError(format!("Image {} has no description", id)))?
                .as_str();
            push_scene_to_vec(scene_items, id, desc)?;
        }
        None => {
            let scene = scene_items
                .iter_mut()
                .find(|x| x.id == id)
                .ok_or_else(|| Error::SyntaxError(format!("Image {} has no description", id)))?;
            scene.occurrences += 1;
        }
    }

    Ok(())
}

fn push_scene_to_vec(scene_items: &mut Vec<SceneItem>, id: &str, desc: &str) -> Result<(), Error> {
    match scene_items.iter_mut().find(|x| x.id == id) {
        Some(item) => {
            if item.description != desc {
                return Err(Error::SyntaxError(format!(
                    "Scene ID {} has multiple descriptions: {} and {}",
                    id, item.description, desc
                )));
            }
            item.occurrences += 1;
        }
        None => scene_items.push(SceneItem::new(id.to_string(), desc.to_string())),
    }

    Ok(())
}
