#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("a = {:?}",a);
    println!("b = {:?}",b);
    println!("c = {:?}",c);
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // a生成後のカウント = {}
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    // b生成後のカウント = {}
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        // c生成後のカウント = {}
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // cがスコープを抜けた後のカウント = {}
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// Box を使った例、コンパイルエラーになる

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Cons(5,
//         Box::new(Cons(10,
//             Box::new(Nil))));
//     let b = Cons(3, Box::new(a)); // ここで所有権移動してる
//     let c = Cons(4, Box::new(a)); // 所有権は再移動できない
// }


