struct Users {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn main() {
    let mut user = Users {
        active: true,
        username: String::from("pablito123"),
        email: String::from("pablito123@gmail.com"),
        sign_in_count: 1,
    };
}
