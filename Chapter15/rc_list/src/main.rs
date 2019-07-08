extern crate rc_list as lib;
use std::rc::Rc;

fn main() {
  let list_a = Rc::new(lib::List::Cons(5, Rc::new(lib::List::Cons(10, Rc::new(lib::List::Nil)))));
  println!("List A created: {:?}! [References to List A; {}]", list_a, Rc::strong_count(&list_a));

  let list_b = lib::List::Cons(3, Rc::clone(&list_a));
  println!("List B created: {:?}! [References to List A; {}]", list_b, Rc::strong_count(&list_a));

  {
    let list_c = lib::List::Cons(4, Rc::clone(&list_a));
    println!("List C created: {:?}! [References to List A; {}]", list_c, Rc::strong_count(&list_a));
  }
  println!("List C goes out of scope! [References to List A; {}]", Rc::strong_count(&list_a));
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn main_ok() {
    main();
  }
}

