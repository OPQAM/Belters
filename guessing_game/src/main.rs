use std::io;                                        // the io (input/output) library
                                                    // which is part of the standard library (std)

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();                  // 'mut' allows for a variable to be mutable
                                                    // 'new' is an assocuated function - a function
                                                    // implemented on a type, in this case, a
                                                    // String

    io::stdin()
        .read_line(&mut guess)                      // Where to store the user input (guess)
        .expect("Failed to read line");             // handling potential failures

    println!("You guessed: {guess}");
}
