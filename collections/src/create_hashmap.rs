fn main() {
    use std::collections::HashMap;
    
    // replace old value with specific key 
    let mut scores = HashMap::new();
    scores.insert("blue".to_string(), 10);
    scores.insert("blue".to_string(), 50);

    println!("{:?}", scores);

}
