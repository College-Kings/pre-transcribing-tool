mod animation_item;
pub mod config;
mod docx;
mod scene_item;

use crate::error::Error;
use crate::regexes::{
    ANIMATION_REGEX, HEADER_REGEX, IMAGE_DESCRIPTION_REGEX, IMAGE_REGEX, SCENE_DESCRIPTION_REGEX,
    SCENE_REGEX,
};
use crate::render_table_creator::config::IGNORE_SCENES;
use animation_item::AnimationItem;
use docx::create_doc;
use regex::{Captures, Regex};
use scene_item::SceneItem;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn process_single_file(selected_file: PathBuf) -> Result<(), Error> {
    let mut header_data: HashMap<String, Vec<String>> = HashMap::new();
    let mut header_order: Vec<String> = Vec::new();
    let scene_number = selected_file
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .replace("scene", "");

    let file = File::open(&selected_file)?;
    let reader = BufReader::new(file);

    let mut header = true;
    let mut scene_items: Vec<SceneItem> = Vec::new();
    let mut animation_items: Vec<AnimationItem> = Vec::new();

    let mut total_render_count = 0;
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if header {
            if line.starts_with("label") {
                header = false;
                continue;
            }

            if let Some(captures) = HEADER_REGEX.captures(line) {
                if let Err(e) = process_header_data(&mut header_data, &mut header_order, captures) {
                    println!("Failed to process header: {}", e);
                    return Err(e);
                }
                continue;
            }
            continue;
        }

        // scene|show image_name
        if let Some(captures) = SCENE_REGEX.captures(line) {
            if let Err(e) = process_scene(
                &mut scene_items,
                &mut total_render_count,
                captures,
                &SCENE_DESCRIPTION_REGEX,
            ) {
                println!("Failed to process scene: {}", e);
                return Err(e);
            }
            continue;
        }

        // image image_name =
        if let Some(captures) = IMAGE_REGEX.captures(line) {
            if let Err(e) = process_image(
                &mut animation_items,
                captures,
                &ANIMATION_REGEX,
                &mut total_render_count,
                &mut scene_items,
                &IMAGE_DESCRIPTION_REGEX,
            ) {
                println!("Failed to process image: {}", e);
                return Err(e);
            }
            continue;
        }
    }

    let scene_item_sort_regex = Regex::new(r"ep(\d+)s(\d+)([a-z])*_(\d+)([a-z])*")?;

    scene_items.sort_by(|a, b| {
        let a_caps = scene_item_sort_regex.captures(&a.id).unwrap();
        let a_episode_number = a_caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let a_scene_number = a_caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let a_scene_suffix = a_caps.get(3).map(|m| m.as_str()).unwrap_or("");
        let a_scene_image_number = a_caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let a_scene_image_suffix = a_caps.get(5).map(|m| m.as_str()).unwrap_or("");

        let b_caps = scene_item_sort_regex.captures(&b.id).unwrap();
        let b_episode_number = b_caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b_scene_number = b_caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let b_scene_suffix = b_caps.get(3).map(|m| m.as_str()).unwrap_or("");
        let b_scene_image_number = b_caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let b_scene_image_suffix = b_caps.get(5).map(|m| m.as_str()).unwrap_or("");

        let a_cmp = (
            a_episode_number,
            a_scene_number,
            a_scene_suffix,
            a_scene_image_number,
            a_scene_image_suffix,
        );
        let b_cmp = (
            b_episode_number,
            b_scene_number,
            b_scene_suffix,
            b_scene_image_number,
            b_scene_image_suffix,
        );

        a_cmp.cmp(&b_cmp)
    });

    create_doc(
        &scene_number,
        header_data,
        header_order,
        scene_items,
        total_render_count,
    )
    .expect("Failed to create docx");

    Ok(())
}

fn process_header_data(
    header_data: &mut HashMap<String, Vec<String>>,
    header_order: &mut Vec<String>,
    captures: Captures,
) -> Result<(), Error> {
    let key = captures
        .get(1)
        .ok_or_else(|| Error::Syntax(format!("Invalid header capture: {:?}", captures)))?
        .as_str();
    let value = captures
        .get(2)
        .ok_or_else(|| Error::Syntax(format!("Header key {} has no value", key)))?
        .as_str();

    header_data
        .entry(key.to_string())
        .and_modify(|x| x.push(value.to_string()))
        .or_insert(vec![value.to_string()]);

    if !header_order.contains(&key.to_string()) {
        header_order.push(key.to_string());
    }

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
        .ok_or_else(|| Error::Syntax(format!("Invalid scene capture: {:?}", captures)))?
        .as_str();

    if IGNORE_SCENES.contains(&id) {
        return Ok(());
    }

    if id.contains("_ani") {
        return Ok(());
    }

    *render_count += 1;

    if let Some(rest) = captures.get(3) {
        match description_regex.captures(rest.as_str()) {
            Some(captures) => {
                let desc = captures
                    .get(1)
                    .ok_or_else(|| Error::Syntax(format!("Scene {} has no description", id)))?
                    .as_str();
                push_scene_to_vec(scene_items, id, desc)?;
            }
            None => {
                let scene = scene_items
                    .iter_mut()
                    .find(|x| x.id == id)
                    .ok_or_else(|| Error::Syntax(format!("Scene {} has no description", id,)))?;
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
        .ok_or_else(|| Error::Syntax(format!("Animation {} has no description", id)))?
        .as_str();
    let clothing = captures
        .get(2)
        .ok_or_else(|| Error::Syntax(format!("Animation {} has no clothing description", id)))?
        .as_str();
    let angle = captures
        .get(3)
        .ok_or_else(|| Error::Syntax(format!("Animation {} has no angle description", id)))?
        .as_str();
    let speed = captures
        .get(4)
        .ok_or_else(|| Error::Syntax(format!("Animation {} has no speed description", id)))?
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
                .ok_or_else(|| Error::Syntax(format!("Image {} has no description", id)))?
                .as_str();
            push_scene_to_vec(scene_items, id, desc)?;
        }
        None => {
            let scene = scene_items
                .iter_mut()
                .find(|x| x.id == id)
                .ok_or_else(|| Error::Syntax(format!("Image {} has no description", id)))?;
            scene.occurrences += 1;
        }
    }

    Ok(())
}

fn push_scene_to_vec(scene_items: &mut Vec<SceneItem>, id: &str, desc: &str) -> Result<(), Error> {
    match scene_items.iter_mut().find(|x| x.id == id) {
        Some(item) => {
            if item.description != desc {
                return Err(Error::Syntax(format!(
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
