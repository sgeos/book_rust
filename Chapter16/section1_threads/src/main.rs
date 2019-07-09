use std::thread;
use std::time::Duration;

fn main() {
  let variation = 0;

  match variation {
    1 => main_a(),
    2 => main_b(),
    3 => main_c(),
    4 => main_d(),
    _ => main_d(),
  }
}

fn main_a() {
  thread::spawn(
    || {
      for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
      }
    }
  );

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
}

fn main_b() {
  let handle = thread::spawn(
    || {
      for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
      }
    }
  );

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();
}

fn main_c() {
  let handle = thread::spawn(
    || {
      for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
      }
    }
  );

  handle.join().unwrap();

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
}

fn main_d() {
  let v = vec![1, 2, 3];

  let handle = thread::spawn(
    move || {
      println!("Here's a vector: {:?}", v);
    }
  );

  handle.join().unwrap();
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

