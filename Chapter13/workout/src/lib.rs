extern crate rand;

use rand::Rng;
use std::error::Error;
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
  calculation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
  where T: Fn(u32) -> u32
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      },
    }
  }
}

pub struct Config {
  pub intensity: u32,
  pub variation: u32,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    let mut intensity = 10;
    for arg in args.iter().skip(1) {
      intensity = match arg.trim().parse() {
        Ok(value) => value,
        _ => intensity,
      };
    }
    let (min, max) = (1, 10);
    let variation = rand::thread_rng().gen_range(min, max + 1);;
    Ok( Config { intensity, variation } )
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  generate_workout(
    config.intensity,
    config.variation
  );
  Ok(())
}

fn generate_workout(intensity: u32, variation: u32) {
  let mut expensive_result = Cacher::new(simulated_expensive_calculation);
  let low_intensity = intensity < 25;
  if low_intensity {
    println!(
      "Today, do {} pushups!",
      expensive_result.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_result.value(intensity)
    );
  } else {
    if 3 == variation {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_result.value(intensity)
      );
    }
  }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
  println!("Calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  let spread = 0.3;
  let min: u32 = (intensity as f64 * (1.0 - spread)) as u32;
  let max: u32 = (intensity as f64 * (1.0 + spread)) as u32;
  rand::thread_rng().gen_range(min, max + 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success() {
    assert!(true);
  }

  #[test]
  fn config_new_ok() {
    let args = vec![];
    Config::new(&args).unwrap();
  }

  #[test]
  fn run_ok() {
    let args = vec!["100".to_string()];
    let config = Config::new(&args).unwrap();
    run(&config).unwrap()
  }

  #[test]
  fn workout_low_intensity_ok() {
    let (intensity, variation) = (0, 0);
    generate_workout(intensity, variation);
  }

  #[test]
  fn workout_high_intensity_ok() {
    let (intensity, variation) = (100, 100);
    generate_workout(intensity, variation);
  }

  #[test]
  fn workout_rest_ok() {
    let (intensity, variation) = (100, 3);
    generate_workout(intensity, variation);
  }

  #[test]
  fn simulated_expensive_calculation_zero_intensity_ok() {
    let intensity = 0;
    simulated_expensive_calculation(intensity);
  }
}

