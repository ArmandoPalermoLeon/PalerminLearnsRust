#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let Rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let Rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let Rect3 = Rectangle {
        width: 60,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", Rect1.can_hold(&Rect2));
    println!("Can rect1 gold rect3? {}", Rect1.can_hold(&Rect3));
}
