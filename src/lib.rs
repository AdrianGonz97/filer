use std::error::Error;
use std::{fs, io};

use clap::Parser;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let dir = fs::read_dir(config.dir_path)?;

    // dbg!(dir);
    let mut entries = fs::read_dir(config.dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    dbg!(entries);

    Ok(())
}

pub fn test() {
    // let cmd = Command::from_args();
    let args = Args::parse();

    println!("Path: {}", args.path)
}

pub fn append() {}

pub fn prepend() {}

pub struct Config {
    pub value: String,
    pub dir_path: String,
    // pub command: Command,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let value = args[1].clone();
        let dir_path = args[2].clone();

        Ok(Config {
            value,
            dir_path,
            // command: Command::Prepend,
        })
    }
}

/// Simple program that modifies file names
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path to the target directory
    path: String,

    /// Appends a provided value to all files in a specified directory
    #[clap(short, long)]
    append: Option<String>,

    /// Prepends a provided value to all files in a specified directory
    #[clap(short, long)]
    prepend: Option<String>,
}
