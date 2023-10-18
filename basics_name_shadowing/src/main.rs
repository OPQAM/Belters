fn main() {
    // MAIN SCOPE
    // A variable is only valid inside the scope in which it was declared


    let x: i32 = 10;
    {
        // INNER SCOPE
        
        let y: i32 = 4;

        println!("The value of x: {}; the value of y: {}", x,y);
    }

    // Here, 'x' will still exist, but not 'y'. 'y' lives in that inner scope

    // This won't run:
    //  println!("The value of x: {}; the value of y: {}", x,y);
    
    // Let's see this principle in action again:
    // Now with Variable Shadowing

    let z = 5;
    println!("z: is {z}");

    {
        let z = 67;          // Now, inside this inner scope, x = 67;
        println!("z: is {z}"); // The outer variable is shadowed by this inner one

    }

    let z = z + 5;              // We'll get 10;

    println!("z: is {z}");      // As we can see, the outer variable didn't stop
                                // existing. It was just shadowed by the inner one.
                                // The outer variable was innacessible inside the
                                // inner scope.
                                // As we exit the inner scope, the inner z goes out
                                // of scope, and the outer z becomes accessible again


}
