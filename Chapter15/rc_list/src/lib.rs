use std::fmt;
use std::rc::Rc;

pub enum List<T> {
  Cons(T, Rc<List<T>>),
  Nil,
}

impl<T> List<T> {
  pub fn iter(&self) -> ListIterator<T> {
    ListIterator { node: self }
  }
}

pub struct ListIterator<'a, T> {
  node: &'a List<T>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.node {
      List::Cons(value, next) => {
        self.node = next;
        Some(value)
      },
      List::Nil => None,
    }
  }
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    if let List::Cons(value, next) = self {
      write!(formatter, "{:?}", value)?;
      for value in next.iter() {
        write!(formatter, ", {:?}", value)?;
      }
    } else {
      write!(formatter, "(empty)")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn print_list_values_ok() {
    let list = List::Cons(1, Rc::new(List::Cons(2, Rc::new(List::Cons(3, Rc::new(List::Nil))))));
    println!("{:?}", list);
  }

  #[test]
  fn print_list_nil_ok() {
    let list = List::<i32>::Nil;
    println!("{:?}", list);
  }

  #[test]
  fn iterate_list_values_ok() {
    let list = List::Cons(1, Rc::new(List::Cons(2, Rc::new(List::Cons(3, Rc::new(List::Nil))))));
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

