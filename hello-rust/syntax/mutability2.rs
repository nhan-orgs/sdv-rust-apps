// this help us print the Reactangle in Debug mode
#[derive(Debug)]
struct Reactangle {
    width: f32,
    height: f32
}

fn main() {
    // if your remove `mut` keyword, you can not change value of `rectangle`
    // and the data inside it (height, width fields) 
    let mut rectangle = Reactangle {
        width: 3.2,
        height: 5.8
    };

    println!("{:?}", rectangle);

    rectangle.width += 2.0;
    rectangle.height -= 0.5;

    println!("{:?}", rectangle);
}
