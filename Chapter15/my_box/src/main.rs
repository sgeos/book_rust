extern crate my_box as lib;
use lib::MyBox;

fn main() {
  let message = MyBox::new(String::from("Rust"));
  hello(&message);
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_ok() {
    main();
  }
}

