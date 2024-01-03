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
    /// Path of directory to act on
    path: std::path::PathBuf,
    /// Regex pattern for files
    pattern: Regex,
    /// Text to add to the file name
    #[arg(short, long)]
    add: Option<String>,
    /// If the text 'add' should become the suffix, as opposed to prefix
    #[arg(short, long)]
    suffix: bool,
    /// Update file extensions of these file to what extension
    #[arg(short, long)]
    extension: Option<String>,
    /// Show all the files the pattern finds
    #[arg(short, long)]
    files: bool,
    /// Recursively searches sub-sirectories
    #[arg(short, long)]
    recurse: bool,
    /// Depth of directories searched from path
    #[arg(short, long)]
    depth: Option<usize>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Arguments = Arguments::parse();
    let files: Vec<PathBuf> = filter_files(
        get_files(&args.path, args.recurse, args.depth)?,
        &args.pattern,
    );
    if args.files {
        files.iter().for_each(|file| println!("{:?}", file));
    }
    // If there are no changes to be made, it doesn't rename anything
    if args.add.is_none() && args.extension.is_none() {
        return Ok(());
    }
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
