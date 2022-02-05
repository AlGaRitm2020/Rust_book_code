fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s[..]); // return index of the end of first word

    println!("First word is {}", word);
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..] 
}

