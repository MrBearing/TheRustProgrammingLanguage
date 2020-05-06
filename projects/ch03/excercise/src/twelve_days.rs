
pub mod lyrics {
    use numeral::Cardinal;

    pub fn replace_num(num_str : &mut String, find :&str, replace: &str){
        if ! num_str.ends_with(find) {
            panic!("not find {}",find);
        }
        let len = num_str.len();
        let f =  match num_str.rfind(find) {
            Some(i) => i,
            None => len,
        };
        num_str.replace_range(f..,replace);
    }

    pub fn get_ordinal(n :i32) -> String{
        let mut num_str: String = n.cardinal();// like one , two, ...
        if num_str.ends_with(&1.cardinal()) {
            replace_num(&mut num_str, &1.cardinal(), "first");
            return num_str;
        }
        if num_str.ends_with(&2.cardinal()) {
            replace_num(&mut num_str, &2.cardinal(), "second");
            return num_str;
        }
        if num_str.ends_with(&3.cardinal()) {
            replace_num(&mut num_str, &3.cardinal(), "third");
            return num_str;
        }
        if num_str.ends_with("ve") {
            replace_num(&mut num_str, "ve", "fth");
            return num_str;
        }
        if num_str.ends_with("e") {
            replace_num(&mut num_str, "e", "th");
            return num_str;
        }
        if num_str.ends_with("t") {
            replace_num(&mut num_str, "t", "th");
            return num_str;
        }
        if num_str.ends_with("y") {
            replace_num(&mut num_str, "y", "ieth");
            return num_str;
        }
        num_str.push_str("th");
        return num_str;
    }

    pub fn get_lyrics(n :i32)->Option<String>{
        if n < 1 {
            return None;
        }
        if n > 12 {
            return None;
        }
        let mut return_lyric = String::from("On the ");
        let fist_line = " day of Christmas my true love sent to me\n";
        return_lyric = return_lyric + &get_ordinal(n) + fist_line;

        // for i in 0..n as usize {
        //     if n == 1 {
        //         break;
        //     }
        //     return_lyric.push_str(PRESENTS[i]);
        // }
        let p = PRESENTS.len() - n as usize;
        for present in PRESENTS[p..].iter(){
            return_lyric.push_str(present)
        }

        return_lyric += "a partridge in a pear tree\n";
        Some(return_lyric)
    }
    
    const PRESENTS: [&str; 12] = [
        "twelve drummers drumming,\n",
        "eleven pipers piping,\n",
        "ten lords a-leaping,\n",
        "nine ladies dancing,\n",
        "eight maids a-milking,\n",
        "seven swans a-swimming,\n",
        "six geese a-laying,\n",
        "five golden rings,\n",
        "four calling birds,\n",
        "three French hens,\n",
        "two turtle doves,\n",
        "and"
        ];

}


#[cfg(test)]
mod tests {
    use super::lyrics;

    #[test]
    fn test_ordinal() {
        assert_eq!(lyrics::get_ordinal(1),"first");
        assert_eq!(lyrics::get_ordinal(2),"second");
        assert_eq!(lyrics::get_ordinal(3),"third");
        assert_eq!(lyrics::get_ordinal(4),"fourth");
        assert_eq!(lyrics::get_ordinal(5),"fifth");
        assert_eq!(lyrics::get_ordinal(6),"sixth");
        assert_eq!(lyrics::get_ordinal(7),"seventh");
        assert_eq!(lyrics::get_ordinal(8),"eighth");
        assert_eq!(lyrics::get_ordinal(9),"ninth");
        assert_eq!(lyrics::get_ordinal(10),"tenth");
        assert_eq!(lyrics::get_ordinal(11),"eleventh");
        assert_eq!(lyrics::get_ordinal(12),"twelfth");
        assert_eq!(lyrics::get_ordinal(20),"twentieth");
        assert_eq!(lyrics::get_ordinal(101),"one hundred first");
        assert_eq!(lyrics::get_ordinal(511),"five hundred eleventh");
    }

}