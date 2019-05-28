pub enum Appetizer {
  Soup,
  Salad,
}

pub struct Breakfast {
  pub toast: String,
  seasonal_fruit: String,
}

impl Breakfast {
  pub fn summer(toast: &str) -> Breakfast {
    Breakfast {
      toast: String::from(toast),
      seasonal_fruit: String::from("peaches"),
    }
  }
  pub fn serve(&self) {
    println!("Here is some {} toast with {}.", self.toast, self.seasonal_fruit);
  }
}

pub fn fix_incorrect_order() {
  cook_order();
  super::front_of_house::serving::serve_order();
}

fn cook_order() { }

