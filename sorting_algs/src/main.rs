mod tools;
mod selection;
mod merge;

use std::time::Instant;

fn main() {

    let nums = tools::random_vector(10000).clone();
    let now = Instant::now();
    merge::sort(nums);
    println!("{}", now.elapsed().as_millis());
    
    let nums = tools::random_vector(10000).clone();
    let now = Instant::now();
    selection::sort(nums.clone());
    println!("{}", now.elapsed().as_millis());
}
