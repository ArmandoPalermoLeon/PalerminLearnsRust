fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    let (s2, len) = calculate_length(s1);
    let size = cal_len(&s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn cal_len(s: &String) -> usize {
    s.len()
}

// Rust is also useful to avoid dangling references, by simply allowing them not to exist.
