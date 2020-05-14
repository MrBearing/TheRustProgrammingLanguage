use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     // ゆっくり計算します
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

}
struct Cacher<T, V> 
    where T: Fn(V) -> V, 
          V: Eq+ Hash+Clone+Copy
{
    calculation: T,
    value_cache: HashMap<V,V>,
}


/// ジェネリックな引数を導入することにより、u32以外にも対応できるように変更
impl<T,V> Cacher<T,V > 
    where T:Fn(V) -> V, 
          V: Eq+ Hash+Clone+Copy
{
    fn new(calculation: T) -> Cacher<T,V>{
        let value_cache = HashMap::new();
        Cacher{
            calculation,
            value_cache,
        }
    }
    fn value(&mut self, arg: V) -> V {
        match self.value_cache.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value_cache.insert(v, v);
                v
            },
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {

    // let expensive_result = simulated_expensive_calculation(intensity);
    // let expensive_clojure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)

        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        assert_eq!(v1, 1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }
    #[test]
    fn call_with_different_values_string() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value("str");
        assert_eq!(v1, "str");
        let v2 = c.value("some_text");
       assert_eq!(v2, "some_text");
    }

}