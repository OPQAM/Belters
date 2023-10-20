use std::io;                                                         // (1),(2)

fn main() {
    let  mut input = String::new();
    
    println!("Do write something for us to print:");

    io::stdin().read_line(&mut input).expect("failed to read line"); // (3),(4),(5)
    println!("{}", input);

}


/*NOTES:
 *
 * (1) std is a CRATE, which is basically a package/library 
 *
 * (2) io is a MODULE, which is a specific piece of functionality
 *
 * (3) '&mut input' is a mutable reference to the 'input' variable
 *
 * (4) if it were just 'input' it would only be a copy of the variable,
 *     and it would not change the original variable
 *
 * (5) '.expect' is to return an error in case we are unable to use the command
 *     It's important to note that if we don't include the expect, the program will
 *     run, but it will warn us to include it
 *
 *
 * */
