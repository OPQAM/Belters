use std::io;


fn main() {

    let mut user_input = String::new();
    
    println!("Please do write something:");

    io::stdin().read_line(&mut user_input).expect("Expected to read line. Error.");
   
    
    let user_input: i64 = user_input.trim().parse().unwrap();     // (1), (2), (3), (4)


    println!("{}", user_input + 2);

}


/*NOTES:
 *
 * (1) '.trim()' will remove a blank character that otherwise will appear in our output
 *
 * (2) '.parse()' will handle the conversion into a number
 *
 * (3) '.unwrap' will handle the integer, unwrap it into the type and return it to us
 *
 * (4) Of course, our entered value must be an integer of data type i64, or it won't run
 *  
 * */
