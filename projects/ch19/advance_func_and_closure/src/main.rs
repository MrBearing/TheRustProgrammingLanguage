fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// コンパイルエラー：コンパイル時にサイズ想定ができないから
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

// static参照にする
pub fn returns_closure_static() -> &'static dyn Fn(i32) -> i32 {
    &|x| x + 1
}

// Boxでサイズを固定すれば扱える
pub fn returns_closure_box() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);// add_oneの関数ポインタを引数に取る
    println!("The answer is: {}", answer);


    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string) //　関数ポインタの使用
        .collect();
    println!("{:?}",list_of_strings);
}