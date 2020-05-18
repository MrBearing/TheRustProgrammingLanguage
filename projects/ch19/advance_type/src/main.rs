// typeを使ってエイリアス可能
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

// "!"はと特殊な戻値型　戻りませんが。。。
pub fn bar() -> ! {
    loop{

    } // loop は！型
}
pub fn generic<T>(_t: T){}
// 上の糖衣構文
pub fn generic_syn_sugar<T: Sized>(_t: T) {}

pub fn generic_unsized<T: ?Sized>(_t: &T) {}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let _f: Thunk = Box::new(|| println!("hi"));

}
