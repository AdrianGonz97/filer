use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::{
    fs,
    io::{self, Write},
};

mod args;
mod command;
use command::*;

pub fn run() -> Result<()> {
    let args = crate::args::Args::parse();

    let mut entries = fs::read_dir(&args.path)?
        .map(|res| res.map(|e| e.path()))
        .flatten()
        .filter(|e| e.is_file())
        .collect::<Vec<_>>();

    println!("Renaming files in directory: {}", args.path);

    // Filter by extension
    if let Some(extensions) = args.extension {
        let mut exts: Vec<String> = vec![];

        for mut ext in extensions {
            // if provided extension begins with a '.', it'll be removed
            if ext.starts_with('.') {
                ext.remove(0);
            }
            exts.push(ext);
        }
        entries = filter(exts, entries);
    }

    let mut final_paths = entries.clone();

    // Prepend
    if let Some(prefix) = args.prefix {
        final_paths = prepend(&prefix, &final_paths);
    }

    // Append
    if let Some(suffix) = args.suffix {
        final_paths = append(&suffix, &final_paths);
    }

    match confirm(&entries, &final_paths) {
        Ok(()) => rename_files(entries, final_paths),
        Err(e) => eprintln!("{e}"),
    }

    Ok(())
}

/// Applies the modification to all file names
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

/// Confirms the changes with the user
fn confirm(init: &Vec<PathBuf>, fin: &Vec<PathBuf>) -> Result<(), String> {
    if fin.len() == 0 {
        return Err(format!(
            "There are no files to rename that match the provided arguments!"
        ));
    }

    println!("\nRenaming {} files:", init.len());
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
