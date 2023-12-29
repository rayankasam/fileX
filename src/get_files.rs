use std::{fs, path::PathBuf};
use anyhow::{Context, Result};
pub fn get_files(dir: &PathBuf) -> Result<Vec<PathBuf>,Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir)
                    .with_context(|| format!("Error reading file: {}", dir.display()))?;
    Ok(entries.filter_map(|entry| Some(entry.ok()?.path()))
            .filter(|file| file.is_file())
            .collect())
}