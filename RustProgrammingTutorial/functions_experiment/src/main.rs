extern crate chrono;                               // adding date
use std::io;
use chrono::Local;
use std::thread;
use std::time::Duration;                           // to add the time pause
use rand::Rng;

fn main() {

    // table of contents, with While and guessing game


    loop {
        println!("***********************************************");
        println!("**Welcome to the main screen! Make your pick:**");
        println!("**                                           **");
        println!("** 1) Just know you are awesome              **");
        println!("** 2) Get the current date, just because     **");
        println!("** 3) Guessing game!                         **");
        println!("** 0) Exit the program                       **");
        println!("***********************************************");
    
        // date
        let current_datetime = Local::now();

        // Take in user input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line.");
        choice = choice.trim().to_string();             // (1)

        println!("Entered: {}", choice);
        // pause
        thread::sleep(Duration::from_secs(1));

        if choice == "1" {
            println!("It's true! You are an awesome human being."); //WIP
        }
        else if choice == "2" {
            println!("Current date and time: {}", current_datetime);
        }
        else if choice == "3" {
            println!("Yey! A guessing game. WooHoo!\nGive me a number from 1 to 10:");
            let mut number_guess = String::new();
            io::stdin().read_line(&mut number_guess).expect("Failed to read line.");
            let number_guess: i8 = number_guess.trim().parse().unwrap();
            guessing_game(number_guess);
            break;

        }
        else if choice == "0" {
            println!("Alright. See you soon!");
            break;
        }
        else
        {
            println!("I don't really understand what you wrote. Try again.");
        }
    }
}
// guesser here
fn guessing_game(guess: i8) {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Your guess: {}. The correct number: {}", guess, secret_number);
    if guess == secret_number {
        println!("Congratulations! You won the game!");
    }
    else {
        println!("Aww... you failed, bud.");
    }
}

/*NOTES:
 *
 * (0) This is a very bare-bones version of a menu with a simple set of options.
 *     It's nothing much, just my testing what I've learned so far.
 *
 * (1) Had to include this line. Otherwise, user input was not read as, for
 *     example '1' but '1\n' - meaning that the newline character was also being
 *     absorbed, which is a no-no.
 *
 * */
