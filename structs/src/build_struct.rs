struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
