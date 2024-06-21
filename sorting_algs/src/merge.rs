#[path="tools.rs"] mod tools;
#[path="insertion.rs"] mod insertion;

pub fn sort(mut nums: Vec<i32>) -> Vec<i32> {
    //divide
    let m = nums.len()/2;
    if nums.len() > 2 {
        let mut l: Vec<i32> = nums[..m].to_vec();
        let mut r: Vec<i32> = nums[m..].to_vec();
        
        l = sort(l);
        r = sort(r);
        //println!("l: {:?}", l);
        //println!("r: {:?}", r);
        l.extend(&r);
        nums = l.clone();
        
    }
    //merge
    nums = insertion::sort(nums.clone());
    return nums;
}