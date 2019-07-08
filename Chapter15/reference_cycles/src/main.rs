extern crate reference_cycles as lib;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
  let leaf = Rc::new(lib::Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf),
  );

  {
    let branch = Rc::new(lib::Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
      "branch strong = {}, weak = {}",
      Rc::strong_count(&branch),
      Rc::weak_count(&branch),
    );

    println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
    );
  }

  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf),
  );
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn main_ok() {
    main();
  }
}

