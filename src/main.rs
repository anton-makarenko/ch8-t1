use std::io;
use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let ints: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse value"))
        .collect();
    let mut sum = 0.0;
    for i in &ints {
        sum += *i as f64;
    }
    let mean = sum / ints.len() as f64;
    println!("mean: {}", mean);

    let mut ints_sorted = ints.to_vec();
    ints_sorted.sort();
    let ints_sorted: Vec<i32> = ints_sorted.to_vec();
    let middle = ints_sorted.len() / 2;
    let median = ints_sorted[middle];
    println!("median: {}", median);

    let mut counts = HashMap::new();
    for i in &ints {
        counts.insert(*i, 0);
    }
    for i in &ints {
        *counts.get_mut(i).unwrap() += 1;
    }
    for (key, value) in &counts {
        println!("{}: {}", key, value);
    }
}
