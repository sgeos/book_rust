extern crate workout as lib;

const EXECUTABLE: &'static str = env!("CARGO_PKG_NAME");

use std::env;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect::<Vec<_>>();
  let executable = match args.len() {
    0 => EXECUTABLE,
    1 | _ => args[0].as_str(),
  };
  let config = lib::Config::new(&args).unwrap_or_else(|error| {
    eprintln!("Application Error: {}", error);
    eprintln!("Usage: {}", executable);
    process::exit(1);
  });;
  if let Err(error) = lib::run(&config) {
    eprintln!("Application Error: {}", error);
    process::exit(1);
  }
  Ok(())
}

