use std::{env, fs};

fn main() {
  let args: Vec<String> = env::args().collect();

  if 2 < args.len() {
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for \"{}\" in file \"{}\".", query, filename);
    let contents = fs::read_to_string(filename)
      .expect("Something went wrong reading the file");
      // TODO: Properly handle error case when attempting to open file.
    println!("With text:\n{}", contents);
  } else if 0 < args.len() {
    let executable = &args[0];
    println!("Usage: {} pattern filename", executable);
  } else {
    println!("Usage: minigrep pattern filename");
  }
}

