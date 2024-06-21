mod tools;
mod merge;
mod selection;
mod insertion;
mod shell;

use std::time::Instant;

fn main() {

    let mut nums = tools::random_vector(10_000).clone();
    let now = Instant::now();
    nums = merge::sort(nums);
    println!("merge: {} ms", now.elapsed().as_millis());
    println!("{}\n", tools::is_sorted(&nums));
    
    let mut nums = tools::random_vector(10_000).clone();
    let now = Instant::now();
    nums = selection::sort(nums);
    println!("selection: {} ms", now.elapsed().as_millis());
    println!("{}\n", tools::is_sorted(&nums));
    
    let mut nums = tools::random_vector(10_000).clone();
    let now = Instant::now();
    nums = insertion::sort(nums);
    println!("insertion: {} ms", now.elapsed().as_millis());
    println!("{}\n", tools::is_sorted(&nums));
    
    let mut nums = tools::random_vector(10_000).clone();
    let now = Instant::now();
    nums = shell::sort(nums);
    println!("shell: {} ms", now.elapsed().as_millis());
    println!("{}\n", tools::is_sorted(&nums));
    
}
