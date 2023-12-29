use std::path::PathBuf;
pub fn filter_files(files: Vec<PathBuf>, pattern: &String) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|file| file.to_str().unwrap().contains(pattern))
        .collect()
}