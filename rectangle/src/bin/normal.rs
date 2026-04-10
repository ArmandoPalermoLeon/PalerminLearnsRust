fn main() {
    let width = 50;
    let height = 30;
    let result = area(width, height);
    println!("The result is: {result}");
}

fn area(w: u32, h: u32) -> u32 {
    let result = w * h;
    result
}
