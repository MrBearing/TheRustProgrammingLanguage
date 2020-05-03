pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules(st:String) {
                println!("{}",st);
            }
        }
    }
}

use a::series::of;
use a::series::of::nested_modules;


#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};
use TrafficLight::*;


fn main() {
    print!("Hello, wrold!");
    a::series::of::nested_modules(String::from("full path"));
    of::nested_modules(String::from("of::nested_modules"));
    nested_modules(String::from("only my name!"));

    let red = Red;
    let yellow = Yellow;
    let green = Green;// TrafficLight::Green;

    println!("{:?},{:?},{:?}",red,yellow,green);

}