fn main(){

    let v1: Vec<i32> = vec![1, 2, 3];

    let result: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // 上のVec<_>便利
    println!("{:?}",result);
}