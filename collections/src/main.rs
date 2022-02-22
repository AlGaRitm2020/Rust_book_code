fn main() {

    let data = "begin containing";

    let s = data.to_string();

    // this method also works in string literal
    let s = "begin containing".to_string();

    let s = String::from("begin containing");

    // you can use different languages
    let hello = "hello".to_string();
    let hello = "Здравствуйте".to_string();
    let hello = "Ola".to_string();

    // adding str literal to end of String value
    let mut s = "foo".to_string();
    s.push_str("bar");

    // you can use string literal after push_str, because this method don't getting own it
    let mut s1 = "foo".to_string();
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // adding a single character to String
    let mut s = String::from("lo");
    s.push('l');

    // concatenating with '+' syntax
    let s1 = "Hello, ".to_string();
    let s2 = "world!".to_string();
    let s2 = s1 + &s2; // note: s1 was moved here and cant use anymore, but s2 can because we use ref to it. Rust using folowwing signature:
    // fn add(self, s: &str) -> String {
    

    // more customisable String formating
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();

    let s = format!("{}-{}-{}", s1, s2, s3);
    // note: format! macrocommand reads more easily and not get own variables


    // this code raise an error, because in Rust String indexies is fobidden
    let s1 = "hello".to_string();
    let h = s1[0];
    // why???
    
    let len = String::from("Hola").len();
    // this len equal 4 bites    

    let len String::from("Здравствуйте").len();
    // but in this case len equal 24, not 12
    

    // so Rust dont need String indexing because it prevent us from problems in delelopment state





}
