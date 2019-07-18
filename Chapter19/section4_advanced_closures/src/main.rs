use std::fmt;

fn main() {
  let variation = 0;

  match variation {
    1 => wrap_function("Main A".to_string(), main_a),
    2 => wrap_function("Main B".to_string(), main_b),
    3 => wrap_function("Main C".to_string(), main_c),
    4 => wrap_function("Main D".to_string(), main_d),
    5 => wrap_function("Main E".to_string(), main_e),
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
  wrap_function("Main C".to_string(), main_c);
  wrap_function("Main D".to_string(), main_d);
  wrap_function("Main E".to_string(), main_e);
}

fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

fn main_a() {
  let answer = do_twice(add_one, 5);

  println!("The answer is: {}", answer);
}

fn main_b() {
  let list_of_numbers = vec![1, 2, 3];
  let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();
  println!("List B: {:?}", list_of_strings);
}

fn main_c() {
  let list_of_numbers = vec![1, 2, 3];
  let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();
  println!("List C: {:?}", list_of_strings);
}

enum Status {
  Value(u32),
  _Stop,
}

impl fmt::Debug for Status {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Status::Value(n) =>
        write!(f, "Value({})", n),
      Status::_Stop =>
        write!(f, "Stop"),
    }
  }
}

fn main_d() {
  let list_of_statuses: Vec<Status> =
    (0u32..20)
    .map(Status::Value)
    .collect();
  println!("List D: {:?}", list_of_statuses);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

fn main_e() {
  let callback = returns_closure();
  let result = callback(8);
  println!("Result E: {}", result);
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

  #[test]
  fn main_c_ok() {
    main_c();
  }

  #[test]
  fn main_d_ok() {
    main_d();
  }

  #[test]
  fn main_e_ok() {
    main_e();
  }
}

