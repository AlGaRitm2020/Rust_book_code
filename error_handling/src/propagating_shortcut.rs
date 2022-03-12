use std::fs::File;
use std::io::{self, Read};
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    
    // you can do this more simply
    File::open("hello.txt")?.read_to_string(&mut s)?;


    Ok(s)

    // even more simply
    //fs::read_to_string("hello.txt")

}

fn main() {
    read_username_from_file();
}
