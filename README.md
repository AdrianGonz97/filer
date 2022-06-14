# Filer

A file management CLI tool for renaming files in batches.

## Features

## Usage

From `--help`:

```
File management tool for renaming files in batches.

USAGE:
    filer.exe [OPTIONS] <-a <SUFFIX>|-p <PREFIX>|-d <VALUE>|-r <OLD> <NEW>> <PATH>

ARGS:
    <PATH>    Path to the target directory

OPTIONS:
    -a, --append <SUFFIX>        Appends a value to the names of all files in a specified directory
    -d, --delete <VALUE>         Deletes all matches in the file name
    -e, --ext <EXTENSION>        File name changes only apply to files with the provided extension
    -h, --help                   Print help information
    -p, --prepend <PREFIX>       Prepends a value to the names of all files in a specified directory
    -r, --replace <OLD> <NEW>    Replaces all matches in the file name
    -V, --version                Print version information
```
