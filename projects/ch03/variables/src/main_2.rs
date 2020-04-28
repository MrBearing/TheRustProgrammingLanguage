
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10; // 範囲外のインデックス コンパイルは通る

    let element = a[index]; // 実行時にpanick する

    println!("The value of element is: {}", element);   // 要素の値は{}です
}

// fn main() {
//     // 足し算
//     let sum = 5 + 10;

//     // 引き算
//     let difference = 95.5 - 4.3;

//     // 掛け算
//     let product = 4 * 30;

//     // 割り算
//     let quotient = 56.7 / 32.2;

//     // 余り
//     let remainder = 43 % 5;

//     let c = 'z';
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';    //ハート目の猫
//  }



// #![allow(unused_variables)]
// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number");
// }