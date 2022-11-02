// ---------------------------------------
// Ownership
//
//      - in rust each value has a variable that is called its owner
//      - there can only be one owner at a time
//      - when the owner goes out of scope, the value is dropped
// ---------------------------------------

fn main() {
    // ---------------------------------------
    // copy and move
    //      there are different variable types (not to confuse with data types)
    //      - Primitives
    //          - cannot be emty
    //          - fixed size
    //      - non-Primitives
    //          - can be emty
    //          - can grow in size
    // ---------------------------------------

    // for PRIMTIVES, rust will make a COPY of the value, and allocate it different memory locations
    let mut x = 32;
    let mut y = x;
    println!("x: {}, y: {}", x, y);

    // with NON-PRIMITIVES (e.g. String), , in rust the ownership principle comes into play
    // ownership will be MOVED
    let s1 = String::from("abc"); //s1 is the owner here
    let s2 = s1; // this assignment changes the ownership (move)
                 //--> no copy, but a move --> s1 is no longer a valid variable

    // println!("s1: {}, s2: {}", s1, s2); //this will produce an error

    let s3 = String::from("abc"); //s1 is the owner here
    let s4 = &s3; // assignment with '&' does not change the ownership (borrowing by reference)
    println!("s1: {}, s2: {}", s3, s4); //this does not produce an error anymore

    let num_vec1: Vec<i32> = vec![5, 6, 9, 8, 7];
    let num_vec2 = &num_vec1; // remove '&' and there will be an error
    println!("first vec: {:?}, second vec: {:?}", num_vec1, num_vec2);

    // ---------------------------------------
    // make a copy
    //      using clone
    // ---------------------------------------
    let num_vec2 = num_vec1.clone();
    println!("original: {:?},  cloned: {:?}", num_vec1, num_vec2);

    // ---------------------------------------
    // scope
    // ---------------------------------------
    {
        // opening a scope
        let my_name = String::from("foo bar");
    }
    // println!("my name is: {}", my_name); // throws an error because it is out of scope
}
