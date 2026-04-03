fn main() {
    println!("Hello, world!");
    another_function();
    a_value(32);
    print_labeled_measurement(5, 'h');
    let mut z = five();
    println!("The value of z is {z}");
    println!("If we add one...");
    z = plus_one(z);
    println!("Now the value is: {z}");
}

fn another_function() {
    println!("Another function oh yeah");
}

// We can also put them with certain parameters
fn a_value(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}
//We can also do the following
fn five() -> i32 {
    5
}
// we can even do this
fn plus_one(z: i32) -> i32 {
    z + 1
}
