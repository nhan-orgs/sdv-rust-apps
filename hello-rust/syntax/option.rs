use std::vec::Vec;

fn main() {
    let nums = vec![0, 1, 4, 6, 5];
    println!("nums = {:?}", nums);
    
    let mut target;
    
    target = 6;
    // &nums means we pass a reference of nums into find_index_of_value()
    // match statement is used to catch all possible cases (of the Option<usize>)
    match find_index_of_value(&nums, target) {
        None => println!("{} does not exist in {:?}", target, nums),
        Some(index) => println!("{} is located at {}", target, index)
    }

    target = -5;
    match find_index_of_value(&nums, target) {
        None => println!("{} does not exist in {:?}", target, nums),
        Some(index) => println!("{} is located at {}", target, index)
    }
}

// Option<T> has two variants:
// None, to indicate failure or lack of value, and
// Some(value), a tuple struct that wraps a value with type T.
// 1. in this case, the target may not exist in the vector nums, 
// so we use Option to manage it
// 2. index in Rust has type `usize`
// so the datatype inside Option is usize
// 3. we only need to access the value inside vector nums and don't modify it
// so we only need a reference (&)
fn find_index_of_value(nums: &Vec<i32>, target: i32) -> Option<usize> {
    // .iter() iterates over elements of type &i32
    // .enumerate() returns (index, value)
    for (id, num) in nums.iter().enumerate() {
        // so type of num is &i32
        // we need to use dereference operator (*) to get i32 value
        if *num == target {
            return Some(id);
        }
    }

    // target is not found in nums
    None
}