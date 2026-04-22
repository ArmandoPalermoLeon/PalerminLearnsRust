// Let's recall the previously rectangle code using struct, we did the #[derive(debug)]
// We're not goung to do something different, we will use methods, this time we'll tackle the
// method syntax
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    // Impl block for rectangle, it'll be associated to the function
    fn area(&self) -> u32 {
        // we take this parameter safe
        self.width * self.heigth // dot followed by the method name
    }
}
// So what is self and why are we using it?
// &self for not getting the ownership of the value, just read the data
fn main() {
    let rect1 = Rectangle {
        width: 30,
        heigth: 30,
    };
    println!("The area of the rectangle is {}", rect1.area());
}

// If we wanted to change the instance that we've called the method on, we can use:
// &mut self as the first parameter
// It is not normal tho, to have a method that takes ownership of the instance by just using self,
// only used when the method transforms self into something else
// They're good becasue they provide a way for organization, we could also do:

impl Rectangle1 {
    fn width(&self) -> bool {
        self.width > 0 // this will return a boolean value if it's more than 0.
    }
}

