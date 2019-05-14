fn main() {
  evaluate_number(3);
  evaluate_number(7);
  let x = get_number(true);
  information_about_number(x);
  let x = get_number(false);
  information_about_number(x);
}

fn get_number(success: bool) -> i32 {
  let result = if success {
    6
  } else {
    5
  };
  result
}

fn evaluate_number(n: i32) {
  let limit = 5;
  println!("n = {}", n);
  println!("Condition n < {}.", limit);
  if n < limit {
    println!("Condition was true.");
  } else {
    println!("Condition was false.");
  }
}

fn information_about_number(n: i32) {
  println!("n = {}", n);
  if 0 == n % 4 {
    println!("Number is divisible by 4.");
  } else if 0 == n % 3 {
    println!("Number is divisible by 3.");
  } else if 0 == n % 2 {
    println!("Number is divisible by 2.");
  } else {
    println!("Number is not divisible by 4, 3, or 2.");
  }
}

