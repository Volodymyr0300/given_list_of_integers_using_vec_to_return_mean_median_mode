use std::collections::HashMap;

#[derive(Debug)]
struct Stats {
    mean: f64,
    median: f64,
    mode: i32,
}

fn mean (nums: &[i32]) -> f64 {
    let mut sum: i32 = 0;
    for num in nums {
        sum += num;
    }
    sum as f64 / nums.len() as f64
}

fn median (nums: &Vec<i32>) -> f64 {
    let mut sorted = nums.clone();
    sorted.sort();

    let len = sorted.len();

    if len % 2 == 1 {
        sorted[len/2] as f64
    } else {
        let left = sorted[len / 2 - 1]; // left_of_middle_element
        let right = sorted[len / 2]; // right_of_middle_element
        (left as f64 + right as f64) /2.0 // average of the two middle elements
    }
}
    
fn mode (nums: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, usize> = HashMap::new();

    for num in nums {
        let counter = counts.entry(*num).or_insert(0);
        *counter += 1;
    }

    let mut max_value = nums[0];
    let mut max_count = 0;

    for (value, count) in &counts {
        if *count > max_count {
            max_value = *value;
            max_count = *count;
        }
    }

    max_value
}

fn compute_stats(nums: &Vec<i32>) -> Option<Stats> {
    if nums.is_empty() {
        return None;
    }

    let result = Stats {
        mean: mean(nums),
        median: median(nums),
        mode: mode(nums),
    };

    Some(result)
}

fn main() {
    let data = vec![5,1,2,2,5,4,3,1,4,5,5];

    println!("Mean: {}", mean(&data));
    println!("Median: {}", median(&data));
    println!("Mode: {}", mode(&data));
    
    match compute_stats(&data) {
        Some(stats) => {
            println!("Stats struct: {:?}", stats);
            println!("Mean: {}", stats.mean);
            println!("Median: {}", stats.median);
            println!("Mode: {}", stats.mode);
        }
        None => {
            println!("The list is empty");
        }
    }
}