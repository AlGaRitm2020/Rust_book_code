fn main() {
    let s1 = String::from("hello");
    let len = calculate_lenght(&s1);
    println!("Len '{}' is {}.", s1, len);
    
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}
