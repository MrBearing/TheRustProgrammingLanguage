struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    let c = CustomSmartPointer { data: String::from("Some data") };
    //c.drop();//コンパイルエラー　明示的にdropを呼ぶことはできない
    drop(c); //早期にdropさせたいならばこの方法を使う
    let _d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");    
}
