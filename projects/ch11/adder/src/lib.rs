#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32{
    a + 2 //not 3
}

pub fn greeting(name: &str) -> String {
    // format!("Hello !") bug
    format!("Hello {}!",name)
}

#[allow(unused_variables)]
pub struct Guess {
    pub value: u32,
}

impl Guess { 
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            //予想値は、1以上でなければなりませんが、{}でした
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            //予想値は100以下でなければなりませんが、{}でした
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }
        Guess{
            value
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn less_than_1() {
        Guess::new(0);
    }


    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result);
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4,add_two(2));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{ length: 8, width: 7};
        let smaller = Rectangle{length: 5, width: 1};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let lager = Rectangle{ length: 8, width: 7};
        let smaller = Rectangle{length: 5, width: 1};

        assert!(!smaller.can_hold(&lager));

    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn another() {
        //panic!("Make this test fail");
    }
}
