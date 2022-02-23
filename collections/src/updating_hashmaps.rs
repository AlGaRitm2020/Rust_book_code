fn main() {
    use std::collections::HashMap;
    
    // replace old value with specific key 
    let mut scores = HashMap::new();
    scores.insert("blue".to_string(), 10);
    scores.insert("blue".to_string(), 50);

    println!("{:?}", scores);

    // using entry to add only if key is empty
    scores.entry("yellow".to_string()).or_insert(50);
    scores.entry("blue".to_string()).or_insert(30);

    println!("{:?}", scores);


    // updating based on previous value
    let text = "hello world beautiful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);


}
