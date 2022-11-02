fn main() {
    // variables in Rust

    let mut x: i64 = 15;
    println!("the value of variable x = {}", x);

    // mutable vs. immutable values
    x = 60;

    // variable names can only contain letters numbers and underscores
    // variable names are case sensitive: x vs. X

    let y = 5 * 5;

    // ---------------------------------------
    //              Data Types
    // there are scalar and compound data types
    // - Scalar
    //      - Integers
    //          - Signed    i8, i16, i32, i64
    //              - range: -2^(i-1)-1 to 2^(i-1)-1
    //              - range: -2^(8-1)-1 to 2^(8-1)-1 = -127 to +127
    //          - Unsigned  u8, u16, u32, u64
    // - Compound
    //      - see later chapter (2-4)
    // ---------------------------------------

    println!("maximum number in i8 is equal to: {}", std::i8::MAX);
    println!("maximum number in u8 is equal to: {}", std::u8::MAX);

    // ---------------------------------------
    //              Floats
    //                  - f32, f64
    // ---------------------------------------
    let z = 3.65;

    // integers and floats cannot be added by the operator '+'
    println!("maximum number in f32 is: {}", std::f32::MAX);

    // ---------------------------------------
    //              Boolean
    // ---------------------------------------

    let status = false;
    println!(
        "the values of some of our variables are {:?}",
        (x, y, z, status)
    );

    let not_equals = 18 != 18; // <, <=, >= and > are also possible
    println!("the value of the condition 18 != 18 is {}", not_equals);

    // ---------------------------------------
    //              Characters
    // ---------------------------------------

    let c1 = 'a';
    let c2 = '3';
    let c3 = '+';
    let c4 = '\u{288A}';
    let c5 = '\"';
    println!(
        "the value of c1 = {}, c2 = {}, c3 = {}, c4 = {}, c5 = {}",
        c1, c2, c3, c4, c5
    );
    println!("this is another variant: c1 = {c1}, c2 = {c2}, c3 = {c3}, c4 = {c4}, c5 = {c5}");
}
