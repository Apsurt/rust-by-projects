#[path="tools.rs"] mod tools;

pub fn sort(mut nums: Vec<i32>) -> Vec<i32> {
    for from in 0..nums.len() {
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
        }
    }
    return nums;
}