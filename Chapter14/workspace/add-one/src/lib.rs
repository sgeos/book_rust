extern crate rand;
use rand::Rng;

pub fn add_one(x: i32) -> i32 {
  x + 1
}

pub fn add_rand(x: i32) -> i32 {
  let min = 1;
  let max = 100;
  let number = rand::thread_rng().gen_range(min, max + 1);
  x + number
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn add_one_ok() {
    assert_eq!(2, add_one(1));
  }

  #[test]
  fn add_rand_ok() {
    let x = 1;
    assert!(x < add_rand(x));
  }
}

