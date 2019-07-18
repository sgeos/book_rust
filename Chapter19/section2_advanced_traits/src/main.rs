use std::fmt;
use std::ops::Add;

fn main() {
  let variation = 0;

  match variation {
    1 => wrap_function("Main A".to_string(), main_a),
    2 => wrap_function("Main B".to_string(), main_b),
    3 => wrap_function("Main C".to_string(), main_c),
    4 => wrap_function("Main D".to_string(), main_d),
    5 => wrap_function("Main E".to_string(), main_e),
    6 => wrap_function("Main F".to_string(), main_f),
    7 => wrap_function("Main G".to_string(), main_g),
    8 => wrap_function("Main H".to_string(), main_h),
    _ => wrap_function("Main All".to_string(), main_all),
  };
}

fn wrap_function<F>(title: String, f: F) where F: FnOnce() {
  println!("--- {} START ---", title);
  f();
  println!("--- {} END ---", title);
}

fn main_all() {
  wrap_function("Main A".to_string(), main_a);
  wrap_function("Main B".to_string(), main_b);
  wrap_function("Main C".to_string(), main_c);
  wrap_function("Main D".to_string(), main_d);
  wrap_function("Main E".to_string(), main_e);
  wrap_function("Main F".to_string(), main_f);
  wrap_function("Main G".to_string(), main_g);
  wrap_function("Main H".to_string(), main_h);
}

#[derive(Debug, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

fn main_a() {
  let point_a = Point { x: 1, y: 0 };
  let point_b = Point { x: 2, y: 3 };
  let point_c = Point { x: 3, y: 3 };
  println!("{:?} + {:?} = {:?}", point_a, point_b, point_c);
  assert_eq!(point_a + point_b, point_c);
}

// trait Add<RHS=Self> {
//   type Output;
// 
//   fn add(self, rhs: RHS) -> Self::Output;
// }

struct Meters(u32);
struct Millimeters(u32);

impl fmt::Display for Meters {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} m", self.0)
  }
}

impl fmt::Display for Millimeters {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} mm", self.0)
  }
}

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000))
  }
}

fn main_b() {
  let m = Meters(2);
  let mm = Millimeters(56);

  print!("{} + {} = ", mm, m);
  println!("{}", mm + m);
}

trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("Up!");
  }
}

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

fn main_c() {
  let person = Human;
  person.fly();
}

fn main_d() {
  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  Human::fly(&person);
  person.fly();
}

trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

fn main_e() {
  println!("A baby dog is called a {}.", Dog::baby_name());
}

fn main_f() {
  // <Type as Trait>::function(receiver_if_method, next_arg, ...);
  println!("A baby dog is called a {}.", <Dog as Animal>::baby_name());
}

trait OutlinePrint: fmt::Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

fn main_g() {
  let point = Point { x: 67, y: 33 };
  point.outline_print();
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

fn main_h() {
  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_ok() {
    main();
  }

  #[test]
  fn main_all_ok() {
    main_all();
  }

  #[test]
  fn main_a_ok() {
    main_a();
  }

  #[test]
  fn main_b_ok() {
    main_b();
  }

  #[test]
  fn main_c_ok() {
    main_c();
  }

  #[test]
  fn main_d_ok() {
    main_d();
  }

  #[test]
  fn main_e_ok() {
    main_e();
  }

  #[test]
  fn main_f_ok() {
    main_f();
  }

  #[test]
  fn main_g_ok() {
    main_g();
  }

  #[test]
  fn main_h_ok() {
    main_h();
  }
}

