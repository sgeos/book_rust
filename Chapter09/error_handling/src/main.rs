use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

const DEFAULT_USERNAME: &str = "(none)";

fn main() -> Result<(), Box<dyn Error>> {
  let filename = "data.sav";
  {
    let _file = File::open(filename)?;
  }
  open_file(filename);
  let username = read_username_from_file(filename).unwrap_or(DEFAULT_USERNAME.to_string());
  println!("Normal Username: {}", username);
  let username_simple = read_username_from_file_simple(filename).unwrap_or(DEFAULT_USERNAME.to_string());
  println!("Simple Username: {}", username_simple);
  Ok(())
}

fn open_file(filename: &str) {
  match File::open(filename) {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create(filename) {
        Ok(file) => file,
        Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
      },
      other_error => panic!("There was a problem opening the file: {:?}", other_error),
    },
  };
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
  let mut f = File::open(filename)?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

fn read_username_from_file_simple(filename: &str) -> Result<String, io::Error> {
  fs::read_to_string(filename)
}

