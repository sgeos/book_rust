fn print_integer(key: &str, value: i32) {
  println!("The value of {} is: {}", key, value);
}

fn print_character(key: &str, value: char) {
  println!("The value of {} is: {}", key, value);
}

fn main() {
  let mut x = 5;
  print_integer("x", x);
  x = 6;
  print_integer("x", x);
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  print_integer("x", x);
  let c = 'z';
  print_character("c", c);
  let z = 'ℤ';
  print_character("z", z);
  let heart_eyed_cat = '😻';
  print_character("heart_eyed_cat", heart_eyed_cat);
}
