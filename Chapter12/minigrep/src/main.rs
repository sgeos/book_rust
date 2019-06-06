extern crate minigrep;

use minigrep::Config;
use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect::<Vec<_>>();
  //let args: Vec<String> = Vec::new();
  let config = Config::new(&args).unwrap_or_else(|error| {
    println!("Application Error: {}", error);
    let executable = match args.len() {
      0 => "minigrep",
      1 | _ => args[0].as_str(),
    };
    println!("Usage: {} PATTERN FILENAME", executable);
    process::exit(1);
  });;
  println!("Searching for \"{}\" in file \"{}\".", config.query, config.filename);
  if let Err(error) = minigrep::run(config) {
    println!("Application Error: {}", error);
    process::exit(1);
  }
  Ok(())
}

