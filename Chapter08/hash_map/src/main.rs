use std::collections::HashMap;

fn main() {
  let mut score_map = HashMap::new();
  score_map.insert(String::from("Blue"), 10);
  score_map.insert(String::from("Blue"), 25);
  score_map.insert(String::from("Yellow"), 50);
  score_map.entry(String::from("Yellow")).or_insert(0);
  score_map.entry(String::from("Red")).or_insert(0);
  print_scores(&score_map);

  let key_list = vec![
    String::from("Red"),
    String::from("Green"),
    String::from("Blue"),
    String::from("Yellow"),
  ];
  let value_list = vec![45, 30, 10, 50];
  let score_map: HashMap<_, _> = key_list.into_iter().zip(value_list.into_iter()).collect();
  print_scores(&score_map);

  let text = "hello world wonderful world";
  let mut word_map: HashMap<String, i32> = HashMap::new();
  for word in text.split_whitespace() {
    let count = word_map.entry(String::from(word)).or_insert(0);
    *count += 1;
  }
  print_scores(&word_map);
}

fn print_scores(score_map: &HashMap<String, i32>) {
  println!("---");
  for (key, value) in score_map {
    println!("{} Score: {}", key, value);
  }
}

