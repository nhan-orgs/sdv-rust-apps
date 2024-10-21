const FIZZBUZZ: &str = "FizzBuzz";
const FIZZ: &str = "Fizz";
const BUZZ: &str = "Buzz";

fn main() {
    fizz_buzz(30);
}

fn fizz_buzz(n: u8) {
    for i in 1..=n {
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