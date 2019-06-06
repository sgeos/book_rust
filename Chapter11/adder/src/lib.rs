// cargo test
// cargo test identity_with_side_effect_ok
// cargo test rect
// cargo test -- --test-threads=1 --nocapture
// cargo test expensive -- --ignored

#[derive(Debug)]
pub struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  pub fn can_hold(&self, target: &Rectangle) -> bool {
    target.width < self.width && target.height < self.height
  }
}

pub struct Guess {
  _value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 {
      panic!("Guess value must be greater than or equal to 1, got {}.", value);
    } else if 100 < value {
      panic!("Guess value must be less than or equal to 100, got {}.", value);
    }
    Guess { _value: value }
  }
}

pub fn identity_with_side_effect(value: i32) -> i32 {
  println!("Value: {}", value);
  value
}

pub fn add_two(value: i32) -> i32 {
  internal_adder(value, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
  a + b
}

pub fn greeting(name: &str) -> String {
  format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success() {
    assert!(true);
  }

  #[test]
  #[should_panic]
  fn failure() {
    assert!(false);
  }

  #[test]
  #[should_panic]
  fn will_panic() {
    panic!("This test will panic.");
  }

  #[test]
  fn result_test() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }

  #[test]
  fn rectangle_big_in_small_ok() {
    let big = Rectangle { width: 8, height: 7 };
    let small = Rectangle { width: 5, height: 1 };
    assert!(big.can_hold(&small));
  }

  #[test]
  fn rectangle_small_in_big_ng() {
    let big = Rectangle { width: 8, height: 7 };
    let small = Rectangle { width: 5, height: 1 };
    assert!(!small.can_hold(&big));
  }

  #[test]
  fn add_two_ok() {
    assert_eq!(4, add_two(2));
  }

  #[test]
  fn internal_adder_ok() {
    assert_eq!(15, internal_adder(6, 9));
  }

  #[test]
  fn greeting_contains_name() {
    let name = "Carol";
    let result = greeting(name);
    assert!(
      result.contains("Carol"),
      "The greeting \"{}\" did not contain the name \"{}\".", result, name
    );
  }

  #[test]
  #[should_panic(expected = "Guess value must be greater than or equal to 1")]
  fn guess_too_small() {
    Guess::new(-200);
  }

  #[test]
  #[should_panic(expected = "Guess value must be less than or equal to 100")]
  fn guess_too_large() {
    Guess::new(200);
  }

  #[test]
  fn identity_with_side_effect_ok() {
    let value = 456;
    assert_eq!(value, identity_with_side_effect(value));
  }

  #[test]
  #[ignore]
  fn expensive_test() {
    // code that takes an hour to run
  }
}

