// extern crate chrono;                    // Will this work crateless?
// use chrono::Local;
use std::io;

fn main() {
    
    // Let's have a loop here
    loop {
        println!("╔═════════════════════════════╗");
        println!("║     Thy Tired Olde Menu     ║");
        println!("║         sayest thou?!       ║");
        println!("║                             ║");
        println!("║     But how can that be?    ║");
        println!("║ Pick your pick, you picker: ║");
        println!("╠═════════════════════════════╣");
        println!("║     1) What day is it?      ║");
        println!("║     2) Attempt to Boink!    ║");
        println!("║     3) Exeunt, all of ya.   ║");
        println!("╚═════════════════════════════╝");

        let mut input = String::new();

        // Read a line of input from the user and store it in the 'input' variable.
        io::stdin().read_line(&mut input).expect("failed to read line");
        
        // Parse the input to an integer (i32) and remove leading/trailing whitespace.
        let choice: i32 = input.trim().parse().expect("Invalid input");

        if choice == 1
        {
            println!("This is just a tired old placeholder. I need to add the date.");
            break;
        }
        else if choice == 2
        {
            println!("Bóink!");
        }
        else if choice == 3
        {
            println!("Goodbye!");
            break;
        }
        else 
        {
            println!("Speak up! I don't get that!");
        }  

    }
}

//  {╣, ═, ╝, ╚, ╔, ╗, ║, ╠, ╦, ╩, ╬}
