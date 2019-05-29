// Usage example:
// cargo run -- -- All you base are belong to us You have no change to suruve

extern crate clap;
use clap::{Arg, App};

fn main() {
  let matches = App::new("Pig latin converter.")
    .version("1.0.0")
    .author("Brendan A. R. Sechter <sgeos [at] hotmail [dot] com>")
    .about("Converts text to pig latin.")
    .arg(Arg::with_name("VALUES")
       .multiple(true)
       .last(true))
    .get_matches();

  let value_list = matches.values_of("VALUES").map(|vals| vals.collect::<Vec<_>>()).unwrap_or(Vec::new());
  for value in value_list.iter() {
    print!("{} ", to_pig_latin(value));
  }
  println!("");
}

fn is_vowel(c: char) -> bool {
  match c {
    'A' | 'E' | 'I' | 'O' | 'U' => true,
    'a' | 'e' | 'i' | 'o' | 'u' => true,
    _ => false,
  }
}

fn to_pig_latin(word: &str) -> String {
  let mut word = String::from(word);
  let mut first_character = match word.chars().next() {
    Some(c) => c,
    None => return word,
  };
  let capitalize = first_character.is_uppercase();
  first_character = first_character.to_lowercase().next().unwrap();
  word = match is_vowel(first_character){
    true => format!("{}-hay", word),
    false => format!("{}-{}ay", &word[1..], first_character),
  };
  if capitalize {
    let mut character_list: Vec<char> = word.chars().collect();
    character_list[0] = character_list[0].to_uppercase().nth(0).unwrap();
    word = character_list.into_iter().collect();
  }
  word
}

