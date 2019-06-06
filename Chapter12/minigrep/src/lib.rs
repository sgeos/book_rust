use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    match args.len() {
      0 => Err("not enough arguments, missing executable"),
      1 | 2 => Err("not enough arguments"),
      3 | _ => {
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok( Config { query, filename, case_sensitive } )
      },
    }
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  let search_result = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in search_result {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
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
  fn search_case_sensitive_ok() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }

  #[test]
  fn search_case_insensitive_ok() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}

