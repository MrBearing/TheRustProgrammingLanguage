mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();  // pubではないのでエラー
    outermost::inside::inner_function();  // insudeモジュールがpubではないのでエラー
    outermost::inside::secret_function(); // insudeモジュールがpubではないのでエラー
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
