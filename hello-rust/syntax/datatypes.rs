use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("\nexample of type {}:", type_name::<T>());
}

fn main() {
    // Scalar types
    println!("SCALAR TYPES");

    let default_integer = 10; // i32
    print_type_of(&default_integer);
    println!("default_integer = {}", default_integer);

    let integer: i8 = -5;
    print_type_of(&integer);
    println!("integer = {}", integer);

    let unsigned_integer: u8 = 10;
    print_type_of(&unsigned_integer);
    println!("unsigned_integer = {}", unsigned_integer);

    let size: usize = 10;
    print_type_of(&size);
    println!("size = {}", size);

    let float: f32 = 3.0;
    print_type_of(&float);
    println!("float = {}", float);

    let default_float = 3.0; // f64
    print_type_of(&default_float);
    println!("default_float = {}", default_float);

    let heart_eyed_cat = '😻'; // char
    print_type_of(&heart_eyed_cat);
    println!("heart_eyed_cat = {}", heart_eyed_cat);

    // Compound types
    println!("\n--------");
    println!("COMPOUND TYPES");

    let tuple: (i32, f64, u8) = (500, 6.4, 1); // tuple
    print_type_of(&tuple);
    println!("tuple = {}, {}, {}", tuple.0, tuple.1, tuple.2);

    let array = [1, 4, 5];
    print_type_of(&array);
    println!("array = {}, {}, {}", array[0], array[1], array[2]);
}
