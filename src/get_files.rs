use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_files(dir: &PathBuf, recurse: bool) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    Ok(WalkDir::new(dir)
        .max_depth(if recurse { std::usize::MAX } else { 1 })
        .into_iter()
        .filter_map(|entry| Some(entry.ok()?.path().to_path_buf()))
        .filter(|file| file.is_file())
        .collect())
}
