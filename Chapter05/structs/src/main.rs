struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let mut main_user = build_user(String::from("someone@example.com"), String::from("someusername123"));
  let secondary_user = User {
    username: String::from("Secondary User"),
    email: String::from("secondary_user@example.com"),
    ..main_user
  };
  main_user.email = String::from("another_email@example.com");
  println!("Hello, {} ({})!", main_user.username, main_user.email);
  display_user(&main_user);
  display_user(&secondary_user);
  println!("Hello, {} ({})!", main_user.username, main_user.email);

  let _black = Color(0, 0, 0);
  let _origin = Point(0, 0, 0);
}

fn display_user(user: &User) {
  println!("---");
  println!("Username: {}", user.username);
  println!("Email: {}", user.email);
  println!("Active: {}", user.active);
  println!("Signin Count: {}", user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 0,
  }
}

