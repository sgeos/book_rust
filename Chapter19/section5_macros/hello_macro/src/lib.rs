pub trait HelloMacro {
  fn hello_macro();
}

#[cfg(test)]
mod tests {
  #[test]
  fn success() {
    assert!(true);
  }
}
