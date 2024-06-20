#[path="tools.rs"] mod tools;

pub fn sort(mut nums: Vec<i32>) -> Vec<i32> {
    while !tools::is_sorted(&nums) {
        for idx in 1..nums.len() {
            if nums[idx-1] > nums[idx] {
                nums.swap(idx-1, idx);
            }
        }
    }
    return nums;
}