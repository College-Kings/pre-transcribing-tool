use std::fs;
use std::path::{Path, PathBuf};

pub fn get_files_from_dir<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    fs::read_dir(path)
        .expect("Unable to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect()
}

pub fn find_game_folder(mut current_path: &Path) -> Option<PathBuf> {
    while let Some(parent) = current_path.parent() {
        if parent.ends_with("game") {
            return Some(parent.to_path_buf());
        }

        current_path = parent;
    }

    None
}
