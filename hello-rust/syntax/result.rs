fn main() {
    // declare a vector of tuples
    // each tuple is a sample input
    let inputs = vec![(1, 5), (2, 0), (6, 4)];

    // iterate over the vector to get each input tuple
    for input in inputs.iter() {
        // divide(...) returns a Result type
        // we can use `match...` to catch all posible cases
        match divide(input.0, input.1) {
            // if divide success
            Ok(res) => println!("{} / {} = {}", input.0, input.1, res),
            Err(e) => println!("Error while trying to calculate {} / {}. {}", input.0, input.1, e)
        }
    }
}

// we can not divide a number by 0
// so this function should return an error is b is 0
// the Result<T,E> type will help us do that
// T is type of expected return value
// E is type of error if the function execute failed
fn divide(a: i32, b: i32) -> Result<i32, String>  {
    if b == 0 {
        // we can remove eturn keywork here
        // Err(...) wraps the error message (a String)
        // it means the division is failed
        Err("b must be a non-zero integer!".to_string())
    } else {
        // Ok(...) wraps the expected value (not an error :v)
        Ok(a / b)
    }
}
