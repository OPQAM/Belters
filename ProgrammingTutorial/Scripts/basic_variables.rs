fn main() {
    // Printing in different number bases:
    
    // Binary
    println!("1465 in Base 2: {:b}", 1465);

    // Octal
    println!("1465 in Base 8: {:o}", 1465);

    // Hexadecimal
    println!("1465 in Base 16: {:x}", 1465);                 // note: :X will return uppercase letters
    println!("1465 in Base 16: {:X}  --> Uppercase", 1465);
    

    // print!    -> Prints to io::stdout
    // println!  -> Does the same but adds a newline
    //
    // eprint!   -> Prints to io::stderr
    // eprintln! -> Does the same but adds a newline
    

    println!("Printing on a newline.");
    print!("This line won't generate a newline.");
    print!("So this line will just glue to the previous one.");

    eprintln!("ERROR, ERROR, Will Robinson!!!");


    let ugly_fruit = "bananas";

    // Positional formatting/indexed placeholders
    println!("\nI really like some {0}, but I thoroughly dislike {1}!", "fruit", ugly_fruit);

   // Named formatting/named placeholders
   println!("I'd rather not talk about {ugly_fruit}");

   // A mutable variable
   let mut sentence1 = "I love you"; 

   // A static variable
   let sentence2 = "Death is final";

    println!("{}", sentence1);
    println!("{}", sentence2);

    sentence1 = "I truly do!";

    println!("{sentence1}");
    println!("{sentence2}");

    // sentence2 = "I overcame death!";         These I cannot run since variable2
    // println!("{sentence2}");                 is immutable.

    // Note: Cargo will warn us if any variable is unnused and ask us to annotate it:

    let _sentence3 = "I'm not here";     // By adding a '_' all is fine


}
