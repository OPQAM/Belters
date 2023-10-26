use std::io;
// DONE BY CHATGPT. I WAS TRYING TO UNDERSTAND 'USER INPUT and TRIMMING'
fn main() {
    loop {
        println!("Choose an option:");
        println!("1) Option 1");
        println!("2) Option 2");
        println!("3) Option 3");
        println!("0) Exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let choice: i32 = input.trim().parse().expect("Invalid input");

        match choice {
            0 => {
                println!("Exiting...");
                break;
            }
            1 => println!("You chose Option 1"),
            2 => println!("You chose Option 2"),
            3 => println!("You chose Option 3"),
            _ => println!("Invalid choice. Try again."),
        }
    }
}
