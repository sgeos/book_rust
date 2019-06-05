use std::fmt;

struct Point<T> {
  x: T,
  y: T,
}

impl fmt::Display for Point<f32> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let &Point {x, y} = self;
    write!(formatter, "({:.3}, {:.3})", x, y)
  }
}

impl fmt::Display for Point<i32> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let &Point {x, y} = self;
    write!(formatter, "({}, {})", x, y)
  }
}

// impl<T: fmt::Display> fmt::Display for Point<T> {
//   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//     let &Point {ref x, ref y} = self;
//     write!(formatter, "({}, {})", x, y)
//   }
// }

fn main() {
  println!("{}", Point { x: 5, y: 10 });
  println!("{}", Point { x: 1.5, y: 11.0/3.0 });
  print_max(&vec![34, 50, 25, 100, 65]);
  print_max(&vec!['y', 'm', 'a', 'q']);
}

fn print_max<T: Copy + fmt::Debug + fmt::Display + PartialOrd>(list: &[T]) {
  println!("Max of {:?} is {:?}.", list, max(list));
}

fn max<T: Copy + PartialOrd>(list: &[T]) -> T {
  let mut max_value = list[0];
  for &value in list.iter() {
    if max_value < value {
      max_value = value;
    }
  }
  max_value
}

