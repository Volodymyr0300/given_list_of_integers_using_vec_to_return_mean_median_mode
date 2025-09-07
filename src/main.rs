use std::collections::HashMap;

#[derive(Debug)]
struct Stats {
    mean: f64,
    median: f64,
    mode: i32,
}

fn mean (nums: &[i32]) -> f64 {
    let sum: i32 = nums.iter().sum();
    sum as f64 / nums.len() as f64
}

fn median (nums: &[i32]) -> f64 {
    let mut sorted = nums.to_vec();
    sorted.sort();

    let len = sorted.len();
    if len % 2 == 1 {
        sorted[len/2] as f64
    } else {
        let left_of_middle_element = sorted[len / 2 - 1];
        let right_of_middle_element = sorted[len / 2];
        (left_of_middle_element as f64 + right_of_middle_element as f64) /2.0
    }
}
    
fn mode (nums: &[i32]) -> i32 {
    let mut counts: HashMap<i32, usize> = HashMap::new();

    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|&(_value, count)| count)
        .map(|(value, _count)| value)
        .unwrap()
}

fn compute_stats(nums: &[i32]) -> Option<Stats> {
    if nums.is_empty() {
        return None;
    }
    Some(Stats {
        mean: mean(nums),
        median: median(nums),
        mode: mode(nums),
    })
}

fn main() {
    let data = vec![5,1,2,2,5,4,3,1,4,5,5];
    println!("Mean: {}", mean(&data));
    println!("Median: {}", median(&data));
    println!("Mode: {}", mode(&data));
    
    match compute_stats(&data) {
        Some(stats) => {
            println!("Mean: {}", stats.mean);
            println!("Median: {}", stats.median);
            println!("Mode: {}", stats.mode);
            println!("Debug: {:?}", stats);
        }
        None => {
            println!("The list is empty");
        }
    }
}