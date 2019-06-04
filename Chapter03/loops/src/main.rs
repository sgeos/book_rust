fn main() {
  finite_loop(10);
  conditional_loop(3);
  let data = [10, 20, 30, 40, 50];
  print_array_contents(&data);
  print_array_contents(&[60, 70, 80, 90]);
  iterative_launch(3);
}

fn _infinite_loop() {
  loop {
    println!("Again!");
  }
}

fn finite_loop(max: i32) {
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if max <= counter {
      break counter * 2;
    }
  };
  println!("The result is {}.", result)
}

fn conditional_loop(max: i32) {
  let mut counter = max;
  while 0 < counter {
    println!("{}!", counter);
    counter -= 1;
  }
  println!("LIFTOFF!");
}

fn print_array_contents(array: &[i32]) {
  for (index, value) in array.iter().enumerate() {
    println!("The value of array index {} is: {}", index, value);
  }
}

fn iterative_launch(max: i32) {
  for counter in (1..max + 1).rev() {
    println!("{}!", counter);
  }
  println!("LIFTOFF!");
}

