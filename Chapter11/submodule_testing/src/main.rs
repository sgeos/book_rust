mod submodule;

fn main() {
  submodule::message();
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success() {
    assert!(true);
  }

  #[test]
  fn main_ok() {
    main();
  }
}

