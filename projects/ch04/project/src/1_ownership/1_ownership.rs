
#![allow(unused_variables)]
fn main() {
    let mut str1 = "hello";
    println!("{}", str1); // これは`hello`と出力する
    let str2 = str1;
    // let str2 = str1 + "world"; これはできない
    println!("{}", str2); // これは`hello`と出力する

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    println!("{}", s); // これは`hello, world!`と出力する

    let s1 = String::from("hello");
    let s2 = s1; // s1 -> s2 にmoveしている = 所有権が移動している
    // println!("{}, world!", s1);// <-コンパイルエラーになる
    println!("{}, world!", s2);// <-コンパイルエラーにならない

    let s1 = String::from("hello");
    let s2 = s1.clone(); // ヒープのディープコピー
    println!("s1 = {}, s2 = {}", s1, s2); 

    let x = 5;
    let y = x; // コピーしているがdeep も shallow も差がない
    println!("x = {}, y = {}", x, y);
}

