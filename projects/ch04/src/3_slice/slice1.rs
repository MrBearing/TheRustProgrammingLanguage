fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }
  &s[..]
}

fn main() {
  let mut s = String::from("hello world");
  println!("{}", s);
  let word = first_word(&s);
  println!("{}", word);
  s.clear(); // this empties the String, making it equal to ""
}