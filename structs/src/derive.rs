#[derive(Debug)]
struct Rectangle  {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50}; 

    println!("rectangle is {:#?}",&rect1);
}


