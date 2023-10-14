src/main.rs
   use std::io;             // the 'input/output' library (part of 'stdin')
   use rand::Rng;                         

   fn main() {
       println!("\nGuess the number!");
    
       let secret_number = rand::thread_rng().gen_range(1, 101);

       println!("The secret number is: {}", secret_number);

       println!("Please input your guess.");
    

       // creating mut variable 'guess' ('new' is an associated function)
       let mut guess = String::new();


       //calling the stdin function from the io module
       io::stdin()
           // where we store the user input ('guess' needs to be mutable)
           .read_line(&mut guess)
           // handling potential exceptions
           .expect("Failed to read line");

       let guess: u32 = guess.trim().parse()
           .expect("Please type a number!");

       println!("You guessed: {}", guess);

       match guess.cmp(&secret_number) {
           Ordering::Less => println!("Too small."),
           Ordering::Greater => println!("Too big."),
           Ordering::Equal => println!("You win!"),
       }
   }

// Note: The 'match' construct and patterns are features that lets us
// handle different situations our code might encounter
