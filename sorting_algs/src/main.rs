mod tools;
mod selection;

fn main() {
    let nums = tools::random_vector(40).clone();
    selection::sort(nums.clone());
}
