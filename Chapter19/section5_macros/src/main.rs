// For more information on macros, see the following links.
//   https://danielkeep.github.io/tlborm/book/index.html
//   https://doc.rust-lang.org/reference/macros.html
//   https://docs.rs/quote/0.6.13/quote/
//   https://doc.rust-lang.org/book/ch19-06-macros.html

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

fn main() {
  let variation = 0;

  match variation {
    1 => wrap_function("Main A".to_string(), main_a),
    2 => wrap_function("Main B".to_string(), main_b),
    _ => wrap_function("Main All".to_string(), main_all),
  };
}

fn wrap_function<F>(title: String, f: F) where F: FnOnce() {
  println!("--- {} START ---", title);
  f();
  println!("--- {} END ---", title);
}

fn main_all() {
  wrap_function("Main A".to_string(), main_a);
  wrap_function("Main B".to_string(), main_b);
}

struct PancakesA;

impl HelloMacro for PancakesA {
  fn hello_macro() {
    println!("Hello, Macro! My name is PancakesA!");
  }
}

fn main_a() {
  PancakesA::hello_macro();
}

#[derive(HelloMacro)]
struct PancakesB;

fn main_b() {
  PancakesB::hello_macro();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_ok() {
    main();
  }

  #[test]
  fn main_all_ok() {
    main_all();
  }

  #[test]
  fn main_a_ok() {
    main_a();
  }

  #[test]
  fn main_b_ok() {
    main_b();
  }
}

