fn main() {
    let number = 5;
    if number > 7 {
        println!("The number var is greater than 7");
    } else if number == 7 {
        println!("the value is 7");
    } else {
        println!("The value is lower than 7");
    }
    //we can also try to do the following
    let mut x = 1;
    let result = loop {
        x += 1;
        if x == 10 {
            break x * 2;
        }
    };
    println!("The value of result is {result}");
}
