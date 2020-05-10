
#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

use std::fmt::Display;

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    // アナウンス！
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");  // '.'が見つかりませんでした
    let i = ImportantExcerpt { part: first_sentence };
    println!("{:?}",i);

    let s: &'static str = "I have a static lifetime.";
    println!("{:?}",s);

    let string1 = String::from("long string is long");

    let string2 = String::from("xyz");
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str() , "this is announcement");
    println!("{:?}",result);
}

// fn main() {
//     // 長い文字列は長い
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }

//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     // 本当に長い文字列
//     let result = String::from("really long string");
//     result.as_str() // エラー
// }


// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
