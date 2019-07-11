extern crate section3_oop_design_patterns as lib;

use lib::solution_a::Post as PostA;
use lib::solution_b::Post as PostB;

fn main() {
  let variation = 0;

  match variation {
    1 => wrap_function("Main A".to_string(), main_a),
    2 => wrap_function("Main B".to_string(), main_b),
    _ => wrap_function("Main B".to_string(), main_b),
  };
}

fn wrap_function<F>(title: String, f: F) where F: FnOnce() {
  println!("--- {} START ---", title);
  f();
  println!("--- {} END ---", title);
}

fn main_a() {
  let mut post = PostA::new();

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.add_text("bad text");
  assert_eq!("", post.content());

  post.reject();
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
}

fn main_b() {
  let mut post = PostB::new();

  post.add_text("I ate a salad for lunch today");

  let post = post.request_review();

  let post = post.reject();

  let post = post.request_review();

  // this is a terrible solution that arguably works
  if let (Some(post), _) = post.approve() {
    if let (_, Some(post)) = post.approve() {
      assert_eq!("I ate a salad for lunch today", post.content());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_ok() {
    main();
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

