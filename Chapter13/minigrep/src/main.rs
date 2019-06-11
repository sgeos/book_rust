extern crate minigrep;

use minigrep::Config;
use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
  let config = Config::new(env::args()).unwrap_or_else(|(executable, error)| {
    eprintln!("Application Error: {}", error);
    eprintln!("Usage: {} PATTERN FILENAME", executable);
    process::exit(1);
  });;
  if let Err(error) = minigrep::run(config) {
    eprintln!("Application Error: {}", error);
    process::exit(1);
  }
  Ok(())
}

