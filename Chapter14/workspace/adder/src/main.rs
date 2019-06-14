extern crate add_one;
extern crate add_two;
extern crate rand;
use rand::Rng;

fn main() {
  let min = 1;
  let max = 100;
  let number = rand::thread_rng().gen_range(min, max + 1);
  let result = number;
  println!("Hello, world!");
  let (number, result) = (result, add_one::add_one(result));
  println!("{} plus one is {}!", number, result);
  let (number, result) = (result, add_two::add_two(result));
  println!("{} plus two is {}!", number, result);
  let (number, result) = (result, add_one::add_rand(result));
  println!("{} plus a randome value is {}!", number, result);
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn main_ok() {
    main();
  }
}

