

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
        _ => t(n-1) - (a(n-1) - a(n)).powf(2.0)*p(n-1)
    }
}

fn pi(n: i32)->f64{
    (a(n) + b(n)).powf(2.0)/4.0/t(n)
}

fn main() {
    for n in 1..10 {
        let pi = pi(n);
        println!("{:?}",pi);
    }
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
    fn test_pi() {
        print!("0 ,");
        assert!(assert_diff(2.9142, pi(0), 4));

        print!("1 ,");
        assert!(assert_diff(PI, pi(1), 2));
        
        print!("2 ,");
        assert!(assert_diff(PI, pi(2), 7));
        
        print!("3 ,");
        assert!(assert_diff(PI, pi(3),15));
    }

}
