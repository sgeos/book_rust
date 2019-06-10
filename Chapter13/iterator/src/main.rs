extern crate iterator as lib;

fn main() {
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  for val in v1_iter {
    println!("Got: {}", val);
  }

  let shoes = vec![
    lib::Shoe { size: 10, style: String::from("sneaker") },
    lib::Shoe { size: 13, style: String::from("sandal") },
    lib::Shoe { size: 10, style: String::from("boot") },
  ];
  let size = 13;
  let shoes = lib::shoes_in_my_size(shoes, size);
  let z = lib::Counter::new()
    .zip(shoes.iter())
    .collect::<Vec<_>>();
  for (index, lib::Shoe {size, style}) in z.iter() {
    println!("Result {}: {} (size {})", index, style, size);
  }
}

