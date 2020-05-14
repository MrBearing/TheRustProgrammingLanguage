extern crate hackelnickel;

use hackelnickel::PrimaryColor;
use hackelnickel::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red,yellow);
}