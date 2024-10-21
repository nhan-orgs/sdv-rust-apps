use std::cmp;

const MIN_LEN: usize = 2;

#[derive(Debug)]
enum Error {
    TooShortVector(usize),
}

fn main() {
    // Example of too short vector
    // let nums = vec![2]; 
    
    // Example input
    let nums = vec![2, 3, -64, 2, 66]; 

    match calculate_maximum_gap(&nums) {
        Ok(maximum_gap) => 
            println!(
                "Maximum gap is: {}", 
                maximum_gap
            ),
        Err(Error::TooShortVector(len)) => {
            println!(
                "Error: Length must be at least {}.",
                MIN_LEN
            );
            println!(
                "Current length of your vector: {}", 
                len);
        }
    }
}

fn calculate_maximum_gap(nums: &[i32]) 
    -> Result<i32, Error> {
    let nums_len = nums.len();
    if nums_len < MIN_LEN {
        return Err(Error::TooShortVector(nums_len));
    }

    let mut maximum_gap = 0;

    for id in 1..nums_len {
        let gap = (nums[id] - nums[id - 1]).abs();
        maximum_gap = cmp::max(maximum_gap, gap);
    }

    Ok(maximum_gap)
}