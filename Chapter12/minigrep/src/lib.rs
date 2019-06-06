use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  println!("With text:\n{}", contents);
  Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    match args.len() {
      0 => Err("not enough arguments, missing executable"),
      1 | 2 => Err("not enough arguments"),
      3 | _ => Ok( Config { query: args[1].clone(), filename: args[2].clone() } ),
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn success() {
    assert!(true);
  }
}

