use anyhow::Result;
use rayon::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_files(
    dir: &PathBuf,
    recurse: bool,
    depth: Option<usize>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    Ok(WalkDir::new(dir)
        .max_depth(match (recurse, depth) {
            (false, _) => 1,
            (_, Some(a)) => a,
            (_, None) => std::usize::MAX,
        })
        .into_iter()
        .par_bridge()
        .filter_map(|entry| Some(entry.ok()?.path().to_path_buf()))
        .filter(|file| file.is_file())
        .collect())
}
