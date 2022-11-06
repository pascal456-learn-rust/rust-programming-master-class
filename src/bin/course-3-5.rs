// -------------------------------------------
// String concatenation and onwership
// -------------------------------------------

fn main() {
    let s1 = String::from("Hello");
    let s2 = "world"; // strig slice

    // the plus operator (for concatenation) can only concatenate
    // the String type with a string slice (&str)
    let s3 = s1 + s2; // the plus operator always returns a "String" type, not a str slice

    // the result will not be stored in a separate memory location
    // it will rather be located in the heap at the position of the pointer s1
    // that means the ownership of the data in s1 moves and the pointer s1 will no longe exist
    // println!("{}", s1);
    println!("{}", s3);

    // -------------------------------------------
    // concatenate strings of type String (and not slices)
    // -------------------------------------------
    let a1 = String::from("Hello");
    let a2 = String::from("world");

    let a3 = a1 + &a2; // the ownership of a1 moved; ownership of s2 will not move since a reference is used
    println!("{} {}", a3, a2); // the String a2 retains its ownership

    // -------------------------------------------
    // concatenate strings of type String (and not slices)
    // -------------------------------------------
    let b1 = String::from("hello");
    let b2 = String::from("world");
    let b3 = String::from(" from Rust");

    let b4 = b1 + &b2 + &b3;
    println!("{} {} {}", b4, b2, b3); // the Strings retain their respective ownership here (b2, b3)
}
