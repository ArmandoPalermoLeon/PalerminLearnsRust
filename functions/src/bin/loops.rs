fn main() {
    let mut counter = 0;
    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;
        loop {
            print!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End count = {counter}");
    // we can also do while same as c++
    let mut value = 10;
    while value > 5 {
        println!("{value}");
        value -= 1;
    }
    // We can do foor loops in the same for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{element}");
    }
    println!("That would be loops");
}
