use std::net::IpAddr;

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }
    Guess {
      value
    }
  }
  pub fn value(&self) -> i32 {
    self.value
  }
}

fn main() {
  let _home: IpAddr = "127.0.0.1".parse().unwrap();
  let _guess = Guess::new(55);
  let _guess = Guess::new(-55);
}
