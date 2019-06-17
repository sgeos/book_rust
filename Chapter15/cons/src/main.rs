extern crate cons as lib;

fn main() {
  let list_a = lib::List::Cons(1, Box::new(lib::List::Cons(2, Box::new(lib::List::Cons(3, Box::new(lib::List::Nil))))));
  println!("List A: {:?}!", list_a);
  let list_b: lib::List<i32> = lib::List::Nil;
  println!("List B: {:?}!", list_b);
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn main_ok() {
    main();
  }
}

