use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if 2 < args.len() {
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
  } else if 0 < args.len() {
    let executable = &args[0];
    println!("Usage: {} filename pattern", executable);
  } else {
    println!("Usage: minigrep filename pattern");
  }
}

