extern crate adder;

use adder::add_two;

mod common;

#[test]
fn add_two_ok() {
  common::setup();
  assert_eq!(4, adder::add_two(2));
  assert_eq!(4, add_two(2));
}

