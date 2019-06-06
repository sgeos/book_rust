use std::error::Error;
use std::fs;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  for line in search(&config.query, &contents) {
    println!("{}", line);
  }
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success() {
    assert!(true);
  }

  #[test]
  fn parse_args_ok() {
    let input: Vec<String> = vec!["minigrep".to_string(), "pattern".to_string(), "filename".to_string()];
    Config::new(&input).unwrap();
  }

  #[test]
  #[should_panic(expected = "not enough arguments")]
  fn parse_args_panic_not_enough() {
    let input: Vec<String> = vec!["minigrep".to_string(), "not enough".to_string()];
    Config::new(&input).unwrap();
  }

  #[test]
  #[should_panic(expected = "not enough arguments, missing executable")]
  fn parse_args_panic_missing_executable() {
    let input: Vec<String> = vec![];
    Config::new(&input).unwrap();
  }

  #[test]
  fn run_ok() {
    let input: Vec<String> = vec!["minigrep".to_string(), ".".to_string(), ".gitignore".to_string()];
    let config = Config::new(&input).unwrap();
    run(config).unwrap();
  }

  #[test]
  #[should_panic(expected = "No such file or directory")]
  fn run_panic_bad_file() {
    let input: Vec<String> = vec!["minigrep".to_string(), "pattern".to_string(), "bad\x15filename".to_string()];
    let config = Config::new(&input).unwrap();
    run(config).unwrap();
  }

  #[test]
  fn search_ok_one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }
}

