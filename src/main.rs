mod get_files;
use crate::get_files::get_files;
mod filter_files;
use crate::filter_files::filter_files;
mod new_file_name;
use crate::new_file_name::new_file_name;
mod update_path;
use crate::update_path::update_path;

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Arguments {
    /// Regex pattern for files
    pattern: Regex,
    /// Path of directory to act on
    path: std::path::PathBuf,
    /// Text to add to the file name (Optional)
    #[arg(short, long)]
    add: Option<String>,
    /// If the text 'add' should become the suffix, as opposed to prefix
    #[arg(short, long)]
    suffix: bool,
    /// Update file extensions of these file to what extension (Optional)
    #[arg(short, long)]
    extension: Option<String>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Arguments = Arguments::parse();
    let files: Vec<PathBuf> = filter_files(get_files(&args.path)?, &args.pattern);
    // Renames each file to the updated version
    for file in files.iter() {
        let updated_path = update_path(
            file.as_path(),
            &new_file_name(
                file.file_name().unwrap().to_str().unwrap(),
                &args.add,
                args.suffix,
            ),
            &args.extension,
        );
        fs::rename(&file.as_path(), &updated_path).with_context(|| {
            format!(
                "Unable to rename file\n{} to {}",
                &file.as_path().display(),
                &updated_path.as_path().display()
            )
        })?;
    }
    Ok(())
}
