// declare constants in Rust
// const <constant-name>: <data-type> = <value>;
// <constant-name> follows the capital style
const FIZZBUZZ: &str = "FizzBuzz";
const FIZZ: &str = "Fizz";
const BUZZ: &str = "Buzz";

fn main() {
    // call the function here
    fizz_buzz(10);
}

// the order of functions in Rust is not important
// fn <function-name>(<para1>: <type1>, <para2>: <type2>) { /* function body goes here */ }
// <function-name> follows snake_case style
fn fizz_buzz(n: u8) {
    // for loop fo thought values (1, 2, ... n)
    for i in 1..=n {
        // long if...else if... statements
        // the condition is a boolean value
        // which can be combined by some boolean statements
        if i % 3 == 0 && i % 5 == 0 {
            println!("{}", FIZZBUZZ);
        } else if i % 3 == 0 {
            println!("{}", FIZZ);
        } else if i % 5 == 0 {
            println!("{}", BUZZ);
        } else {
            println!("{}", i);
        }
    }
}