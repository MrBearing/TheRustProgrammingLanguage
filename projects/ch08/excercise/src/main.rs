use excercise::statistics;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut array = vec![1,2,3,4,5,6,7,8,9,10];
    array.shuffle(&mut rng);
    println!("{:?}",array);
    statistics::median(&mut array);
    println!("{:?}",array);
}
