struct Users {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

struct AlwaysEqual;

fn build_user(email: String, username: String) -> Users {
    Users {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user = Users {
        active: true,
        username: String::from("pablito123"),
        email: String::from("pablito123@gmail.com"),
        sign_in_count: 1,
    };
    let user2 = Users {
        active: user.active,
        username: user.username,
        email: user.email,
        sign_in_count: user.sign_in_count,
    };
    let subject = AlwaysEqual;
}
