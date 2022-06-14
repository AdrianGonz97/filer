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

    /// Replaces all matches in the file name. Delimited with a "->". ex: "from->to"
    #[clap(short = 'r', long = "replace", parse(try_from_str=validate_replace))]
    pub replace: Option<String>,

    /// Deletes all matches in the file name.
    #[clap(short = 'd', long = "delete")]
    pub delete: Option<String>,

    /// File name changes only apply to files with the provided extension
    #[clap(short = 'e', long = "ext", parse(try_from_str=validate_extension))]
    pub extensions: Option<Vec<String>>,
}

/// Validates that the provided path is a directory
pub fn validate_path(s: &str) -> Result<String, String> {
    match fs::read_dir(s) {
        Ok(_) => Ok(s.to_owned()),
        Err(_) => Err("Target path must be a directory.".to_owned()),
    }
}

/// Validates that the provided extension doesn't include a '.'
pub fn validate_extension(s: &str) -> Result<String, String> {
    if s.starts_with('.') {
        return Err(
            "Please exclude the '.' character from that start of your extension.".to_owned(),
        );
    }
    return Ok(s.to_owned());
}

/// Validates that the provided value contains the "->" delimiter.
pub fn validate_replace(s: &str) -> Result<String, String> {
    if !s.contains("->") {
        return Err("The \"->\" delimiter is missing. Use as such: \"old->new\"".to_owned());
    }
    return Ok(s.to_owned());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_path() {
        assert_eq!(
            Err(format!("Target path must be a directory.")),
            validate_path("somerandomname")
        );
        assert_eq!(Ok(".".to_owned()), validate_path("."));
    }

    #[test]
    fn valid_extension() {
        assert_eq!(
            Err("Please exclude the '.' character from that start of your extension.".to_owned()),
            validate_extension(".txt")
        );

        assert_eq!(Ok("txt".to_owned()), validate_extension("txt"));
    }

    #[test]
    fn valid_replace() {
        assert_eq!(
            Err("The \"->\" delimiter is missing. Use as such: \"old->new\"".to_owned()),
            validate_replace("from to")
        );

        assert_eq!(Ok("from->to".to_owned()), validate_replace("from->to"));
    }
}
