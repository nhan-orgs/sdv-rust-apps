fn main() {
    // value of the string is stored in heap
    // now, first is its owner
    let first = String::from("this-is-a-string");
    println!("first = {}", first);

    // second is currently the owner of that string
    // you can not use `first` anymore
    let mut second = first; 
    second.push(':');

    println!("second = {}", second);

    // error!!!
    // println!("first = {}", first);
}