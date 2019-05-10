extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  let min = 1;
  let max = 100;
  'main: loop {
    let secret_number = rand::thread_rng().gen_range(min, max + 1);
    // println!("The secret number is: {}", secret_number);
    println!("---");
    println!("Guess the number!");
    'game: loop {
      println!("Please input your guess.");
      let mut input_buffer = String::new();
      io::stdin().read_line(&mut input_buffer)
        .expect("Failed to read line.");
      let guess: u32 = match input_buffer.trim().parse() {
        Ok(value) => value,
        Err(_) => match input_buffer.to_lowercase().starts_with("q") {
          true => break 'main,
          _ => continue 'game,
        }
      };
      println!("You guessed: {}", guess);
      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => {
          println!("You win!\n");
          break 'game;
        }
        Ordering::Greater => println!("Too big!"),
      }
    }
  }
}

