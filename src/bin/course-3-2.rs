// ---------------------------------------
// Application Memory
//      Heap vs. Stack
//
//
//
// ---------------------------------------

// |------------------------
// |
// |
// |
// |
// |------------------------
// Heap
//      data that can change in size must be stored in the Heap
//      heap is not organized sequentially
//      the

// |------------------------
// |
// |
// |------------------------
// |
// |    square
// |        num
// |
// |
// |------------------------
// |
// |    square_sum
// |        num1, num2
// |
// |------------------------
// |
// |    main
// |        x, y
// |
// |------------------------
// Stack
//      all data stored on the stack must have a known fixed size
//      otherwise when data grows, it can overflow

// |------------------------
// |    MAX_VALUE
// |
// |
// |
// |------------------------
// Global

// |------------------------
// |
// |
// |
// |
// |------------------------
// Code / Text

const MAX_VALUE: i32 = 40_000;

fn main() {
    let (x, y): (i32, i32) = (2, 4);
    let sum_value: i32 = square_sum(x, y);
    println!("Value: {}", sum_value);

    let i1: i32 = 5; // will be put in the stack frame

    let s1: String = String::from("some string"); // s1 can change in size

    /*
    pointer will be stored in the stack
    this is pointing to the actual location in the heap

    now, when reassigning s1 to another location / variable, in other languages it
    is necessary to explicitly free the memory again
    in case of rust the space is deallocated as soon as the program leaves the scope

    with `let s2 = s1;` the ownership moves and the pointer s1 will be deleted from the stack
    the data which it points to will remain in the heap because
    at the same time, a new pointer s2 will be created that points to the data

    with `let s3 = &s2;` another pointer will be added, which points to the data in the heap

    the final statement `let s4 = s2.clone();` another pointer s4 will be created and it
    points to a new memory block. The OS managed to allocate new space in the Heap

    summary: this means with the principle of ownership rusts ensures that the Heap is used more efficiently
    */

    let s2 = s1;
    let s3 = &s2;
    let s4 = s2.clone();
}

fn square_sum(num1: i32, num2: i32) -> i32 {
    let result = square(num1 + num2);
    result
}

fn square(num: i32) -> i32 {
    num * num
}
