// -------------------------------------------
// 			Rust Ownership
// 			- Each value in Rust has a variable that’s called its owner.
// 			- There can be only one owner at a time.
// 			- When the owner goes out of scope, the value will be dropped.
// -------------------------------------------

fn main() {
    // -------------------------------------------
    // Ownership and functions
    // -------------------------------------------

    let stack_num: i32 = 32;
    let mut heap_num = vec![4, 5, 6];

    stack_function(stack_num);
    println!(
        "the stack variable is copied and the original value was: {}",
        stack_num
    );

    /*
    the following returns an error: "value borrowed here after move".
    This is because the value in on the heap and
    the ownership of this value is moved when it is passed as argument
    The ownership is moved "to the inside" of the function
    Then the value of the current scope ("main") is dropped
    */
    // heap_function(heap_num);
    // println!("value of the vector outside the function is {:?}", heap_num);

    /* instead: pass a value on the heap by reference
    Note: in that case the function parameter notation must also be changed, see function "heap_function"
    */
    heap_function(&heap_num);
    println!("value of the vector outside the function is {:?}", heap_num);

    // in this case also the vector variable in the heap is also changed in the "main" scope
    heap_function_mutable(&mut heap_num);
    println!("value of the vector outside the function is {:?}", heap_num);

    // -------------- quick quiz: who is the owner of the vector in the following three lines?
    let mut heap_num1 = vec![7, 8, 9];
    let ref1 = heap_num1;
    let ref2 = &ref1;

    // answer: ref1

    // --------------
    // common mistake
    // --------------
    let mut heap_variable = vec![4, 5, 6];
    let mut ref1 = &heap_variable;
    println!("immutable references are {:?}", ref1);
    // this code compiles but does not make much sense:
    // we should not create a variable that itself is mutable and points to a reference of a mutable value
    // rather we should create mutable references. So put the mut infront the variable name and after the '&':
    let mut _ref1 = &mut heap_variable;
    // so one has to differentiate between mutable reference and a reference being mutable

    // -------------------------------------------
    // When will references becomnes handy
    //      in scenarios involving large data, to save resources
    // -------------------------------------------
    let large_data1 = String::from("This is the first long string");
    let large_data2 = String::from("This is the second long string");

    let huge_data: Vec<&str> = vec![&large_data1, &large_data2];
    println!(
        "The values of the the combined strings are '{:?}",
        huge_data
    );
}

fn stack_function(mut var: i32) {
    var = 56; // this is another value; it is a copy;
              // even if the variable name was the same like the one that would have been used as the argument
    println!("copied value was updated to: {}", var);
}

// fn heap_function(var: Vec<i32>) {
//     println!("value of vector inside the function is: {:?}", var)
// }

fn heap_function(var: &Vec<i32>) {
    println!("value of vector inside the function is: {:?}", var)
}

fn heap_function_mutable(var: &mut Vec<i32>) {
    var.push(555);
    println!("changed value of the vector inside the function: {:?}", var)
}
