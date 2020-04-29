fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1//; // セミコロン無で戻り値
    // ()の行が隠れてる () : はユニット型
}


// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }


// fn main() {
//     let x = 5;

//     let y = {
//         let x = 3;
//         println!("The value of x is: {}", x); //3
//         x + 1
//     };

//     println!("The value of y is: {}", y); // 4
//     println!("The value of x is: {}", x); // 5
// }


// fn main() {
//     let x = (let y = 6); // コンパイルエラー
// }


// fn main() {
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }



// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);   // xの値は{}です
// }


// fn main() {
//     println!("Hello world!");
//     another_function();
// }

// fn another_function() {
//     println!("Another functio.");
// }