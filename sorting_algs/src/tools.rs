use rand::prelude::*;

pub fn print_histogram(nums: Vec<i32>) {
    let max = *nums.iter().max().unwrap() as f64;
    let min = *nums.iter().min().unwrap() as f64;
    
    for num in nums {
        let t: f64 = (num as f64-min)/max;
        
        let height = min + (max - min + 1.0) * t;
        let line: String = "█".repeat(height as usize);
        println!("{}", line);
    }
}

pub fn random_vector(n: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    
    let mut nums: Vec<i32> = (1..n+1).collect();
    nums.shuffle(&mut rng);
    return nums
}