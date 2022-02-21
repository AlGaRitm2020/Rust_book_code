fn main() {
    let v: Vec<i8> = vec![1, 2, 3, 4, 5];

    let third: &i8 = &v[2];

    println!("Third element is {}", third);

    match v.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("Third element doesn't exist"),
    }
}
