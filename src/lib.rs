use anyhow::Result;
use clap::Parser;
use colored::Colorize;
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

    print_path(&args.path)?;

    // Filter by extension
    if let Some(extensions) = args.extensions {
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
        Err(e) => eprintln!("{}", format!("\n{e}").bright_red()),
    }

    Ok(())
}

use path_clean::PathClean;
fn print_path(path: &str) -> Result<()> {
    let path = PathBuf::from(path);

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    }
    .clean();

    println!(
        "Renaming files in directory: {}",
        format!("{:?}", absolute_path).bright_cyan()
    );

    Ok(())
}

/// Applies the name modification to targeted files
fn rename_files(initial_paths: Vec<PathBuf>, final_paths: Vec<PathBuf>) {
    println!("\n{}", format!("Initiating operation...").bright_cyan());
    for i in 0..initial_paths.len() {
        match fs::rename(&initial_paths[i], &final_paths[i]) {
            Ok(()) => println!(
                "{:^20}{:^10}{:^30}",
                format!("{:?}", initial_paths[i].file_name().unwrap())
                    .strikethrough()
                    .bright_red(),
                "->",
                format!("{:?}", final_paths[i].file_name().unwrap()).bright_green(),
            ),
            Err(e) => eprintln!("{}", e),
        }
    }
    println!("\n{}", format!("Operation completed!").bright_green());
}

/// Confirms the changes with the user
fn confirm(initial_paths: &Vec<PathBuf>, final_paths: &Vec<PathBuf>) -> Result<(), String> {
    if final_paths.len() == 0 {
        return Err(format!(
            "There are no files to rename that match the provided arguments!"
        ));
    }

    println!(
        "\nRenaming {} files:",
        format!("{}", initial_paths.len()).bright_cyan().bold()
    );
    for i in 0..initial_paths.len() {
        println!(
            "{:^20}{:^10}{:^30}",
            format!("{:?}", initial_paths[i].file_name().unwrap()).bright_red(),
            "->",
            format!("{:?}", final_paths[i].file_name().unwrap()).bright_green(),
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
