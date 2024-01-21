# cat_rust

**cat_rust** is a Rust implementation of the `cat` command, a standard Unix utility for concatenating and displaying the content of files.

## Dependencies

This program has been developed using Rust version 1.74.0. Command-line argument management has been simplified thanks to the use of the crate clap in its version 4.4.11. clap offers a declarative interface for defining and parsing program arguments, making it easy to create a friendly and efficient user experience.

## Installation

1. Clone the repository: `git clone https://github.com/tuusuario/cat_rust.git`
2. Navigate to the project directory: `cd cat_rust`
3. Compile the project: `cargo build --release`

## Options

- `-n, --line-pos`: Number the output lines.
- `-b, --line-pos-nonblank`: Number only non-blank output lines.
- `-e, --line-end`: Show end of the line with '$'.
- `-h, --help`: Show the program's help.
- `-V, --version`: Show the program's version.

## Usage

To use cat_rust, you can compile the program and run it from the command line. It supports various options to customize the output:

```bash
# Example usage with options
./cat-rust file1.txt file2.txt -n -b -e
```
