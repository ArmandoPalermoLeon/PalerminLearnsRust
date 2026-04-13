#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30*scale),
        heigth: 50,
    }
    dbg!(&rect1)
}

// What does dbg! does?
// Is a macro '!' that prints to the standard error console stream (stdrr), this prints to the
// stanratd output console stream (stdout)
