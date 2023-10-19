fn main() {
    
    // DATA TYPES

// let <var_name>: <type_of_var> = <value_ofvar>


    // default values
    let mut x = 2;             // by adding 'mut' I make it mutable

    // specified data type: bits (i -> signed; u -> unsigned):
    let y: i8 = -7;
    let z: u8 = 8;             // can't be a negative, of course


    x = 3;
    
    x = 4;                     // this is ok also. 'x' is was declared mutable
    // Single and double precision:
    
    let floating_point: f32 = 10.94443291349;  // stops at 6 decimal places
    let floating_point2: f64 = 11.94443291349; // stops at 10 decimal places
    println!("{floating_point}");
    println!("{floating_point2}");
    
    

    // booleans

    let true_or_false: bool = false;    // can be also 0 and 1 for true

    
    // characters

    let char: char = 'a';

    // Tuples


    let tup: (i32, bool, char) = (1, true, 's');

    let mut tup2: (i8, bool, char) = (1, true, 's');

    // Important: note that the previous tuples are of different types, because
    // of the integer type (!)
    
    // Accessing tuple elements
    println!("{} and {} and, of course {}", tup.0, tup.1, tup.2);

    // changing tuple elements
    tup2.0 = 10;

    println!("{}", tup2.0);
}
