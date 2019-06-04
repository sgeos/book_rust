fn main() {
  let data = [
    String::from("السلام عليكم"),
    String::from("Dobrý den"),
    String::from("Hello"),
    String::from("שָׁלוֹם"),
    String::from("नमस्ते"),
    String::from("こんにちは"),
    String::from("안녕하세요"),
    String::from("你好"),
    String::from("Olá"),
    String::from("Здравствуйте"),
    String::from("Hola"),
  ];
  for (index, value) in data.iter().enumerate() {
    println!("{}: {}", index, value);
  }
  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s1 is {}", s1);
  println!("s2 is {}", s2);
  let mut s = String::from("lo");
  s.push('l');
  println!("s is {}", s);
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2;
  println!("s3 is {}", s3);
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("s is {}", s);
  println!("{}", &"Здравствуйте"[0..4]);
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }
  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}
