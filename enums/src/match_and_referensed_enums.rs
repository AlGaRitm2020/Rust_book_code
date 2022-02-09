#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
    Montana,
    Men,
    Ohao,
    North_Dakota,
    South_Dakota,
    Kanzas,
    Arizona,
    Washington,
    Oregon,
    Nevada,
    Illinoys,
    Penselwanya,
    New_York,
    Florida,
    Luisiana,
    Missuri,
    Boston,
    North_Carolina,
    South_Carolina,
    
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from state {:?}!", state);

            25
        },
    }
    
}


fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}


