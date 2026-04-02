use std::io; // Parh of the std, for input, output

fn main() {
    println!("Guess the number: ");
    println!("Please input your guess."); //Normal prints
    let mut guess = String::new(); // create mutable variable called guess, we could also make let
                                   // apple=5, that would be immutable
                                   // The :: syntax means it is an associated function of the
                                   // string type.
                                   // An associated function is a function thats implemend on a
                                   // type, in this case, is String.
    io::stdin().read_line(&mut guess).expect("failed to read line"); // the stdin function returns
                                                                    // an instance of
                                                                    // std::io::stdin, which is a
                                                                    // type that represents a
                                                                    // handle to the standard
                                                                    // input
    //The readline(&mut guess) calls the method, passing &mut as a parameter/argument (the
    //reference) to read_line 
    //Take whatever the user types into the standard input & append that into a string, the string
    //argument therefore needs to be mutable
    //read_line puts what tge user enters into the string, but it also returns a Result value,
    //result is an enum, whioch is a type that can be in one of multiple possible states, we call
    //this a variant.
    // So basically, it returns a result type, this could be Ok and Err, if err, we should have
    // error handling with the except, it will create a Rust Warnining nif we dont add it.
    println!("You guessed: {guess}");
}
