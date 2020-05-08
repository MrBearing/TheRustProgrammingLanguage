fn is_vowel(c : char) -> bool {
     c == 'a' ||  c == 'e' || c == 'i' || c == 'o' || c == 'u'
    || c == 'A' || c == 'E'|| c == 'I' || c == 'O' || c == 'U'
}

pub fn get_pig_latin_word(word: &str) -> String{
    // 母音が頭文字の場合
    if word.starts_with(is_vowel){
        return word.to_string() + &String::from("hay");
    }

    let mut word_vec :Vec<char> = word.clone().chars().collect();
    // 大文字が頭にあった場合
    if word.starts_with(char::is_uppercase) {
        // 1文字目を小文字　2文字目を大文字に
        word_vec[0] = word_vec[0].to_ascii_lowercase();
        word_vec[1] = word_vec[1].to_ascii_uppercase();
    }
    let s :String = word_vec.into_iter().collect();
    let word = &s;
    
    // 頭1文字子音切り出し + 末尾にay
    [ &word[1..],&word[..1],"ay"].concat().to_string()
}

pub fn translate(text: &String) -> String {
    let mut translated_string = String::new();

    for mut word_option in text.split_ascii_whitespace(){
        let mut terminal : Option<&str> = None;

        //'.','?','!',':',','で終わっていた場合の処理
        if word_option.ends_with(".") {
            // word_optionから末尾を取り除く
            println!("word_option :: {:?}",word_option);
            let end = word_option.len()-1;
            let t = &word_option[end..];
            word_option = &word_option[..end];
            println!("t :: {:?}",t);
            terminal = Some(t);
        }

        let word = get_pig_latin_word(word_option);
        translated_string += &word;
        match terminal {
            Some(e) => {
                println!("e :: {:?}",e);
                translated_string.push_str(e)
            },
            None => translated_string.push_str(" "),
        }
    }

    translated_string
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        let ascii = 'a';
        let non_ascii = '❤';
        let q = '?';
        
        assert!(ascii.is_ascii());
        assert!(q.is_ascii());
        assert!(!q.is_ascii_alphabetic());
        assert!(!q.is_alphabetic());
        assert!(!non_ascii.is_ascii());
    }

    #[test]
    fn test_string() {
        assert_eq!(String::from("This is a pen."),String::from("This is a pen."));
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(&String::from("I don't understand.")),
                              String::from("Ihay on'tday understandhay."));
        assert_eq!(translate(&String::from("This is a pen.")),
                              String::from("Histay ishay ahay enpay."));
        assert_eq!(translate(&String::from("Good morning")),
                              String::from("Oodgay orningmay "));
    }
    #[test]
    fn test_get_pig_latin_word(){
        assert_eq!(get_pig_latin_word("I"),"Ihay");
        assert_eq!(get_pig_latin_word("understand"),"understandhay");
        assert_eq!(get_pig_latin_word("don't"),"on'tday");
        assert_eq!(get_pig_latin_word("good"),"oodgay");
        assert_eq!(get_pig_latin_word("morning"),"orningmay");
        assert_eq!(get_pig_latin_word("Hello"),"Ellohay");
        assert_eq!(get_pig_latin_word("first"),"irstfay");
        assert_eq!(get_pig_latin_word("First"),"Irstfay");
        assert_eq!(get_pig_latin_word("apple"),"applehay");
        assert_eq!(get_pig_latin_word("Apple"),"Applehay");
    }
}


