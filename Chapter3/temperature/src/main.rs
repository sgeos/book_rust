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

  let from = matches.value_of("FROM").unwrap();
  let to = matches.value_of("TO").unwrap();
  let values = matches.values_of("VALUES").map(|vals| vals.collect::<Vec<_>>()).unwrap();
  for value in values.iter() {
    let input = match value.trim().parse() {
      Ok(value) => value,
      Err(_) => continue,
    };
    let from = t_to_c(from, input);
    let to = c_to_t(to, from);
    println!("{}", to);
  }
}

fn t_to_c(t_type: &str, t: f64) -> f64 {
  match t_type {
    "f" => (t - 32.0) * 5.0 / 9.0,
    "k" => t - 273.15,
    "c" => t,
    _ => std::f64::NAN,
  }
}

fn c_to_t(t_type: &str, t: f64) -> f64 {
  match t_type {
    "f" => t *  9.0 / 5.0 + 32.0,
    "k" => t + 273.15,
    "c" => t,
    _ => std::f64::NAN,
  }
}

