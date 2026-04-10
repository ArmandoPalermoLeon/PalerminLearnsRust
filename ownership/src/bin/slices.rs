fn main() {
    let s = String::from("Hello");
    slices(&s);
    //we can also generate a slice by doing this:
    let mut str = String::from("value we want");
    let mut value = &str[0..4];
    println!("The slice is {value}");
}

fn slices(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// this would be the implementation
fn first_word_new(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
