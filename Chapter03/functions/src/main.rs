fn main() {
  println!("Hello, world!");
  another_function();
  print_integer("the literal constant", 5);
  let x = five();
  let y = plus_one(x);
  yet_another_function((x, y));
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

fn another_function() {
  println!("Another function.");
}

fn yet_another_function(tuple: (i32, i32)) {
  let (x, y) = tuple;
  print_integer("x", x);
  print_integer("y", y);
}

fn print_integer(key: &str, value: i32) {
  println!("The value of {} is: {}", key, value);
}

