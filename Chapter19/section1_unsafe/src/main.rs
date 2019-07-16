use std::slice;

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
}

fn main_a() {
  let mut num = 5;
  println!("num = {}", num);

  let r1 = &num as *const i32;
  println!("r1 = {:?}", r1);
  unsafe {
    println!("*r1 = {}", *r1);
  }

  let r2 = &mut num as *mut i32;
  println!("r2 = {:?}", r2);
  unsafe {
    println!("*r2 = {}", *r2);
  }
}

fn main_b() {
  let address = 0x012345usize;
  let r = address as *const i32;

  println!("r = {:?}", r);
}

fn main_c() {
  unsafe {
    dangerous();
  }
}

unsafe fn dangerous() {
  println!("unsafe fn dangerous() called");
}

fn main_d() {
  let mut v = vec![1, 2, 3, 4, 5, 6];

  let r = &mut v[..];

  let (a, b) = r.split_at_mut(3);

  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);

  println!("a = {:?}", a);
  println!("b = {:?}", b);
}

fn main_e() {
  let mut v = vec![1, 2, 3, 4, 5, 6];

  let r = &mut v[..];

  let (a, b) = split_at_mut(r, 3);

  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);

  println!("a = {:?}", a);
  println!("b = {:?}", b);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
    )
  }
}

extern "C" {
  fn abs(input: i32) -> i32;
}

fn main_f() {
  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }
  call_from_c();
}

#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn main_g() {
  add_to_count(3);

  unsafe {
    println!("COUNTER: {}", COUNTER);
  }
}

fn add_to_count(inc: u32) {
  unsafe {
    COUNTER += inc;
  }
}

unsafe trait Foo {
  // methods go here
}

unsafe impl Foo for i32 {
  // method implementations go here
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
}

