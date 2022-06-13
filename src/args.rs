use clap::{ArgGroup, Parser};
use std::fs;

/// Simple program that modifies file names
#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .multiple(true)
        .args(&["suffix", "prefix"]),
))]
pub struct Args {
    /// Path to the target directory
    #[clap(parse(try_from_str=validate_path))]
    pub path: String,

    /// Appends a value to the names of all files in a specified directory
    #[clap(short = 'a', long = "append")]
    pub suffix: Option<String>,

    /// Prepends a value to the names of all files in a specified directory
    #[clap(short = 'p', long = "prepend")]
    pub prefix: Option<String>,

    /// File name modifications only apply to files with the provided extension
    #[clap(short = 'e', long = "ext", parse(try_from_str=validate_extension))]
    pub extensions: Option<Vec<String>>,
}

/// Validates that the provided path is a directory
pub fn validate_path(s: &str) -> Result<String, String> {
    match fs::read_dir(s) {
        Ok(_) => Ok(s.to_owned()),
        Err(_) => Err(format!("Target path must be a directory.")),
    }
}

/// Validates that the provided extension doesn't include a '.'
pub fn validate_extension(s: &str) -> Result<String, String> {
    if s.starts_with('.') {
        return Err(format!(
            "Please exclude the '.' character from that start of your extension."
        ));
    }
    return Ok(s.to_owned());
}
