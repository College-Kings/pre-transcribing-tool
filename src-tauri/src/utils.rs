use std::fs;
use std::path::{Path, PathBuf};

pub fn get_files_from_dir<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    fs::read_dir(path)
        .expect("Unable to read directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>()
}
