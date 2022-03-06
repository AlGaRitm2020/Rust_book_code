use std::fs::File;
use std::io::ErrorKind;

fn main() {
    
    // unwrap return containing of Result if there is no error, else raise panic
    let f = File::open("hello.txt").unwrap();

    // expect do the same, but you can write panic message
    let f = File::open("hello.txt").expect("Problem with opening file hello.txt");

}

