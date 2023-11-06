fn main() {
    let mut x = 4;

    x = 5;

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;          // (1)

    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing

    let x = x + 1;

    {
        let x = x * 2;
        println!("The inner scope value of x is: {x}");
    }

    println!("The value of x actually is: {x}");

    let mut spaces = "   ";                                       // (2)
    let spaces = spaces.len();                                    // (3)

    println!("Number of characters in word: {spaces}")
}

/*NOTES:
 *
 * (1) Constants are immutable and can't even have 'mut' applied to them.
 *     By convention they are written in uppercase.
 *     Constants are always annotated.
 *
 * (2) Although we can add 'mut' here, we'll get a warning from the 
 *     compiler, asking us to remove it. The RustBook tells us that we 
 *     can't actually add 'mut' and then shadow this variable like we
 *     do afterwards. But that's actually not the case anymore.
 *     The RustBook might be slightly out of date.
 *
 * (3) Without 'mut' this works just fine, since we are able to shadow 
 *     the variable type, from a tring to an integer.
 *
 * */
