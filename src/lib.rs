use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{
    fs,
    io::{self, Write},
};

use anyhow::Result;
use clap::{ArgGroup, Parser};

pub fn run() -> Result<()> {
    let args = Args::parse();

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

/// Appends a suffix to the provided Paths
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

/// Prepends a prefix to the provided Paths
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
fn filter(extensions: Vec<String>, paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let entries = paths
        .iter()
        .filter(|&e| {
            for ext in &extensions {
                if e.extension().unwrap_or_default() == ext.as_str() {
                    return true;
                }
            }
            return false;
        })
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

/// Validates that the provided path is a directory
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

/// Simple program that modifies file names
#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .multiple(true)
        .args(&["suffix", "prefix"]),
))]
struct Args {
    /// Path to the target directory
    #[clap(parse(try_from_str=validate_path))]
    path: String,

    /// Appends a value to the names of all files in a specified directory
    #[clap(short = 'a', long = "append")]
    suffix: Option<String>,

    /// Prepends a value to the names of all files in a specified directory
    #[clap(short = 'p', long = "prepend")]
    prefix: Option<String>,

    /// File name modifications only apply to files with the provided extension
    #[clap(short = 'e', long = "ext")]
    extension: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_paths() {
        let paths = vec![PathBuf::from("./foo.txt"), PathBuf::from("./bar.txt")];

        assert_eq!(
            vec![PathBuf::from("./foobaz.txt"), PathBuf::from("./barbaz.txt")],
            append("baz", &paths)
        );
    }

    #[test]
    fn prepend_paths() {
        let paths = vec![PathBuf::from("./foo.txt"), PathBuf::from("./bar.txt")];

        assert_eq!(
            vec![PathBuf::from("./bazfoo.txt"), PathBuf::from("./bazbar.txt")],
            prepend("baz", &paths)
        );
    }

    #[test]
    fn filter_by_extensions_paths() {
        let paths = vec![
            PathBuf::from("./foo.txt"),
            PathBuf::from("./bar.txt"),
            PathBuf::from("./baz.txt"),
        ];

        assert_eq!(
            vec![
                PathBuf::from("./foo.txt"),
                PathBuf::from("./bar.txt"),
                PathBuf::from("./baz.txt")
            ],
            filter(vec!["txt".to_owned()], paths)
        );
    }
}
