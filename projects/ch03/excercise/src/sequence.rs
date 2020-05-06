pub mod fibonacci{
    /// 再帰を使用した求め方
    pub fn get_by_recursion(order: i32) -> i32{
        match order {
            0 => 0,
            1 => 1,
            _ => get_by_recursion(order - 1) + get_by_recursion(order - 2),
        }
    }

    /// ループで求める方法
    pub fn get_by_loop(n :i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut a1 = 1;
        let mut an = 1;
        for _ in 0..n-2 {
            let tmp = an;
            an = an + a1;
            a1 = tmp;
        }
        an
    }

    /// 一般項を用いた求め方
    pub fn get_by_genral_term(n :i32) -> f64{
        let sqrt_5 = 5.0_f64.sqrt();
        let gold : f64 = (1.0_f64 + sqrt_5)/2.0;
        let a :f64  = gold.powi(n)/sqrt_5 + 1.0/2.0;
        a.floor()
    }
}

#[cfg(test)]
mod tests{
    use super::fibonacci;
    #[test]
    fn test_loop() {
        assert_eq!(fibonacci::get_by_loop(0),0);
        assert_eq!(fibonacci::get_by_loop(1),1);
        assert_eq!(fibonacci::get_by_loop(2),1);
        assert_eq!(fibonacci::get_by_loop(3),2);
        assert_eq!(fibonacci::get_by_loop(4),3);
        assert_eq!(fibonacci::get_by_loop(5),5);
        assert_eq!(fibonacci::get_by_loop(12),144);
    }

    #[test]
    fn test_general_term() {
        assert_eq!(fibonacci::get_by_genral_term(0),0.0);
        assert_eq!(fibonacci::get_by_genral_term(1),1.0);
        assert_eq!(fibonacci::get_by_genral_term(2),1.0);
        assert_eq!(fibonacci::get_by_genral_term(3),2.0);
        assert_eq!(fibonacci::get_by_genral_term(4),3.0);
        assert_eq!(fibonacci::get_by_genral_term(5),5.0);
        assert_eq!(fibonacci::get_by_genral_term(12),144.0);
    }

    #[test]
    fn test_recursion() {
        assert_eq!(fibonacci::get_by_recursion(0),0);
        assert_eq!(fibonacci::get_by_recursion(1),1);
        assert_eq!(fibonacci::get_by_recursion(2),1);
        assert_eq!(fibonacci::get_by_recursion(3),2);
        assert_eq!(fibonacci::get_by_recursion(4),3);
        assert_eq!(fibonacci::get_by_recursion(5),5);
        assert_eq!(fibonacci::get_by_recursion(12),144);
    }

}