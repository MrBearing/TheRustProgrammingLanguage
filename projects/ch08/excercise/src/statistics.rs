use std::collections::HashMap;

/// 平均
pub fn mean(data: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for datum in data.iter() {
        sum += datum;
    }
    sum as f64 / data.len() as f64
}

/// 中央値
pub fn median(data: &Vec<i32>) -> i32 {
    let mut clone_data = data.clone();
    clone_data.sort();
    let x :f64 = clone_data.len() as f64 /2.0_f64.floor();
    clone_data[x as usize]
}

/// 最瀕値
pub fn mode(data: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32,i32> = HashMap::new();
    let mut max_count = 0;
    let mut mode: &i32 = &0;
    for datum in data.iter(){
        let counter = counts.entry(*datum).or_insert(0);
        *counter += 1;
        if *counter > max_count{
            max_count = *counter;
            mode = datum;
        }
    }
    println!("{:?}",counts);
    println!("{:?}",data);
    println!("mode {:?}", mode);
    *mode
}


// 11章を参照するとユニットテストはここが良いみたい
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    

    #[test]
    fn test_mean() {
        let array = vec![1,2,3,4,5,6,7,8,9,10];
        assert_eq!(mean(&array), 5.5);
    }

    #[test]
    fn test_median() {
        let mut rng = rand::thread_rng();
        let mut array = vec![0,1,2,3,4,5,6,7,8,9,10];
        array.shuffle(&mut rng);
        assert_eq!(median(&array),5); 
    }

    #[test]
    fn test_mode() {
        let mut array = Vec::new();
        for i in 0..=10 as i32 { 
            for _ in 0..=i as i32 {
                array.push(i);
            }
        }
        let mut rng = rand::thread_rng();
        array.shuffle(&mut rng);
        assert_eq!(mode(&array),10);
    }


}