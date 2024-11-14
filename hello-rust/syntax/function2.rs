// the order of functions in Rust is not important
// fn <function-name>(<para1>: <type1>, <para2>: <type2>) -> <return-type> { /* function body. */ }
// <function-name> follows snake_case style
fn is_prime(num: i64) -> bool {
    if num < 2 {
        // return value with the correct type 
        return false;
    }

    // declare a mutable variable - 
    // which can be modified after inited (new value, but same type)
    let mut i = 2;

    // while loop to go throught values 
    while i * i <= num {
        // if n has any divisor (<n), it is not a prime
        if num % i == 0 {
            return false;
        }

        // increase i by 1 for the next loop
        i += 1;
    }

    // we do not need return keyword here
    true
}

// entry point of a Rust program
fn main() {
    // declare a vector - sample input
    let nums = vec![-5, 3, 10, 13, 29];

    // go throught each element in the vector
    for num in nums {
        // if...else statement
        // because the is_prime return a boolean value
        // we can use its return value as the condition
        if is_prime(num) {
            println!("{} is a prime", num);
        } else {
            println!("{} is not a prime", num);
        }
    }
}
