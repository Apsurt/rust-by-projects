use rand::{seq::SliceRandom, thread_rng};
use std::time::Instant;

mod radix;

fn main() {
    let from = 0;
    let to = 10_000_000;
    let mut nums: Vec<usize> = (from..to).collect();
    nums.shuffle(&mut thread_rng());
    let now = Instant::now();
    let nums = radix::sort(&nums);
    println!("{} ms", now.elapsed().as_millis());
    assert!(nums == Vec::from_iter(from..to));
}
