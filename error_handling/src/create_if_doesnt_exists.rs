use std::fs::File;
use std::io::ErrorKind;

fn main() {

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() { 
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem with creating file: {:?}", error),
            },
            other_error => panic!("Problem with opening file: {:?}", other_error),
            
        },


    };
}

