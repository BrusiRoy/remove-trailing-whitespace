#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate serde_derive;
extern crate docopt;

use std::fs::File;
use std::io::prelude::*;

// Docopt usage string
docopt!(Args derive Debug, "
rtw.

Usage:
  ./rtw <source> <dest>
");

// TODO: - Support multiple files
//       - Spawn multiple threads (for fun)
//       - Print information to the screen
//       - Display usage when invalid args
//       - Support trailing tabs
//       - Show numbers of line modified

fn main() {
  // Read the command line arguments
  let args: Args = Args::docopt().deserialize().unwrap_or_else(|e| e.exit());
  println!("{:?}", args);

  let input_file = args.arg_source;
  let output_file = args.arg_dest;


  // Read input file
  println!("Opening input file '{}'.", input_file);
  let mut input_file = File::open(input_file)
    .expect("File not found.");

  let mut content = String::new();
  input_file.read_to_string(&mut content)
    .expect("Something went wrong reading the file.");

  // Remove trailing space
  let output_content = remove_trailing_whitespace(&content);

  // Write to ouput file
  println!("Writing result to file '{}'.", output_file);
  let mut output_file = File::create(output_file)
    .expect("Unable to create output file.");
  output_file.write_all(output_content.as_bytes())
    .expect("Something went wrong while writing output file.");
}

// Remove trailing whitespace
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

        last_char_index  = i + 1;
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

