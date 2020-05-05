fn main() {
    // let mut s = String::new();
    // println!("{:?}",s);
    let data = "initial contents";
    let s = data.to_string();
    println!("{:?}",s);
    let s = "initial_contents".to_string();
    println!("{:?}",s);
    let s = String::from("initial_contents");
    println!("{:?}",s);
    
    let v = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    for i in v {
        println!("{:?}",i);
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{:?}",s);


  
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    
    let mut s = String::from("lo");
    s.push('l');
    println!("{:?}",s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意

    // println!("{:?}",s1);  // ムーブされてるのでコンパイルエラー
    println!("{:?}",s3);

    let len = String::from("Hola").len();
    println!("{:?}",len);

    let len = String::from("Здравствуйте").len(); // 24 
    println!("{:?}",len);

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    // println!("{:?}",answer);

    let s = &hello[0..4];
    println!("{:?}",s);


    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

}

