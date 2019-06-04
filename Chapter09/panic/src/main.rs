fn main() {
  let list = vec![0, 1, 2, 3];
  let invalid_index = list.len() + 1;
  print_element(&list, invalid_index);
  panic!("crash and burn");
}

fn print_element(list: &Vec<i32>, index: usize) {
  println!("Element {}: {}", index, list[index]);
}

