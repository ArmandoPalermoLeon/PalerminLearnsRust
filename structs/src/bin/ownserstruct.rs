struct Users {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u32,
}

fn main() {
    let user1 = Users {
        active: true,
        username: "something",
        email: "something@gmail.com",
        sign_in_count: 1,
    };
}

// This code will give a compiler error, especially about lifetimes. We will target lifestyles
// further into the book, but they could be solved in another way.
// We could solve them by adding a 'a before it, thats a lifetime.
