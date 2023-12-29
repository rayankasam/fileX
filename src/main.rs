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
    #[arg(short,long)]
    suffix: bool,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Arguments = Arguments::parse();
    let files: Vec<PathBuf> = filter_files(get_files(&args.path)?,&args.pattern);
    // Renames each file to the updated version
    for file in files.iter() {
        let updated_path = update_path(file.as_path(), &new_file_name(file.file_name().unwrap().to_str().unwrap(), &args.add, args.suffix));
        match fs::rename(file.as_path() ,updated_path) {
            Ok(_) => (),
            Err(err) => return Err(Box::new(err))
        }

    }
    Ok(())
}