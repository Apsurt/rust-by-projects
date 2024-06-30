
fn counting_sort(nums: &Vec<usize>, place: usize) -> Vec<usize> {
    let len = nums.len();
    let mut new_nums = vec![0; len];
    let mut bins: Vec<usize> = vec![0; 10];
    
    for i in 0..len {
        let index = nums[i] / place;
        bins[index%10] += 1;
    }
    
    
    for i in 1..10 {
        bins[i] += bins[i-1];
    }
    
    let mut i = len;
    while i > 0 {
        let index = nums[i-1] / place;
        new_nums[bins[index%10]-1] = nums[i-1];
        bins[index%10] -= 1;
        i -= 1;
    }
    
    new_nums
}

pub fn sort(nums: &Vec<usize>) -> Vec<usize>{
    let max = nums.into_iter().max().unwrap();
    let mut place: usize = 1;
    let mut output: Vec<usize> = nums.clone();
    while (max / place) > 0 {
        output = counting_sort(&output, place);
        place *= 10;
    }
    output
}