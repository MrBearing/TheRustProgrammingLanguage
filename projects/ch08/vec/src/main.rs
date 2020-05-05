fn main() {
    
    let v :Vec<i32> = Vec::new();
    println!("{:?}",v);
    let v1  = vec![1,2,3,4];
    println!("{:?}",v1);
    let mut mut_vec = Vec::new();
    mut_vec.push(5);
    mut_vec.push(6);
    mut_vec.push(7);
    mut_vec.push(8);
    println!("{:?}",mut_vec);

    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("{:?}",third);
    let third: Option<&i32> = v.get(2);
    println!("{:?}",third);
    
    //範囲外を動かした場合
    // let does_not_exist = &v[100];// 実行するとPanic
    // println!("{:?}",does_not_exist);

    let does_not_exist = v.get(100); // None
    println!("{:?}",does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    println!("{:?}",first);
    v.push(6);
    println!("{:?}",v);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{:?}",i)
    }
    
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; //参照外し
    }

    println!("{:?}",v);

    // list 8-10
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}",row);

}
