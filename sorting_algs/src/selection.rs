#[path="tools.rs"] mod tools;
use std::{thread, time};

pub fn sort(mut nums: Vec<i32>) -> Vec<i32> {
    let ten_millis = time::Duration::from_millis(250);
    for from in 0..nums.len() {
        tools::print_histogram(nums.clone());
        println!("");
        let mut min: i32 = i32::MAX;
        let mut min_idx: usize = 0;
        for idx in from..nums.len() {
            let val = nums[idx];
            if val < min {
                min = val;
                min_idx = idx;
            }
        }
        if from != min_idx {
            nums.swap(from, min_idx);
            thread::sleep(ten_millis);
        }
    }
    tools::print_histogram(nums.clone());
    return nums;
}