use std::io;
use rand::Rng;
use std::cmp::Ordering;

//custom type for operating numbers from 1 to 100
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret number is {}", secret_number);
    loop{
        println!("Enter your number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Reading the string failed");
        

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };
        
        
        println!("You was guess: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Secret number is bigger than your"),
            Ordering::Greater => println!("Secret number is smaller than your"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
