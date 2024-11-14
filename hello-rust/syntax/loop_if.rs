fn main() {
    // declare a variable
    // the default type of 10 is i32, so n is i32
    // we will talk more about it in Immutability example
    let range = 10;

    // a for loop to go through all values (0, 1, 2 ... n)
    for num in 0..=range {
        // an if-else statement
        // do not need (...) around the condition
        // the condition must be a boolean value (true/false)
        if num % 2 == 0 {
            println!("{} is an even", num);
        } else {
            println!("{} is an odd", num);
        }
    }
}

