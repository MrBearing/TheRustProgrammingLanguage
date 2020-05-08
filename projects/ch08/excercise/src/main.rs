// use excercise::statistics;
// use rand::prelude::*;

// fn main() {
//     let mut rng = rand::thread_rng();
//     let mut array = vec![1,2,3,4,5,6,7,8,9,10];
//     array.shuffle(&mut rng);
//     println!("{:?}",array);
//     statistics::median(&mut array);
//     println!("{:?}",array);
// }

// fn main(){
//     let st = "first!";//String::from("First");
//     println!("{:?}",st);

//     let s = &st[1..st.len()-1];
//     println!("{:?}",s);
//     let s1 = &st[..1];
//     println!("{:?}",s1);
    
//     //let pig  = [s,s1,"ay"].concat();
//     let pig  = [&st[1..st.len()-1], &st[..1],"ay"].concat();
//     println!("{:?}",pig);
// }
use excercise::employee_manager::*;
fn main(){
    // let mut iter = "A few words".split_ascii_whitespace();
    // assert_eq!(Some("A"), iter.next());
    // assert_eq!(Some("few"), iter.next());
    // assert_eq!(Some("words"), iter.next());
    // let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
    // assert_eq!(v, ["Mary", "had", "a little lambda"]);
    // let v: Vec<&str> = "Mary had a little lambda".splitn(2, ' ').collect();
    // println!("{:?}",v);
    run();
}
