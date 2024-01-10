use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref DIALOGUE_LINE_REGEX: Regex = Regex::new(r#"^ *(\w+) +".+""#).unwrap();
}
