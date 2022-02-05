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
    let user2 = User {
        email: String::from("second@mail.com"),
        username: String::from("second_user_name"),
        ..user1
    };
}


