fn test_two()                                              // (1)
{
    println!("Test number two has been called...");
}

fn main() {

// FUNCTIONS, EXPRESSIONS AND STATEMENTNS
    
    let _x = 20;                                           // (2)
    // let y = (let x = 20)                                // (3)

    let number = {                                         // (4)
        let x = 3;
        x + 1
    };
    
    println!("{}", number);

    let result = add_numbers(2, 3);
    println!("{}", result);

    test_two();                                            // (5)

    println!("Multiplah: {}!",multiply_numbers(7, 3));
 
    let mish_mash = subtract_numbers(7, 7);
    println!("{}", mish_mash);

}

fn add_numbers(x: i32, y: i32) -> i32 {                    // (6)
    x + y                                                  // (7)

}

fn multiply_numbers(x: i16, y: i16) -> i16 {
    return x * y;                                          // (8)

}

fn subtract_numbers(x: i8, y: i8) -> i8 {                  // (9)
    let result = x - y;
    if result <= 10 {
        return -1;
    } else {
    result  
    }
}




/*NOTE:
 * (0) Rust functions return expressions, but do not return statements
 * (7) Everything that returns or gives a value, that is true or false, etc
 *     that would be an expression
 *
 * (1) It doesn't matter where we place the function. Main will read it
 *
 * (2) These is the example of a statement
 *
 * (3) We can't do this in Rust (unlike Python). See (0)
 *
 * (4) 'let number' is an expression. It returns a value: '4'. No ';' in 'x + 1'
 *
 * (5) A function call
 *
 * (6) x and y are the function parmameters. Also part of a statement,
 *     the '->' tells us what will be the return value, an 'i32'
 *
 * (7) Note that, without the ';' we have basically a 'RETURN X + Y'
 *
 * (8) But!, if we do add the 'return' statement, we can have the semicolon
 *
 * (9) A slightly more involved example. Notice we are doing 'return result;'
 *
 * */
