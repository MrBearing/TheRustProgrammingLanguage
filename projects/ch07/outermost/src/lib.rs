pub mod outermost {
    pub fn middle_function() {
    }

    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {}

        pub fn secret_function() {
        }
    }

    mod m1 {
        fn f1(){}        
    }

    mod m2 {
        // fn f2() {
        //     crate::outermost::m1::f1();
        //     super::m1::f1();
        // }
    }
}

pub fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();  // pubではないのでエラー
    crate::outermost::middle_secret_function();
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



// insideモジュールが公開だったらどうだろうか？
// -> outermost::inside::secret_function()がエラーになる。この関数はpubではない
// outermostが公開で、insideが非公開ならどうだろうか？
// -> outermost::middle_secret_functionは通るけど
// inner_functionの本体で::outermost::middle_secret_function()を呼び出したらどうだろうか？ 
//  (頭の二つのコロンは、ルートモジュールから初めてモジュールを参照したいということを意味します)
// 大本のoutermostが非公開なのでエラー