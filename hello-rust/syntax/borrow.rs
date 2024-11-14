fn main() {
    // owner is the owner of the string "this-is"
    let mut owner = String::from("this-is-a-string");
    println!("owner = {}", owner);

    // an immutable borrowing
    // a immutable reference is passed into immutable_action
    // it can access value inside owner
    immutable_action(&owner);
    println!("After immutable_action: owner = {}", owner);

    // a mutable borrowing
    // a mutable reference is passed into mutable_action
    // it can access anf modify the value inside owner
    mutable_action(&mut owner);
    println!("After mutable_action: owner = {}", owner);
}

fn mutable_action(value: &mut String) {
    println!("mutable_action...");

    // you can modify value inside a mutable reference
    // value of the owner will be changed, too
    value.push_str("-hehe");
    println!("value: {}", value);
}

fn immutable_action(value: &String) {
    println!("immutable_action...");

    // error: `value` is an immutable reference, 
    // so the data it refers to cannot be borrowed as mutable
    // value.push_str("-hjhj");
    println!("value: {}", value);
}