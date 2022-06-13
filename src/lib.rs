use std::ffi::OsString;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{
    fs,
    io::{self, Write},
};

use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    let args = Args::parse();

    let mut entries = fs::read_dir(&args.path)?
        .map(|res| res.map(|e| e.path()))
        .flatten()
        .filter(|e| e.is_file())
        .collect::<Vec<_>>();

    println!("Renaming files in directory: {}", args.path);

    // Filter by extension
    if let Some(mut ext) = args.extension {
        // if provided extension begins with a '.', it'll be removed
        if ext.starts_with('.') {
            ext.remove(0);
        }

        entries = filter(&ext, entries);
    }

    let mut final_paths = entries.clone();

    // Prepend
    if let Some(prefix) = args.prepend {
        final_paths = prepend(&prefix, &final_paths);
    }

    // Append
    if let Some(suffix) = args.append {
        final_paths = append(&suffix, &final_paths);
    }

    match confirm(&entries, &final_paths) {
        Ok(()) => rename_files(entries, final_paths),
        Err(e) => eprintln!("{e}"),
    }

    Ok(())
}

fn append(suffix: &str, paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut entries = vec![];

    for file in paths.iter() {
        let ext = file.extension().unwrap_or_default();
        let mut file_name = file
            .file_stem()
            .expect("should always have a file name")
            .to_os_string();

        file_name.push(suffix); // appends the suffix before the file extension
        let path = Path::new(&file_name).with_extension(ext);

        let mut f = file.to_owned();
        f.pop();
        f.push(path);

        entries.push(f);
    }

    return entries;
}

fn prepend(prefix: &str, paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut entries = vec![];

    for file in paths.iter() {
        let file_name = file.file_name().unwrap();
        let mut modified_name = OsString::new();

        modified_name.push(prefix);
        modified_name.push(file_name);

        let mut f = file.to_owned();
        f.pop();
        f.push(modified_name);

        entries.push(f);
    }

    return entries;
}

/// Filters out file paths that do not contain the provided extension
fn filter(ext: &str, paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let entries = paths
        .iter()
        .filter(|&e| e.extension().unwrap_or_default() == ext)
        .map(|e| e.to_owned())
        .collect::<Vec<PathBuf>>();

    return entries;
}

/// Renames all of the files
fn rename_files(from: Vec<PathBuf>, to: Vec<PathBuf>) {
    for i in 0..from.len() {
        match fs::rename(&from[i], &to[i]) {
            Ok(()) => println!(
                "Renaming file: {:?} -> {:?}",
                from[i].file_name().unwrap(),
                to[i].file_name().unwrap()
            ),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn validate_path(s: &str) -> Result<String, String> {
    match fs::read_dir(s) {
        Ok(_) => Ok(s.to_owned()),
        Err(_) => Err(format!("Target path must be a directory.")),
    }
}

/// Confirms with the user that the
fn confirm(init: &Vec<PathBuf>, fin: &Vec<PathBuf>) -> Result<(), String> {
    if fin.len() == 0 {
        return Err(format!(
            "There are no files to rename that match the provided arguments!"
        ));
    }

    println!("\nRenaming the following files:");
    for i in 0..init.len() {
        println!(
            "{:^20}{:^10}{:^30}",
            init[i].file_name().unwrap().to_str().unwrap(),
            "->",
            fin[i].file_name().unwrap().to_str().unwrap(),
        )
    }

    loop {
        print!("\nDo you want to continue? [Y/n] ");
        let _ = io::stdout().flush();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "" | "y" | "Y" => return Ok(()),
            "n" | "N" => return Err(format!("Operation cancelled")),
            _ => continue,
        }
    }
}

/// Simple program that modifies file names
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to the target directory
    #[clap(parse(try_from_str=validate_path))]
    path: String,

    /// Appends a provided value to the names of all files in a specified directory
    #[clap(short, long)]
    append: Option<String>,

    /// Prepends a provided value to the names of all files in a specified directory
    #[clap(short, long)]
    prepend: Option<String>,

    /// File name modifications only apply to files with the provided extension
    #[clap(short, long = "ext")]
    extension: Option<String>,
}
