
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1 {:?}",user1);

    let user2 = build_user(String::from(" anyone@example.com"),
            String::from(" anyusername123"));
    println!("user2 {:?}",user2);

    let user3 = User{
        email: String::from("abc@example.com"),
        username: String::from("abc"),
        ..user1 // 構造体更新記法: 残りのフィールドを user1 と同じにする
    };
    println!("user2 {:?}",user3);


    #[derive(Debug)] // println で文字列に変換する際に必要　Debugトレイトの自動実装
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);    

    println!("black {:?}",black);
    println!("origin {:?}",origin);
}
