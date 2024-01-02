use regex::Regex;
use std::path::PathBuf;

pub fn filter_files(files: Vec<PathBuf>, pattern: &Regex) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|file| pattern.is_match(file.to_str().unwrap()))
        .collect()
}
