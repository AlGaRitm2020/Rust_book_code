pub trait Summary {
    fn summarize(&self) -> String {
        "Read more...".to_string()
    }
}



pub fn notify(item: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {}", item.summarize());
}


pub fn notify3(item: &(impl Summary + Display)) {}

pub fn notify4<T: Summary + Display>(itme: &T) {}


// you can make this defenition clearer

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}



