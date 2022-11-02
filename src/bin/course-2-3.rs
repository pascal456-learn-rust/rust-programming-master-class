fn main() {
    // ---------------------------------------
    //              Initializing multiple variables
    // ---------------------------------------

    let (first_number, second_number) = (250, 480.32);

    println!(
        "The first number is {} and the second number is {}",
        first_number, second_number
    );

    // ---------------------------------------
    //              readability of large numbers
    // ---------------------------------------

    let large_number = 1_000_000;
    println!("the value of large number is: {}", large_number);

    // ---------------------------------------
    //              Integer overflow
    // ---------------------------------------

    // throws "error: literal out of range for `u8`"
    // let overflow_number: u8 = 256;

    // ---------------------------------------
    //              decimal numbers in other formats
    // ---------------------------------------

    let x = 255;
    println!(
        "the value of variable x in hexadecimal is {x:o},
    and in octal it is: {x:X}
    and in binary it is {x:b}"
    );

    // ---------------------------------------
    //              snake case convention
    // ---------------------------------------

    // throws: "warning: variable `Number` should have a snake case name"
    let Number = 45;

    // ---------------------------------------
    //              Operations on numbers in different types
    // ---------------------------------------

    let n1 = 14;
    let n2 = 15.6;

    let n3 = n1 + n2 as i32;
    println!("decimals are truncated, not rounded: {n3}");

    let n4 = n1 as f64 + n2;
    println!("casting n1 to float: {n4}");

    // ---------------------------------------
    //              Shadowing
    //  declare variables with the same name like before
    // ---------------------------------------

    let s = 5;
    let s = 5 * 5;
    println!("shadowed value: {s}");

    let mut s = 4;
    let s = 4 * 4; // it is turned into an immutable; it cannot changed anymore
    println!("shadowed value: {s}");

    let s = 32;
    println!("the value of s = {s} and it is currently integer");

    let s = 'A';
    println!("the value of s = {s} and is currently a character");

    let s = 64.5;
    println!("the value of s = {s} and is currently a float");

    // ---------------------------------------
    //              Scope
    // ---------------------------------------

    // scope (re-declaring the variable)
    let mut s = 65;
    {
        let s = 60;
        println!("the value in the inner scope is {s}");
    }
    println!("the value in the outer scope is {s}");

    // scope (re-initializing the variable)
    let mut s = 65;

    {
        s = 60; // reinitialize (remove let keyword)
        println!("the value in the inner scope is {s}");
    }

    println!("the value in the outer scope is {s}");

    // ---------------------------------------
    //              Constants
    // difference between immutable variables and constants
    // constants are always immutable and cannot be changes to mutable
    // --> 'mut' cannot be used
    // constant types *must* be annotated
    // ---------------------------------------

    const MAX_SALARY: u32 = 1_000_000;
    println!("constant value is {MAX_SALARY}");
}
