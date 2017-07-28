#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate serde_derive;
extern crate docopt;

use std::fs::File;
use std::io::prelude::*;

// TODO: - Support multiple files
//       - Spawn multiple threads (for fun)
//       - Support trailing tabs
//       - Show numbers of line modified
//       - Support both Windows and Linux
//       - Use regex
//       - Look into mmap()


// Docopt usage string
docopt!(Args derive Debug, "
rtw.

Usage:
  ./rtw (-i | --in-place) <file>...
  ./rtw (-d | --directory) <dir>...

Options:
  -i --in-place   Modifying the files in-place.
  -d --directory  Modifying all the files within a directory (in-place).
");

fn main() {
    // Read the command line arguments
    let args: Args = Args::docopt().deserialize().unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    if args.flag_in_place {
        for file in args.arg_file {
            remove_trailing_whitespace_in_place(&file[..]);
        }
    } else if args.flag_directory {
        // TODO: Find all the files within a directory
        for dir in args.arg_file {
            let paths = std::fs::read_dir(dir).unwrap(); // Maybe not use unwrap
            for path in paths {
                remove_trailing_whitespace_in_place(path.unwrap().path().to_str().unwrap());
            }
        }
    }
}

fn remove_trailing_whitespace_in_place(file: &str) {
    let input_content = extract_string_content(&file[..]);
    let output_content = remove_trailing_whitespace(&input_content);
    std::fs::remove_file(&file).expect("Unable to apply in-place.");
    write_to_file(&file[..], &output_content[..]);
}

/// Returns a String representing the content of input file `file_name`
fn extract_string_content(file_name: &str) -> String {
    println!("Opening input file '{}'.", file_name);
    let mut input_file = File::open(file_name).expect("File not found.");

    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Something went wrong reading the file.");

    content
}

/// Writes `content` to `file_name`
fn write_to_file(file_name: &str, content: &str) {
    println!("Writing result to file '{}'.", file_name);
    let mut output_file = File::create(file_name).expect("Unable to create output file.");
    output_file
        .write_all(content.as_bytes())
        .expect("Something went wrong while writing output file.");
}

/// Returns a String with the same content of `str` but with no trailing whitespace
fn remove_trailing_whitespace(str: &String) -> String {
    let mut return_str = String::new();
    let mut last_char_index = 0;
    let mut start_line_index = 0;

    for (i, c) in str.chars().enumerate() {
        match c {
            '\n' => {
                if last_char_index != start_line_index {
                    return_str.push_str(&str[start_line_index..last_char_index + 1]);
                }
                return_str.push('\n');

                last_char_index = i + 1;
                start_line_index = i + 1;
            }
            ' ' => {
                continue;
            }
            _ => {
                last_char_index = i;
            }
        }
    }

    return_str
}
