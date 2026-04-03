use std::io::{self, Read};

fn main() {
    let mut x = 5;
    println!("The value for x is: {x}");
    x = 6;
    println!("The value of x is {x}");
    // we can also declare constants this way
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("The result for three hours in seconds: {THREE_HOURS_IN_SECONDS}");
    // The next will prove how shadowing works on Rust
    let y = 5;
    let y = y + 1; // if it isnt mut we cant reassign without putting "let"
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }
    println!("The value of y is: {y}");
    // Every value is of a certain data type, which tells rust what kind of data is being
    // specificed so it knows how to work with it, rust is statically typed, so it must known the
    // data type at compile time.
    // A scalar type represents a single value, Rust has four primary scalar types: integers,
    // floating-point numbers, Booleans & chars
    let num: u32 = 104;
    let dec: f32 = 32.34;
    let f: bool = false;
    let z: char = 'Z';
    //for the compound types, Rust has tuples & arrays, to make a tuple:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //Tuples have fixed lengths, they cannot grow or shrink in size once they're declared.
    let tup1 = (400, 3.4, 2);
    let (x, y, z) = tup;
    println!("The value of y is {y}");
    //That was a way to access certain value, we can also use the '.'
    let four_hundred = tup1.0;
    //Arrays are other primitive compound type, also has fixed length and all of them must be the
    //same type.
    let a = [1, 2, 3, 4, 5]; //Different to a vector, a vector grows in size because it resides on the
    //heap
    let b: [i32; 5] = [20, 40, 60, 80, 100];
    // An array is a single chunk of memory of a knwon fixed size that can be allocated on the
    // stack.
    let fourty = b[1];
    // Now let's check it really works properly, let's try & do to check if we can find an invalid
    // index position
    let arr1 = [1, 2, 3, 4, 5];
    println!("Please enter an array index:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
