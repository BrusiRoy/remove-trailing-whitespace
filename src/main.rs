use std::env;

fn main() {
  // 0. Read the command line arguments
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    panic!("Invalid number of arguments");

  }

  let input_file  = &args[1];
  let output_file = &args[2];
  println!("Input file: {}", input_file);
  println!("Output file: {}", output_file);

  // 1. Try opening the file. (if it's not a valid file, print error message)
  // 2. Read the file, if the char is '\n' and the current lenght is not 0, clear it
  // 3. For the moment use an output file... (TODO: Do in-place...)
}
