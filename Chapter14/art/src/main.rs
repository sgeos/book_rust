extern crate art as lib;

use lib::PrimaryColor;
use lib::mix;

fn main() {
  let red = PrimaryColor::Red;
  let yellow = PrimaryColor::Yellow;
  mix(red, yellow);
}

