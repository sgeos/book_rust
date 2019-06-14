use std::fmt;

pub enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}

impl<T> List<T> {
  pub fn iter(&self) -> ListIterator<T> {
    ListIterator { node: Some(self) }
  }
}

pub struct ListIterator<'a, T> {
  node: Option<&'a List<T>>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.node {
      Some(List::Cons(value, next)) => {
        self.node = Some(next);
        Some(value)
      },
      Some(List::Nil) => {
        self.node = None;
        None
      },
      None => None,
    }
  }
}

impl<T: std::fmt::Display> fmt::Display for List<T> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    if let List::Cons(value, next) = self {
      let mut result = write!(formatter, "{}", value);
      for value in next.iter() {
        result = write!(formatter, ", {}", value);
      }
      result
    } else {
      write!(formatter, "(empty)")
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn success() {
    assert!(true);
  }

  #[test]
  fn print_list_values_ok() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{}", list);
  }

  #[test]
  fn print_list_nil_ok() {
    let list = List::<i32>::Nil;
    println!("{}", list);
  }

  #[test]
  fn iterate_list_values_ok() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    let mut iter = list.iter();
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
  }

  #[test]
  fn iterate_list_nil_ok() {
    let list = List::<i32>::Nil;
    let mut iter = list.iter();
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
  }
}

