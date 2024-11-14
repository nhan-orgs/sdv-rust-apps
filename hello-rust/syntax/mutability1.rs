fn main() {
    // declare a variable
    // by default, it is an immutable one
    // it means you can not modify it value after init
    let num = 1;
    
    // Rust compiler will raises an error if you try to modify num
    // because it is an immutable variable
    // error: cannot assign twice to immutable variable
    num = num + 1;

    println!("num = {}", num);
}

// Hint:   
// `let mut num = 1;` will make num a mutable variable
// then, you can change it value