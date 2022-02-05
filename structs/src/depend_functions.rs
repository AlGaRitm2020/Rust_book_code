#[derive(Debug)]
struct Rectangle  {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
    
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50}; 
    let square1 = Rectangle::square(30);

    println!("rect1 is {:#?}",rect1);
    println!("square1 is {:#?}", square1);

}


