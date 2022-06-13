use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// Appends a suffix to the provided Paths
pub fn append(suffix: &str, paths: &Vec<PathBuf>) -> Vec<PathBuf> {
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
pub fn prepend(prefix: &str, paths: &Vec<PathBuf>) -> Vec<PathBuf> {
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
pub fn filter(extensions: Vec<String>, paths: Vec<PathBuf>) -> Vec<PathBuf> {
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
