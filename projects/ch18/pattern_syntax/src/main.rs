fn main() {
    let x = 1;

    match x {
        1 => println!("one"),       // 1
        2 => println!("two"),       // 2
        3 => println!("three"),     // 3
        _ => println!("anything"),  // なんでも
    }

    //
    // クイズ：このコードの出力結果は？
    // 
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x), // Noneのときのみ
    }
    // 答えは　Matched, y = 5　(y はあくまでmatch式内で宣言された変数)
    println!("at the end: x = {:?}, y = {:?}", x, y);
    //

    //
    // 複数の条件に同じ動作を指定する
    //
    let x = 1;
    match x {
        // 1か2
        1 | 2 => println!("one or two"),
        // 3
        3 => println!("three"),
        // なんでも
        _ => println!("anything"),
    }

    //
    // 範囲に当てることもできる
    //
    let x = 'c';

    match x {
        // ASCII文字前半
        // 'a' ... 'j' => println!("early ASCII letter"), 
        // ... を使用する範囲指定はdeprecatedです。
        'a' ..= 'j' => println!("early ASCII letter"),
        // ASCII文字後半
        // 'k' ... 'z' => println!("late ASCII letter"),
        'k' ..=  'z' => println!("late ASCII letter"),
        // それ以外
        _ => println!("something else"),
    }

    //
    // 構造体にパターンマッチもできる
    //
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };

    match p {
        // x軸上の{}
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // y軸上の{}
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // どちらの軸上でもない: ({}, {})
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    //
    // enumも分配できる
    //
    let _msg = Message::Move{x:19 , y:20};
    let _msg = Message::Write;
    let _msg = Message::Quit;
    let msg = Message::ChangeColor(0, 160, 255);
    
    match msg {
        Message::Quit => {
            // Quit列挙子には分配すべきデータがない
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                // x方向に{}、y方向に{}だけ動く
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        // テキストメッセージ: {}
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                // 色を赤{}, 緑{}, 青{}に変更
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
    //
    // 参照も可能です
    //
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|Point { x, y }| x * x + y * y) // 型不一致エラー出ないです。。。
        // .map(|&Point { x, y }| x * x + y * y)
        .sum();

    println!("{}",sum_of_squares);


    // _ で一部の値を無視する

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            // 何らかの数値: {}, {}, {}
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    // match numbers {
    //     (.., second , ..) => {// タプル内では1回しか..は使えません
    //         println!("Some numbers: {}", second)
    //     },
    // }

    let robot_name = Some(String::from("Bors"));

    match robot_name {
        // 名前が見つかりました: {}
        Some(name) => println!("Found a name: {}", name),
        None => (),
    }
    // robot_nameは: {:?}
    // println!("robot_name is: {:?}", robot_name); // 値の所有権が奪われるのでエラー
    


    let robot_name = Some(String::from("Bors"));
    match robot_name {
        // ref キーワードを使うと所有権が移動しません
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    // robot_nameは: {:?}
    println!("robot_name is: {:?}", robot_name);



    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        // ref mutで可変参照をにできます
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }
    // robot_nameは: {:?}
    println!("robot_name is: {:?}", robot_name); // 値の所有権が奪われるのでエラー


    // 
    // match式にガードを導入する
    // 
    let num = Some(4);

    match num {
        // 5未満です: {}
        Some(x) if x < 5 => println!("less than five: {}", x), // ガード
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);


    let x = 4;
    let y = false;

    match x {
        // はい
        4 | 5 | 6 if y => println!("yes"),

        // いいえ
        _ => println!("no"),
    }

    enum Message2 {
        Hello { id: i32 },
    }
    
    let msg = Message2::Hello { id: 5 };
    
    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => {
            // 範囲内のidが見つかりました: {}
            println!("Found an id in range: {}", id_variable)
        },
        Message2::Hello { id: 10..=12 } => {
            // 別の範囲内のidが見つかりました
            println!("Found an id in another range")
        },
        Message2::Hello { id } => {
            // それ以外のidが見つかりました
            println!("Found some other id: {}", id)
        },
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
