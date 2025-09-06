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
    

fn main() {
    let data = vec![5,1,2,2,5,4,3,1,4,5,5];
    println!("Mean: {}", mean(&data));
    println!("Median: {}", median(&data));
}