mod get_files;
mod filter_files;
mod new_file_name;
mod update_path;
use std::path::PathBuf;
use std::fs;
use clap::Parser;
use filter_files::filter_files;
use new_file_name::new_file_name;
use crate::update_path::update_path;
use crate::get_files::get_files;

#[derive(Parser)]
struct Arguments {
    pattern: String,
    add: String,
    path: std::path::PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Arguments = Arguments::parse();
    let files: Vec<PathBuf> = filter_files(get_files(&args.path)?,&args.pattern);
    files
        .iter()
        .for_each(|file| 
            _ = fs::rename(file.as_path().to_str().unwrap() ,update_path(file.as_path().to_str().unwrap(), &new_file_name(&file.file_name().unwrap().to_str().unwrap(), &args.add))));
    Ok(())
}