// Usage example:
// cargo run

static NTH_STRING: &'static [&str] = &[
  "zeroth", "first", "second", "third", "fourth",
  "fifth", "sixth", "seventh", "eighth", "ninth",
  "tenth", "eleventh", "twelfth"
];

fn main() {
  for n in 1..12+1 {
    verse(n);
  }
}

fn verse(n: usize) {
  if 0 < n && n < 13 {
    println!("On the {} day of Christmas", NTH_STRING[n]);
    println!("My true love gave to me");
  }
  if 12 <= n {
    println!("Twelve drummers drumming,");
  }
  if 11 <= n {
    println!("Eleven pipers piping,");
  }
  if 10 <= n {
    println!("Ten lords a-leaping,");
  }
  if 9 <= n {
    println!("Nine ladies dancing,");
  }
  if 8 <= n {
    println!("Eight maids a-milking,");
  }
  if 7 <= n {
    println!("Seven swans a-swimming,");
  }
  if 6 <= n {
    println!("Six geese a-laying,");
  }
  if 5 <= n {
    println!("Five gold rings,");
  }
  if 4 <= n {
    println!("Four calling birds,");
  }
  if 3 <= n {
    println!("Three French hens,");
  }
  if 2 <= n {
    println!("Two turtle doves,");
  }
  if 1 < n {
    println!("And a partridge in a pear tree.");
  } else if 1 == n {
    println!("A partridge in a pear tree.");
  }
  println!("");
}

