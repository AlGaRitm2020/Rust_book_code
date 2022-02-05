struct Rectangle  {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50}; 

    println!(
        "Square of rectangle is {} square pixels",
        area(&rect1)
        );
}

fn area(rect: &Rectangle) -> u32 {
   rect.width * rect.height 
}

