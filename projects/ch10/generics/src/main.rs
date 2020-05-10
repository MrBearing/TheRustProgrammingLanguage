pub fn largest_i(list: &[i32]) -> i32{
    let mut largest = list[0];
    
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest_num = number_list[0];

    for number in number_list {
        if number > largest_num {
            largest_num = number;
        }
    }

    // 最大値は{}です
    println!("The largest number is {}", largest_num);
    assert_eq!(largest_num, 100);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_i() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_i(&number_list);
        println!("The largest number is {}", result);
        assert_eq!(result, 100);
    
        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    
        let result = largest_i(&number_list);
        println!("The largest number is {}", result);
        assert_eq!(result, 6000);
    }



    #[test]
    fn test_generics_largest() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);
        assert_eq!(result,100);
        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);
        assert_eq!(result,'y');
    }

}
