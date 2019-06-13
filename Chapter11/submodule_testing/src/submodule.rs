pub fn message() {
  println!("Hello, submodule!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn message_ok() {
    message();
  }
}
