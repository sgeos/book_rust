pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}

pub trait IteratorGeneric<T> {
  fn next_generic(&mut self) -> Option<T>;
}

pub struct Counter {
  count: u32,
}

impl Counter {
  pub fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;
    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

impl IteratorGeneric<u32> for Counter {
  fn next_generic(&mut self) -> Option<u32> {
    self.count += 1;
    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn counter_iterator_ok() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
  }

  #[test]
  fn counter_iterator_generic_u32_ok() {
    let mut counter = Counter::new();

    assert_eq!(counter.next_generic(), Some(1));
    assert_eq!(counter.next_generic(), Some(2));
    assert_eq!(counter.next_generic(), Some(3));
    assert_eq!(counter.next_generic(), Some(4));
    assert_eq!(counter.next_generic(), Some(5));
    assert_eq!(counter.next_generic(), None);
  }
}

