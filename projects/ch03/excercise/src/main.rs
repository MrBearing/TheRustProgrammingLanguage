
use excercise::temp::Temprature;

extern crate numeral;
use numeral::Cardinal;
use excercise::twelve_days;
fn main() {
    println!("Hello, world! {} ", (1+2)*3);
    let c_0_degree = Temprature::from_celsius(0.0);
    println!("{} F",c_0_degree.as_fahrenheit());
    let f50_degree = Temprature::from_fahrenheit(50.0);
    println!("{} C",f50_degree.as_celsius());

    let n = 127;
    println!("{} is written: {}", n, n.cardinal());
    let st = "abcdefghij";
    println!("{}",&st[..3]);

    for i in 1..=12 {
        let result = twelve_days::lyrics::get_lyrics(i);
        match result {
            Some(r) => print!("{}",r),
            None => println!("fail"),
        }
        println!("");
    }

}
