//! # Exp cargo
//!
//! `my_crate`は、ユーティリティの集まりであり、特定の計算をより便利に行うことができます。

/// Adds one to the number given.


/// 与えられた数値に1を足す
/// # Examples
/// 以下の例はcargo testで自動的にテストに組み込まれる
/// ```
/// let five = 5;
///
/// assert_eq!(6, exp_cargo::add_one(5)); 
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

