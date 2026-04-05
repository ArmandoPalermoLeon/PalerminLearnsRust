// The string type is a very important type in Rust that can teach how ownership works in a basic
// level
fn main() {
    // The next would be a normal string (immutable)
    let s = "Hello";
    // The String type manages how data is allocated on the Heap
    //We can also do
    let mut s = String::from("Hello");
    // What is the difference between these? First one is mutable, that means we can do the
    // following
    s.push_str(", world!");
    println!("{s}");
    // Those 2 deal with memory differently
    // In the case of an string literal, we know the contents at compile time.
    // With the string type, in order to be mutable, we need to allocate an amount of energy in the
    // heap, this leaves us with:
    // The memory must be requested from the memory allocator at runtime, this implies that we also
    // need a way of returning thjis memory to the allocator once we finish
    // What rust does: The memory is automatically returned once the variable that owns it get outs
    // of scope
    // Multiple variables can interact with the same data
    let x = 5;
    let y = x;
    // The integers are fixed values, so it's simple, let's see strings:
    let str = String::from("Value");
    let str1 = str;
    let st = String::from("hello"); // st comes into scope 
    takes_ownership(s); // st valuye moves in the function, so s is not longer valid
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
