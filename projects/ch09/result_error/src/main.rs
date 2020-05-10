use std::fs::File;
use std::io;
use std::io::Read;
use std::net::IpAddr;


fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?; // < 便利！！
    Ok(s)
}

fn read_username_from_file() -> Result<String,io::Error>{
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main(){
    match read_username_from_file(){
        Ok(file_string) => println!("{:?}",file_string),
        Err(e) => println!("{:?}",e),
    }
    match read_username_from_file2(){
        Ok(file_string) => println!("{:?}",file_string),
        Err(e) => println!("{:?}",e),
    }
    // let f = File::open("hello.txt")?; // error: 関数の戻り値がResult or Optionでないと使えない

    let home: IpAddr = "127.0.0.1".parse().unwrap();// 直に&strをparseしていたりするときはunwrapしてもOK
    println!("{:?}",home);
}

// fn main(){
//     //let f = File::open("hello.txt").unwrap();
//     // unwrap失敗時にメッセージ表示
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
//     println!("{:?}",f);
// }

// パニックさせない例
// use std::fs::File;
// use std::io::ErrorKind;
// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(ref error) if error.kind() == ErrorKind::NotFound => {
//             match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => {
//                     panic!(
//                         //ファイルを作成しようとしましたが、問題がありました
//                         "Tried to create file but there was a problem: {:?}",
//                         e
//                     )
//                 },
//             }
//         },
//         Err(error) => {
//             panic!(
//                 "There was a problem opening the file: {:?}",
//                 error
//             )
//         },
//     };

//     println!("{:?}",f);
   
//}
