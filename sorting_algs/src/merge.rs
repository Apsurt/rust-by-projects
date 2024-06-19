#[path="tools.rs"] mod tools;
#[path="selection.rs"] mod selection;

pub fn sort(mut nums: Vec<i32>) -> Vec<i32> {
    //divide
    if nums.len() > 2 {
        let m = nums.len()/2;
        
        let mut l: Vec<i32> = nums[..m].to_vec();
        let mut r: Vec<i32> = nums[m..].to_vec();
        
        l = sort(l);
        r = sort(r);
        l.extend(&r);
        nums = l.clone();
        
    }
    //merge
    nums = selection::sort(nums);
    return nums;
}