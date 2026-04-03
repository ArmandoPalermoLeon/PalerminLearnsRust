use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!"); // Generic println!, uses '!' for it to be a macro, a macro is a
                                   // way to generate code 
    let secret_number = rand::thread_rng().gen_range(1..=100); //uses range to have the begin..=end
                                                             //of the numbers we want
    // loop notation, will not stop until it hits the break;
    loop {
        println!("Please input your string");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line"); //Use expect because the
        //read_line returns an enum, a result value which would be Ok or Err, this is in case we
        //get error.
        // Next we will use shadowing, lets us reuse the guess varianle name rather tahn forcing us
        // to create unique variables
        // We convert the string to u32, we use trim for the enter the user puts when he gives the
        // number.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //if match completed its process of converting, then we will have this
                            //ok message,
            Err(_) => continue, //if not, these
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
