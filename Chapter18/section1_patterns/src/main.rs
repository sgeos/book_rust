fn main() {
  let variation = 0;

  match variation {
    1 => wrap_function("Main A".to_string(), main_a),
    2 => wrap_function("Main B".to_string(), main_b),
    3 => wrap_function("Main C".to_string(), main_c),
    4 => wrap_function("Main D".to_string(), main_d),
    _ => wrap_function("Main D".to_string(), main_d),
  };
}

fn wrap_function<F>(title: String, f: F) where F: FnOnce() {
  println!("--- {} START ---", title);
  f();
  println!("--- {} END ---", title);
}

fn main_a() {
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background color");
    } else {
      println!("Using orange as the background color");
    }
  } else {
    println!("Using blue as the background color");
  }
}

fn main_b() {
  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  while let Some(top) = stack.pop() {
    println!("{}", top);
  }
}

fn main_c() {
  let v = vec!['a', 'b', 'c'];

  for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
  }
}

fn main_d() {
  let point = (3, 5);
  print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
  println!("Current location: ({}, {})", x, y);
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

  #[test]
  fn main_c_ok() {
    main_c();
  }

  #[test]
  fn main_d_ok() {
    main_d();
  }
}

