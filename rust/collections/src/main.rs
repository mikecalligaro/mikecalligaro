use rand::Rng;
use std::collections::HashMap;

fn main() {
    averages();
}

fn averages() {
    const NUM_VALUES: i32 = 25;
  
    let mut v = get_vector(NUM_VALUES);

    let mean = mean(&v);
    println!("mean = {}", mean);

    let median = median(&mut v);
    println!("median = {}", median);

    let mode = mode(&v);
    println!("mode = {}", mode);

    println!("vector = {:?}", v);
}

fn get_vector(num: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    let mut index = 0;
    while index < num {
        v.push(rand::thread_rng().gen_range(1, 101));
        index += 1;
    };
    
    println!("vector = {:?}", v);

    v
}

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    let mut count = 0;

    for i in v {
        sum += i;
        count += 1;
    }

    f64::from(sum) / f64::from(count)
}

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();

    v[v.len() / 2]
}

fn mode(v: &Vec<i32>) -> i32 {
    
    let mut map = HashMap::new();

    let mut max_val = 0;
    let mut max_count = 0;
    
    for val in v {
        let count = map.entry(val).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count = *count;
            max_val = *val;
        }
    }

    // No real mode if every value is unique
    if max_count == 1 {
        max_val = -1;        
    }

    max_val
}