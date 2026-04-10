fn main() {
    let t = (30, 50);
    println!("The area of the rectangle is {}", area(t));
}

fn area(dimensiones: (u32, u32)) -> u32 {
    dimensiones.0 * dimensiones.1
}
