use std::fs;
use walkdir::{DirEntry, WalkDir};

pub fn load_from_path(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}

fn get_sub_paths_with_ext(path: &str, extension: &str) -> Vec<DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .to_str()
                .unwrap()
                .chars()
                .rev()
                .zip(extension.chars().rev())
                .all(|(a, b)| a == b)
        })
        .collect()
}
