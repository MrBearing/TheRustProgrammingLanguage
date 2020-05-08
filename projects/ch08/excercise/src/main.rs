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

fn main(){
    let st = "first!";//String::from("First");
    println!("{:?}",st);

    let s = &st[1..st.len()-1];
    println!("{:?}",s);
    let s1 = &st[..1];
    println!("{:?}",s1);
    
    //let pig  = [s,s1,"ay"].concat();
    let pig  = [&st[1..st.len()-1], &st[..1],"ay"].concat();
    println!("{:?}",pig);
    
    


}