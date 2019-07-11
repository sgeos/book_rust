extern crate section2_oop_traits as lib;
use lib::{Button, Draw, Screen};

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // code to actually draw a select box
    println!("Draw SelectBox {{ width: {}, height: {}, options: {:?} }}", self.width, self.height, self.options);
  }
}

fn main() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No")
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };

  screen.run();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_ok() {
    main();
  }
}

