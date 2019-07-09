use std::sync::{Arc, Mutex};
use std::thread;

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
  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }

  println!("m = {:?}", m);
}

fn main_b() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(
      move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
      }
    );
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}

fn main_c() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for index in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(
      move || {
        let mut num = counter.lock().unwrap();

        println!("Update: {}", index);
        *num = index;
      }
    );
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}

fn main_d() {
  let counter = Arc::new(Mutex::new(vec![]));
  let mut handles = vec![];

  for index in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(
      move || {
        let mut list = counter.lock().unwrap();

        list.push(index);
      }
    );
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {:?}", *counter.lock().unwrap());
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

