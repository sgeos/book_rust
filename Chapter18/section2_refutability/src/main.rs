fn main() {
  let some_option_value = Some(42);

  if let Some(x) = some_option_value {
    println!("{}", x);
  }
}

