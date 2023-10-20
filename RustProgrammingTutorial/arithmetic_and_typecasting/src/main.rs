fn main() {
    
    let x: u8 = 10;                    // range: 0 -> 255
    let _y: i8 = 11;                   // range: -128 -> 127

    // let z = x + y;                  // (1)
    // let x: f32 = 9;                 // (2)
    
    let w: u8 = 255;

    // let z = x - w;                  // (3)
    // let z = x + w;                  // (4)

    // Integer division
    let z = w / x;                     // (5)

    println!("{}", z);

    let k: f64 = 255.0;
    let v: f64 = 10.0;

    // Float division
    let m = k / v;

    println!("{}", m);                 // (6)
                                    

    // Modulus operator
    let m = k % v;

    println!("{}", m);          


    // Type casting and conversion

    // The default float is an f64. Ex:
    let u = 33.2;

    // But what if we want to have an f32?
    let y = 33.2f32;
    let z = 33.2_f32;
    let w = 33.2 as f32;
    println!("{} and {}, and {} and {}", u, y, z, w);

    
    // Conversion for calculations
    let x = 127_000 as i64;

    let z = x / (y as i64);            // (7)

    println!("Converted division result: {}", z);


    // readability
    let x: i64 = 240453;
    let y: i64 = 240_453;

    println!("The numbers: {} and {}.", x, y);

    
    // Getting the maximum value of a data type
    let x = i32::MAX;

    let y = (i32::MAX as i64) + 1;

    println!("We get 'x' as an i32, at its max value: {}, and converted + 1 {}", x, y);
    
    let z = i32::MIN;
    let w = u32::MIN;
    let y = u32::MAX;
    println!("{} = lowest possible value for a i32 data type.", z);
    println!("{} for the minimum, of course, and {} for the maximum", w, y);



}



/*NOTES: 
 *
 * (1) This won't run because we can't add different data types
 *
 * (2) Another problem. This is a float, but being given an int. Solution: '10.0'
 *
 * (3) The result will be a negative number, but the variables are unsigned (overflow)
 *
 * (4) Another overflow here. 'u8' has an upper limit of 255. And we're addinng 10 to that
 *
 * (5) This returns an integer (25) because the operands are int. So we get a floor result
 *
 * (6) Since this is now a float, it will round to the adequate number of decimal places
 *
 * (7) A simple conversion here, so that we can have an single data type operation
 *     Also note that we dont need the parenthesis '(y as i64)'. We can do without
 *
 *
 * */
