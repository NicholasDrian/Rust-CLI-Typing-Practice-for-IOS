use std::fs;
use walkdir::{DirEntry, WalkDir};

fn load_from_path(path: &DirEntry) -> String {
    fs::read_to_string(path.path()).expect("Unable to read file")
}

fn get_sub_paths_with_ext(path: &String, extension: &String) -> Vec<DirEntry> {
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

pub fn get_sub_paths_with_exts(path: &String, extensions: &Vec<String>) -> Vec<DirEntry> {
    let mut res: Vec<DirEntry> = vec![];
    for ext in extensions {
        res.append(&mut get_sub_paths_with_ext(&path, &ext));
    }
    res
}

pub fn concat_file_contents(paths: Vec<DirEntry>) -> String {
    let mut res: String = "".to_string();
    for path in paths {
        res.push_str(&load_from_path(&path));
    }
    res
}
