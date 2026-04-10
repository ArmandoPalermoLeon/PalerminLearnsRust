struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    println!("The value of the rectangle is {}", area(&rect1));
    //We can't do area(rect1), that would be a compilation error. The curly brackets tell println!
    //to use formatting known as Display, structs are not definitive types, so they cant do that.
    //We need to do it that way
}
