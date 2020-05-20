use std::time::Instant;

/// 実行時間計測マクロ
/// 
/// 参考にした
/// [Qiita記事](https://qiita.com/pseudo_foxkeh/items/5d5226e3ffa27631e80d)
/// 
macro_rules! measure {
  ( $x:expr ) => {
    {
      let start = Instant::now();
      let result = $x; //実行される関数や式
      let end = start.elapsed();
      println!("time: {:02}.{:09} , result :{:.6}",
        end.as_secs(), end.subsec_nanos(), result);
      result
    }
  };
}


fn a(n: i32) -> f64 {
    match n {
        0 => 1.0,
        _ => (a(n-1) + b(n-1) ) / 2.0,
    }
}

fn b(n: i32) -> f64 {
    match n {
        0 => 1.0/2.0_f64.sqrt(),
        _ => (a(n-1)*b(n-1)).sqrt()
    }
}

fn p(n: i32) -> f64 {
    match n {
        0 => 1.0,
        _ => 2.0*p(n-1),
    }
}

fn t(n: i32) -> f64 {
    match n {
        0 => 1.0 /4.0,
        _ => t(n-1) - (a(n-1) - a(n)).powi(2)*p(n-1)
    }
}

fn pi_recursion(n: i32)->f64{
    (a(n) + b(n)).powf(2.0)/4.0/t(n)
}

fn pi_loop(d: i32) -> f64{
    let mut a = 1.0;
    let mut b = 1.0/2.0_f64.sqrt();
    let mut t = 1.0/4.0;
    let mut p = 1.0;
    
    for _ in 1..=d {
        let an = (a + b) / 2.0;
        b = (a*b).sqrt();
        t -= (an -a).powi(2) * p;
        p *= 2.0;
        a = an;
    }
    (a + b).powi(2) / (4.0 * t)
}

struct Pi{
    a: f64,
    b: f64,
    t: f64,
    p: f64,
}

impl Pi {
    fn new() -> Pi {
        Pi{
             a: 1.0,
             b: 1.0/2.0_f64.sqrt(),
             t: 1.0/4.0,
             p: 1.0,
        }
    }

    fn get(&self) -> f64{
        (self.a + self.b).powi(2) / (4.0 * self.t)
    }
}

fn pi_iter(d: i32) -> f64{
    (1..=d).fold(Pi::new(), move | acc, _| {
        let an = (acc.a + acc.b) / 2.0;
        Pi {
            a: an,
            b: (acc.a*acc.b).sqrt(),
            t: acc.t - (an -acc.a).powi(2) * acc.p,
            p: 2.0*acc.p,
        }
    }).get()
}

/// 計算結果
/// loop :: time: 00.000002055 
/// iter :: time: 00.000002295 
/// rec  :: time: 00.661693180
/// 
/// 所感・考察:イテレータのほうが若干遅い。 
/// 1000回目までの計算時間はだいたい10micro程度の差が出た。
/// 構造体を定義してしまったのが意外と効いてしまってる？
/// 
/// イテレータの実装はfold()を見つけるまでに苦労した。
/// 
/// 再帰はシャレで実装したけど計算結果のメモ化がされてないため、
/// 実行時間がかなりかかってる印象　aとbだけでもメモ化すればかなり変わるかな。

fn main() {
    print!("loop :: ");
    measure!(pi_loop(23));
    print!("iter :: ");
    measure!(pi_iter(23));
    print!("rec  :: ");
    measure!(pi_recursion(23)); // bの計算値がメモ化されていないため、遅い
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn assert_diff( expected: f64, actual: f64 , digit: i32) -> bool{
        let diff = (expected - actual).abs();
        println!("exp:{:<20}, act:{:<20}, diff:{:<20}",expected, actual, diff);
        diff <= 10.0_f64.powf(-digit as f64)
    } 
    
 
    #[test]
    fn test_pi_recursion() {
        print!("0 ,");
        assert!(assert_diff(2.9142, pi_recursion(0), 4));

        print!("1 ,");
        assert!(assert_diff(PI, pi_recursion(1), 2));
        
        print!("2 ,");
        assert!(assert_diff(PI, pi_recursion(2), 7));
        
        print!("3 ,");
        assert!(assert_diff(PI, pi_recursion(3),15));
    }
    #[test]
    fn test_pi_loop() {
        print!("0 ,");
        assert!(assert_diff(2.9142, pi_loop(0), 4));

        print!("1 ,");
        assert!(assert_diff(PI, pi_loop(1), 2));
        
        print!("2 ,");
        assert!(assert_diff(PI, pi_loop(2), 7));
        
        print!("3 ,");
        assert!(assert_diff(PI, pi_loop(3),15));
    }

    #[test]
    fn test_pi_iter() {
        print!("0 ,");
        assert!(assert_diff(2.9142, pi_iter(0), 4));

        print!("1 ,");
        assert!(assert_diff(PI, pi_iter(1), 2));
        
        print!("2 ,");
        assert!(assert_diff(PI, pi_iter(2), 7));
        
        print!("3 ,");
        assert!(assert_diff(PI, pi_iter(3),15));
    }

}
