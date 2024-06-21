use std::ptr::swap;


pub fn sort(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    let pow_2 = f64::ceil(f64::log2(n as f64)) as i32;
    let mut h: Vec<i32> = (1..pow_2)
            .map(|n| 2_i32.pow(n as u32)-1)
            .collect();
    h.reverse();
    
    let mut sub_parts: Vec<Vec<usize>> = Vec::new(); // Vector of vectors of usize
    for &step in &h {
        let slice: Vec<_> = (0..nums.len()).step_by(step as usize).collect();
        sub_parts.push(slice);
    }
    
    for part in sub_parts {
        let mut is_sorted: bool = false;
        while !is_sorted {
            let mut no_swaps: bool = true;
            for part_idx in 1..part.len() {
                if nums[part[part_idx-1]] > nums[part[part_idx]] {
                    nums.swap(part[part_idx-1], part[part_idx]);
                    no_swaps = false;
                }
            }
            if no_swaps {
                is_sorted = true;
            }
        }
    }
    return nums;
}