#[macro_export]
macro_rules! vec2 {
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($x);
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
}

// use proc_macro;
// 
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream { }

// #[route(GET, "/")]
// fn index() {

// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

// let sql = sql!(SELECT * FROM posts WHERE id=1);

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vec2_ok() {
    let mut v = vec2!(1, 2, 3);
    assert_eq!(Some(3), v.pop());
    assert_eq!(Some(3), v.pop());
    assert_eq!(Some(2), v.pop());
    assert_eq!(Some(2), v.pop());
    assert_eq!(Some(1), v.pop());
    assert_eq!(Some(1), v.pop());
    assert_eq!(None, v.pop());
    assert_eq!(None, v.pop());
  }
}

