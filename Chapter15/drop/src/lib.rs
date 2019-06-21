pub struct CustomSmartPointer {
  pub data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn drop_ok() {
    let c = CustomSmartPointer { data: String::from("test") };
    drop(c);
  }
}

