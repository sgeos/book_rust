use std::fmt::Display;

fn main() {
  let string1 = String::from("abcd");
  let result;
  {
    let string2 = "xyz";
    result = longest_with_announcement(string1.as_str(), string2, "Calculating the longest string...");
    println!("The longest string is \"{}\".", result);
  }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
  where T: Display
{
  println!("Announcement! {}", announcement);
  longest(x, y)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if y.len() < x.len() {
    x
  } else {
    y
  }
}


