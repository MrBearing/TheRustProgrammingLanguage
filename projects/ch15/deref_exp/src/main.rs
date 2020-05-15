pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }

    pub fn my_deref(&self)-> &T {
        &self.0
    }
}

use std::ops::Deref;

impl <T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}


fn hello(name: &str){
    println!("Hello , {}",name);
}


fn main(){
    let m = MyBox::new(String::from("Rust"));
    hello(&m);//型矯正で自動的に下記の変換がなされる
    hello(&(*m)[..]);// 文字列全体のStringの文字列スライスの参照を渡す
}


#[test]
fn test() {
    let x = 5;
    //let y = &x;
    //let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.my_deref()));//参照先の値を返すように工夫すると実装可能

    assert_eq!(5, *(y.deref()));
    assert_eq!(5, *y); // 上の糖衣構文　derefを実装しないとコンパイルエラー
}
