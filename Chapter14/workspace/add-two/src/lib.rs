pub fn add_two(x: i32) -> i32 {
  x + 2
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn add_two_ok() {
    assert_eq!(4, add_two(2));
  }
}

