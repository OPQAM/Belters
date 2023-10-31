fn main() {
    let x: i32 = 5; 
    let _y: i32; 

    assert_eq!(x, 5);                           // (1) (2)   
    println!("Success!");
}

/* NOTES:
 *
 * (1) *assert_eq!* is a Rust macro. Rust macros are identified by the '!',
 *     distinguishing them from regular functions. Macros operate on the
 *     Rust syntax at compile time
 *
 * (2) This particular macro is trying to check if x == 5   
 * */
