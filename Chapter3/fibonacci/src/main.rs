// Usage example:
// cargo run -- -- 0 1 2 3 4 5 6 7 8 -9 -10

extern crate clap;
use clap::{Arg, App};

fn main() {
  let matches = App::new("Temperature Converter")
    .version("1.0.0")
    .author("Brendan A. R. Sechter <sgeos [at] hotmail [dot] com>")
    .about("Prints the nth Fibonacci number.")
    .arg(Arg::with_name("VALUES")
       .multiple(true)
       .last(true))
    .get_matches();

  let values = matches.values_of("VALUES").map(|vals| vals.collect::<Vec<_>>()).unwrap_or(Vec::new());
  for value in values.iter() {
    let n = match value.trim().parse() {
      Ok(value) => value,
      Err(_) => continue,
    };
    print!("{} ", fibonacci(n));
  }
  println!("");
}

fn fibonacci(n: i64) -> i64 {
  if n < 0 {
    -fibonacci(-n)
  } else {
    match n {
      0 => 0,
      1 => 1,
      n => fibonacci(n - 1) + fibonacci(n - 2),
    }
  }
}

