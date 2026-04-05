fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // What's happening here? What we're actually copying is the ptr, the length & the capacity,
    // not the valuye itself, as thats stored in the heap, not the stack.
    // So it we do:
    // println!("{s1}, world!"); we would get an error, this is known as a move
}
