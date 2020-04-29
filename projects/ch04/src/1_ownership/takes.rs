#![allow(unused_variables)]

fn main() {
    let s = String::from("hello");  // sがスコープに入る
    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない
    //  println!("{}", s); // ムーブしているのでコンパイルエラー
    
    let x = 5;                      // xがスコープに入る
    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても大丈夫
    println!("{}", x);
} // ここでxがスコープを抜ける。sも同様。だけど、sの値はムーブされてるので、何も特別なことはない。

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
  println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。


fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
  println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
