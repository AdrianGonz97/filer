use std::path::{Path, PathBuf};
use std::{fs, io};

use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    let args = Args::parse();

    println!("Path: {}", args.path);

    // let path = Path::new(&args.path);

    let mut entries = fs::read_dir(args.path)?
        .map(|res| res.map(|e| e.path()))
        .flatten()
        .filter(|e| e.is_file())
        .collect::<Vec<_>>();

    dbg!(&entries);

    if let Some(mut ext) = args.extension {
        // if provided extension begins with a '.', it'll be removed
        if ext.starts_with('.') {
            ext.remove(0);
        }

        let paths = filter(&ext, entries);

        dbg!(&paths);
    }

    Ok(())
}

fn append() {}

fn prepend() {}

/// Filters out file paths that do not contain the provided extension
fn filter(ext: &str, paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let entries = paths
        .iter()
        .filter(|&e| e.extension().unwrap_or_default() == ext)
        .map(|e| e.to_owned())
        .collect::<Vec<PathBuf>>();

    return entries;
}

/// Simple program that modifies file names
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to the target directory
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
