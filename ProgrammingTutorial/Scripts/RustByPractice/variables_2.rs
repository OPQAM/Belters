/*
// Fix the error with the use of define_x
fn main() {
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}
*/

// The function is out of scope, since it is being read only after main(). This means that
// 'x' has no value attributed to it.

fn main() {
    let x = define_x();                            // (1)
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}


/*NOTES:
 *
 * (1) We're calling the function and getting the value of 'x'. Now we can do the println!
 * 
 * */
