pub trait Summary {
    fn summarize(&self) -> String {
        "Read more...".to_string()
    }
}


fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "hourse_ebooks".to_string(),
        content: "of cource, as you pobably already known, people",
        reply: false,
        retweet: false,
    }
}

fn main() {}
