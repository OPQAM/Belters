fn main() {
    // CONDITIONS AND CONTROL FLOW (<, >, >=, <=, ==)
    let cond = 2 < 3;                   // (1), (2)
    println!("{}", cond);

    let cond2 = 3.3 < 2.5;             // (3)
    println!("{}", cond2);

    let cond3 = (2 as f32) <= 2.3;      // (4)              
    println!("{}", cond3);

    //Compound Conditions (and/or/not)
    let cond4 = false && !cond;
    println!("{}", cond4);

    let cond5 = !(true || cond3);      // (5)
    println!("{}", cond5);

    //Flow Control
    let food = "lamprey";

    if food == "cookie" 
    {
        println!("Cookie? {food}!");
    }
    else if food == "lamprey" 
    {
        println!("{} is yuck and unholy. PURGE!", food);
    }
    else
    {
        println!("{food}? No cookie?...");
    }




}

/*NOTES:
 *
 * (1) this condition evaluates to true, since 2 is less than 3
 * 
 * (2) if we do, for example, 2 < 2.3 it will return an error, since it is
 *     expecting to be operating with two integers and not int + float. see (4)
 *
 * (3) this will run just fine since we're operating with two floats
 *
 * (4) this runs fine, since the integer is being converted into a float
 *
 * (5) an 'OR' and a 'NOT'. And in the previous example, an 'AND' as well
 *
 * */
