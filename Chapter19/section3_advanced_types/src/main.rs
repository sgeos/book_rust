use std::fmt;
use std::io;
use std::io::Write;

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

type Kilometers = i32;

fn main_a() {
  let x: i32 = 5;
  let y: Kilometers = 5;

  println!("x + y = {}", x + y);
}

fn takes_long_type_b(f: Box<dyn Fn() + Send + 'static>) {
  print!("Callback B: ");
  f();
}

fn returns_long_type_b() -> Box<dyn Fn() + Send + 'static> {
  Box::new(|| println!("Hi, B!"))
}

fn main_b() {
  let callback: Box<dyn Fn() + Send + 'static> = returns_long_type_b();
  takes_long_type_b(callback);
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type_c(f: Thunk) {
  print!("Callback C: ");
  f();
}

fn returns_long_type_c() -> Thunk {
  Box::new(|| println!("Hi, C!"))
}

fn main_c() {
  let callback: Thunk = returns_long_type_c();
  takes_long_type_c(callback);
}

// pub trait WriteD {
//   fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//   fn flush(&mut self) -> Result<(), Error>;
// 
//   fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//   fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait WriteD {
  fn write(&mut self, buf: &[u8]) -> Result<usize>;
  fn flush(&mut self) -> Result<()>;

  fn write_all(&mut self, buf: &[u8]) -> Result<()>;
  fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn _bar() -> ! {
  print!("forever ");
  loop {
    // never returns
    print!("and ever ");
  }
}

fn main_d() {
  let guess: u32 = loop {
    print!("Guess a number: ");
    io::stdout().flush().expect("Failed to flush to stdout.");
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess = "0".to_string();
    match guess.trim().parse() {
      Ok(num) => break num,
      Err(_) => continue,
    }
  };
  println!("You guessed: {}", guess);
}

// impl<T> Option<T> {
//   pub fn unwrap(self) -> T {
//     match self {
//       Some(val) => val,
//       None => panic!("called `Option::unwrap()` on a `None` value"),
//     }
//   }
// }

fn generic_1<T>(t: T) -> T {
  println!("Generic 1.");
  t
}

fn generic_2<T: Sized>(t: T) -> T {
  println!("Generic 2.");
  t
}

fn generic_3<T: ?Sized>(t: &T) -> &T {
  println!("Generic 3.");
  &t
}

fn main_e() {
  let a = generic_1(5);
  let b = generic_2(a + 1);
  let c = generic_3(&b);
  println!("a: {}, b: {}, c: {}", a, b, c);
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

