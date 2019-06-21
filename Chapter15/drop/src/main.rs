extern crate drop as lib;
use lib::CustomSmartPointer;

fn main() {
  let c = CustomSmartPointer { data: String::from("some data") };
  println!("CustomSmartPointer created.");
  drop(c);
  println!("CustomSmartPointer dropped before the end of main.");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_ok() {
    main();
  }
}

