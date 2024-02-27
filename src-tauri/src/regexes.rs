use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref DIALOGUE_LINE_REGEX: Regex = Regex::new(r#"^ *(\w+) +".+("$| #)"#).unwrap();
    pub static ref OUTFIT_HEADER_REGEX: Regex = Regex::new(r#"^# Outfit: (\w+),$"#).unwrap();
    pub static ref HEADER_REGEX: Regex = Regex::new(r"^#\s*(.+):\s*(.+)").unwrap();
    pub static ref SCENE_REGEX: Regex = Regex::new(r"^(scene|show)\s+(\S+)(.*)").unwrap();
    pub static ref FRAT_SCENE_REGEX: Regex = Regex::new(r"^(frat_scene)\s+(\S+)(.*)").unwrap();
    pub static ref SCENE_DESCRIPTION_REGEX: Regex = Regex::new(r".+#\s*(.+)$").unwrap();
    pub static ref IMAGE_REGEX: Regex = Regex::new(r"^image\s+(\S+)\s*=(.+)").unwrap();
    pub static ref IMAGE_DESCRIPTION_REGEX: Regex = Regex::new(r#""\s*(.+)"$"#).unwrap();
    pub static ref ANIMATION_REGEX: Regex = Regex::new(
        r#"\s*Movie\((?:play="([^"]+)",\s*)?(?:clothing="([^"]+)",\s*)?(?:angle="([^"]+)",\s*)?(?:speed="([^"]+)")?\)"#
    ).unwrap();
    pub static ref CHARACTER_DEFINITION_REGEX: Regex = Regex::new(r#"^ *define ([\w.]+) *= *(Character|Speaker)\(_?\(?"([^"]+)""#).unwrap();
}
