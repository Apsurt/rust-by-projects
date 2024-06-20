#[path="tools.rs"] mod tools;
#[path="selection.rs"] mod selection;

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
    if nums.len() == 1 {
        return nums;
    }
    else if nums.len() == 2 {
        if nums[0] > nums[1] {
            nums.swap(0, 1);
        }
        return nums;
    }
    else {
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
}