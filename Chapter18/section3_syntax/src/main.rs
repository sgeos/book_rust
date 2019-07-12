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
    9 => wrap_function("Main I".to_string(), main_i),
    10 => wrap_function("Main J".to_string(), main_j),
    11 => wrap_function("Main K".to_string(), main_k),
    12 => wrap_function("Main L".to_string(), main_l),
    13 => wrap_function("Main M".to_string(), main_m),
    14 => wrap_function("Main N".to_string(), main_n),
    15 => wrap_function("Main O".to_string(), main_o),
    16 => wrap_function("Main P".to_string(), main_p),
    17 => wrap_function("Main Q".to_string(), main_q),
    18 => wrap_function("Main R".to_string(), main_r),
    19 => wrap_function("Main S".to_string(), main_s),
    20 => wrap_function("Main T".to_string(), main_t),
    21 => wrap_function("Main U".to_string(), main_u),
    22 => wrap_function("Main V".to_string(), main_v),
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
  wrap_function("Main I".to_string(), main_i);
  wrap_function("Main J".to_string(), main_j);
  wrap_function("Main K".to_string(), main_k);
  wrap_function("Main L".to_string(), main_l);
  wrap_function("Main M".to_string(), main_m);
  wrap_function("Main N".to_string(), main_n);
  wrap_function("Main O".to_string(), main_o);
  wrap_function("Main P".to_string(), main_p);
  wrap_function("Main Q".to_string(), main_q);
  wrap_function("Main R".to_string(), main_r);
  wrap_function("Main S".to_string(), main_s);
  wrap_function("Main T".to_string(), main_t);
  wrap_function("Main U".to_string(), main_u);
  wrap_function("Main V".to_string(), main_v);
}

fn main_a() {
  let x = 1;

  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
  }
}

fn main_b() {
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn main_c() {
  let x = 1;

  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }
}

fn main_d() {
  let x = 5;

  match x {
    1...5 => println!("one through five"),
    _ => println!("something else"),
  }
}

fn main_e() {
  let x = 'c';

  match x {
    'a'...'j' => println!("early ASCII letter"),
    'k'...'z' => println!("late ASCII letter"),
    _ => println!("something else"),
  }
}

struct Point {
  x: i32,
  y: i32,
}

fn main_f() {
  let p = Point { x: 0, y: 7 };

  let Point { x: a, y: b } = p;
  assert_eq!(0, a);
  assert_eq!(7, b);
  println!("a: {}, b: {}", a, b);
}

fn main_g() {
  let p = Point { x: 0, y: 7 };

  let Point { x, y } = p;
  assert_eq!(0, x);
  assert_eq!(7, y);
  println!("x: {}, y: {}", x, y);
}

fn main_h() {
  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }
}

enum MessageI {
  _Quit,
  _Move { x: i32, y: i32 },
  _Write(String),
  ChangeColor(i32, i32, i32),
}

fn main_i() {
  let msg = MessageI::ChangeColor(0, 160, 255);

  match msg {
    MessageI::_Quit => {
      println!("The Quit variant has no data to destructure.")
    },
    MessageI::_Move { x, y } => {
      println!(
        "Move in the x direction {} and in the y direction {}",
        x,
        y
      );
    }
    MessageI::_Write(text) => println!("Text message: {}", text),
    MessageI::ChangeColor(r, g, b) => {
      println!(
        "Change the color to red {}, green {}, and blue {}",
        r,
        g,
        b
      )
    }
  }
}

enum Color {
 _Rgb(i32, i32, i32),
 Hsv(i32, i32, i32),
}

enum MessageJ {
  _Quit,
  _Move { x: i32, y: i32 },
  _Write(String),
  ChangeColor(Color),
}

fn main_j() {
  let msg = MessageJ::ChangeColor(Color::Hsv(0, 160, 255));

  match msg {
    MessageJ::_Quit => {
      println!("The Quit variant has no data to destructure.")
    },
    MessageJ::_Move { x, y } => {
      println!(
        "Move in the x direction {} and in the y direction {}",
        x,
        y
      );
    }
    MessageJ::_Write(text) => println!("Text message: {}", text),
    MessageJ::ChangeColor(Color::_Rgb(r, g, b)) => {
      println!(
        "Change the color to red {}, green {}, and blue {}",
        r,
        g,
        b
      )
    },
    MessageJ::ChangeColor(Color::Hsv(h, s, v)) => {
      println!(
        "Change the color to hue {}, saturation {}, and value {}",
        h,
        s,
        v
      )
    }
  }
}

fn main_k() {
  let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
  println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y)
}

fn main_l() {
  foo(3, 4);
}

fn foo(_: i32, y: i32) {
  println!("This code only uses the y parameter: {}", y);
}

fn main_m() {
  let mut setting_value = Some(5);
  let new_setting_value = Some(10);

  match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
      println!("Can't overwrite an existing customized value");
    }
    _ => {
      setting_value = new_setting_value;
    }
  }

  println!("setting is {:?}", setting_value);
}

fn main_n() {
  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, _, third, _, fifth) => {
      println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
  }
}

fn main_o() {
  let _x = 5;
  let y = 10;
  println!("y: {}", y)
}

fn main_p() {
  let s = Some(String::from("Hello!"));

  if let Some(_) = s {
    println!("found a string");
  }

  println!("{:?}", s);
}

struct PointQ {
  x: i32,
  _y: i32,
  _z: i32,
}

fn main_q() {
  let origin = PointQ { x: 0, _y: 0, _z: 0 };

  match origin {
    PointQ { x, .. } => println!("x is {}", x),
  }
}

fn main_r() {
  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    },
  }
}

fn main_s() {
  let num = Some(4);

  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }
}

fn main_t() {
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {:?}", n),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn main_u() {
  let x = 4;
  let y = false;

  match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }
}

enum MessageU {
  Hello { id: i32 },
}

fn main_v() {
  let msg = MessageU::Hello { id: 5 };

  match msg {
    MessageU::Hello { id: id_variable @ 3...7 } => {
      println!("Found an id in range: {}", id_variable)
    },
    MessageU::Hello { id: 10...12 } => {
      println!("Found an id in another range")
    },
    MessageU::Hello { id } => {
      println!("Found some other id: {}", id)
    },
  }
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

  #[test]
  fn main_i_ok() {
    main_i();
  }

  #[test]
  fn main_j_ok() {
    main_j();
  }

  #[test]
  fn main_k_ok() {
    main_k();
  }

  #[test]
  fn main_l_ok() {
    main_l();
  }

  #[test]
  fn main_m_ok() {
    main_m();
  }

  #[test]
  fn main_n_ok() {
    main_n();
  }

  #[test]
  fn main_o_ok() {
    main_o();
  }

  #[test]
  fn main_p_ok() {
    main_p();
  }

  #[test]
  fn main_q_ok() {
    main_q();
  }

  #[test]
  fn main_r_ok() {
    main_r();
  }

  #[test]
  fn main_s_ok() {
    main_s();
  }

  #[test]
  fn main_t_ok() {
    main_t();
  }

  #[test]
  fn main_u_ok() {
    main_u();
  }

  #[test]
  fn main_v_ok() {
    main_v();
  }
}

