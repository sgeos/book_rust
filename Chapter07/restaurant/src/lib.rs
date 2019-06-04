mod front_of_house;
mod back_of_house;

pub fn eat_at_restaurant() {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();
  // Relative path
  front_of_house::hosting::seat_at_table();
  // Use every to avoid warnings
  front_of_house::serving::take_order();
  front_of_house::serving::serve_order();
  back_of_house::fix_incorrect_order();
  front_of_house::serving::take_payment();
}

pub use self::back_of_house::{Appetizer, Breakfast};

pub fn take_detailed_order() {
  let _order1 = Appetizer::Soup;
  let _order2 = Appetizer::Salad;
  // Order a breakfast in the summer with Rye toast
  let mut meal = Breakfast::summer("Rye");
  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);
  meal.serve();
  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}

