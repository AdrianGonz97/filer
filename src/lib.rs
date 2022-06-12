use std::error::Error;
use std::path::Path;
use std::{fs, io};

use clap::Parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Path: {}", args.path);

    let path = Path::new(&args.path);
    dbg!(path);

    let mut entries = fs::read_dir(args.path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    dbg!(entries);

    Ok(())
}

pub fn test() {
    // let cmd = Command::from_args();
}

pub fn append() {}

pub fn prepend() {}

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
}
