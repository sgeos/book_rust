// Usage example:
// cargo run -- -f k -t f -- 0 25 88.9 100 1000
// Temperature types: c f k

extern crate clap;
use clap::{Arg, App};

fn main() {
  let matches = App::new("Temperature Converter")
    .version("1.0")
    .author("Brendan A. R. Sechter <sgeos [at] hotmail [dot] com>")
    .about("Converts temperatures.")
    .arg(Arg::with_name("FROM")
       .help("Temperature to convert from")
       .short("f")
       .long("from")
       .default_value("c"))
    .arg(Arg::with_name("TO")
       .help("Temperature to convert to")
       .short("t")
       .long("to")
       .default_value("c"))
    .arg(Arg::with_name("VALUES")
       .multiple(true)
       .last(true))
    .get_matches();

  let from = first_lowercase_character(matches.value_of("FROM"), '_');
  let to = first_lowercase_character(matches.value_of("TO"), '_');
  let values = match  matches.values_of("VALUES").map(|vals| vals.collect::<Vec<_>>()) {
    Some(v) => v,
    None => Vec::new(),
  };
  for value in values.iter() {
    let input = match value.trim().parse() {
      Ok(value) => value,
      Err(_) => continue,
    };
    let from = t_to_c(from, input);
    let to = c_to_t(to, from);
    print!("{} ", to);
  }
  println!("");
}

fn first_lowercase_character(input: Option<&str>, default: char) -> char {
  match input {
    Some(s) => match s.to_lowercase().chars().next() {
      Some(c) => c,
      None => default,
    },
    None => default,
  }
}

fn t_to_c(t_type: char, t: f64) -> f64 {
  match t_type {
    'f' => (t - 32.0) * 5.0 / 9.0,
    'k' => t - 273.15,
    'c' => t,
    _ => std::f64::NAN,
  }
}

fn c_to_t(t_type: char, t: f64) -> f64 {
  match t_type {
    'f' => t *  9.0 / 5.0 + 32.0,
    'k' => t + 273.15,
    'c' => t,
    _ => std::f64::NAN,
  }
}

