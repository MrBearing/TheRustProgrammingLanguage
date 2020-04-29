fn main() {
  let mut s = String::from("hello");

  // let r1 = &mut s;
  // let r2 = &mut s; //コンパイルエラー 2回以上の参照はできない
  // println!("{},{}",r1, r2 );

  let r1 = &s; // 問題なし
  let r2 = &s; // 問題なし
  println!("{},{}",r1, r2 );

  // let r3 = &mut s; // 大問題！
  // println!("{},{},{}",r1, r2, r3 );

}
