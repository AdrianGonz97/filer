# Filer

A simple file management CLI tool for renaming files in batches.

## Features

Easily append, prepend, delete, and replace file name values in bulk.
Changes can be applied to all files or to files that contain certain extensions.

## Usage

```bash
$ filer --help
filer 0.1.0
File management tool for renaming files in batches.

USAGE:
    filer.exe [OPTIONS] <--append <SUFFIX>|--prepend <PREFIX>|--delete <VALUE>|--replace <OLD> <NEW>> <PATH>

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

## Examples

Say we have the following directory:

```
dir
│   1.txt
│   2.txt
│   3.txt
│   4.txt
│   chunks.mp4
│   cookie.json
```

Let's prepend "Task" and append a "0" to the `TXT` files.

```sh
filer -p "Task " -a 0 -e txt /path/to/dir
```

Resulting changes:

```
dir
│   Task 10.txt
│   Task 20.txt
│   Task 30.txt
│   Task 40.txt
│   chunky.mp4
│   cookie.json
```

Let's replace "k" for an "te" to only `TXT` and `MP4` files.

```sh
filer -r k te -e txt -e mp4 /path/to/dir
```

Resulting changes:

```
dir
│   Taste 10.txt
│   Taste 20.txt
│   Taste 30.txt
│   Taste 40.txt
│   chuntey.mp4
│   cookie.json
```

Let's delete the space in the `TXT` files.

```sh
filer -d " " -e txt /path/to/dir
```

Resulting changes:

```
dir
│   Taste 10.txt
│   Taste 20.txt
│   Taste 30.txt
│   Taste 40.txt
│   chuntey.mp4
│   cookie.json
```
