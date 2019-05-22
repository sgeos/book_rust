// Usage example:
// cargo run -- -f k -t f -- 0 25 88.9 100 1000
// Temperature types: c f k

extern crate clap;
use clap::{Arg, App};

const CELSIUS: &str = "Celsius";
const KELVIN: &str = "Kelvin";
const FAHRENHEIT: &str = "Fahrenheit";
const UNKNOWN_FORMAT: &str = "unknown format";
static FORMAT_LIST: &'static [&str] = &[CELSIUS, KELVIN, FAHRENHEIT];

fn main() {
  let matches = App::new("Temperature Converter")
    .version("1.0.0")
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
    .arg(Arg::with_name("ROUND")
       .help("Round output flag")
       .short("r")
       .long("round"))
    .arg(Arg::with_name("VERBOSE")
       .help("Verbose output flag")
       .short("v")
       .long("verbose"))
    .arg(Arg::with_name("VALUES")
       .multiple(true)
       .last(true))
    .get_matches();

  let from = get_format(matches.value_of("FROM"));
  let to = get_format(matches.value_of("TO"));
  let values = matches.values_of("VALUES").map(|vals| vals.collect::<Vec<_>>()).unwrap_or(Vec::new());
  let round = matches.occurrences_of("ROUND");
  let verbose = matches.occurrences_of("VERBOSE");
  if 0 < verbose {
    println!("Convering {} temperature values from {} to {}.", values.len(), from, to);
  };
  for value in values.iter() {
    let input = match value.trim().parse() {
      Ok(value) => value,
      Err(_) => continue,
    };
    let from = t_to_c(from, input);
    let to = c_to_t(to, from);
    if 0 < round {
      print!("{:.3} ", to);
    } else {
      print!("{} ", to);
    };
  }
  println!("");
}

fn get_format(input: Option<&str>) -> &str {
  let input = input.unwrap_or(UNKNOWN_FORMAT).trim().to_lowercase();
  for format in FORMAT_LIST {
    if format.to_lowercase().starts_with(&input) {
      return format
    }
  }
  return UNKNOWN_FORMAT
}

fn t_to_c(t_type: &str, t: f64) -> f64 {
  match t_type {
    CELSIUS => t,
    KELVIN => t - 273.15,
    FAHRENHEIT => (t - 32.0) * 5.0 / 9.0,
    _ => std::f64::NAN,
  }
}

fn c_to_t(t_type: &str, t: f64) -> f64 {
  match t_type {
    CELSIUS => t,
    KELVIN => t + 273.15,
    FAHRENHEIT => t *  9.0 / 5.0 + 32.0,
    _ => std::f64::NAN,
  }
}

