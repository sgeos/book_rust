extern crate minigrep;

use minigrep::Config;
use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect::<Vec<_>>();
  let config = Config::new(&args).unwrap_or_else(|error| {
    eprintln!("Application Error: {}", error);
    let executable = match args.len() {
      0 => "minigrep",
      1 | _ => args[0].as_str(),
    };
    eprintln!("Usage: {} PATTERN FILENAME", executable);
    process::exit(1);
  });;
  if let Err(error) = minigrep::run(config) {
    eprintln!("Application Error: {}", error);
    process::exit(1);
  }
  Ok(())
}

