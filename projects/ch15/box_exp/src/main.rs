/// よく見る関数型言語でのリスト実装
#[derive(Debug)]
enum List {
    Cons(i32,Box<List>),
    //Cons(i32,List), //ヒープでのListのサイズが決定できないからコンパイルエラー
    Nil,
}

use List::{Cons,Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}",b);
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("{:?}",list);
}
