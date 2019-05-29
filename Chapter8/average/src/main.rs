// Usage example:
// cargo run -- -- 1 3 12 2 6 4 -1 -2 4 5.3 NaN 12 9 8 6

extern crate clap;
use clap::{Arg, App};
use std::collections::HashMap;
use std::cmp::Ordering::Equal;

fn main() {
  let matches = App::new("Mean, median and mode calculator.")
    .version("1.0.0")
    .author("Brendan A. R. Sechter <sgeos [at] hotmail [dot] com>")
    .about("Calculates the mean, median and mode of the supplied numbers.")
    .arg(Arg::with_name("VALUES")
       .multiple(true)
       .last(true))
    .get_matches();

  let command_line_value_list = matches
    .values_of("VALUES")
    .map(|vals| vals.collect::<Vec<_>>())
    .unwrap_or(Vec::new());
  let mut value_list: Vec<f64> = Vec::new();
  for value in command_line_value_list.iter() {
    match value.trim().parse::<f64>() {
      Ok(value) => value_list.push(value),
      Err(_) => (),
    }
  }
  println!("Input:  {}", printable_list(&value_list));
  sort_and_filter_f64_list(&mut value_list);
  println!("Values: {}", printable_list(&value_list));
  println!("Sum:    {}", value_list.iter().fold(0.0f64, |sum, value| sum + value));
  println!("Count:  {}", value_list.len());
  println!("Mean:   {}", mean(&value_list));
  println!("Median: {}", median(&value_list));
  println!("Mode:   {}", printable_list(&mode(&value_list)));
}

fn sort_and_filter_f64_list(value_list: &mut Vec<f64>) {
  *value_list = value_list
    .iter()
    .filter_map(|v| if v.is_nan() { None } else { Some(*v) } )
    .collect();
  value_list.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
}

fn printable_list(value_list: &Vec<f64>) -> String {
  value_list
    .iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>()
    .join(", ")
}

fn mean(value_list: &Vec<f64>) -> f64 {
  let length = value_list.len();
  if 0 == length {
    std::f64::NAN
  } else {
    let sum: f64 = value_list.iter().fold(0.0f64, |sum, value| sum + value);
    sum / length as f64
  }
}

fn median(value_list: &Vec<f64>) -> f64 {
  let length = value_list.len();
  match (length, length % 2) {
    (0, _) => std::f64::NAN,
    (_, 0) => (value_list[length / 2 - 1] + value_list[length / 2]) / 2.0,
    (_, _) => value_list[length / 2],
  }
}

fn mode(value_list: &Vec<f64>) -> Vec<f64> {
  let length = value_list.len();
  if 0 == length {
    vec![std::f64::NAN]
  } else {
    let mut max_count = 0;
    let mut map = HashMap::new();
    for value in value_list {
      let (_value, count) = map.entry(value.to_string()).or_insert((*value, 0));
      *count += 1;
      if max_count < *count {
        max_count = *count;
      }
    }
    let mut result = Vec::new();
    for (_key, (value, count)) in map {
      if max_count <= count {
        result.push(value);
      }
    }
    sort_and_filter_f64_list(&mut result);
    result
  }
}

