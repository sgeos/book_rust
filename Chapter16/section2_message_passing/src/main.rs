use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
  let (tx, rx) = mpsc::channel();

  thread::spawn(
    move || {
      let val = String::from("hi");
      tx.send(val).unwrap();
    }
  );

  let received = rx.recv().unwrap();
  println!("RX: {}", received);
}

fn main_b() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(
    move || {
      let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
      ];

      for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
      }
    }
  );

  for received in rx {
    println!("RX: {}", received);
  }
}

fn main_c() {
  let (tx0, rx0) = mpsc::channel();
  let tx1 = mpsc::Sender::clone(&tx0);

  thread::spawn(
    move || {
      let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
      ];

      for val in vals {
        tx0.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
      }
    }
  );

  thread::spawn(
    move || {
      let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
      ];

      for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
      }
    }
  );

  for received in rx0 {
    println!("RX: {}", received);
  }
}

fn main_d() {
  let (tx, rx) = mpsc::channel();
  let millisecond_delay = 1000;

  spawn_message_thread(
    mpsc::Sender::clone(&tx),
    millisecond_delay + 1,
    vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ]
  );

  spawn_message_thread(
    tx,
    millisecond_delay - 1,
    vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ]
  );

  for received in rx {
    println!("RX: {}", received);
  }
}

fn spawn_message_thread(tx: mpsc::Sender<String>, delay: u64, vals: Vec<String>) {
  thread::spawn(
    move || {
      for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_millis(delay));
      }
    }
  );
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

