enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

fn main() {
  let mut v: Vec<i32> = Vec::new();
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);
  let third: &i32 = &v[2];
  println!("The third element is {}", third);
  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  }

  let mut row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
  for r in &mut row {
    match r {
      SpreadsheetCell::Int(v) => *v += 50,
      SpreadsheetCell::Text(v) => *v = format!("[String] {}", v),
      SpreadsheetCell::Float(v) => *v += 50.0,
    }
  }
  for r in row {
    match r {
      SpreadsheetCell::Int(v) => println!("Hello, Int({})!", v),
      SpreadsheetCell::Text(v) => println!("Hello, Text({})!", v),
      SpreadsheetCell::Float(v) => println!("Hello, Float({})!", v),
    }
  }
}

