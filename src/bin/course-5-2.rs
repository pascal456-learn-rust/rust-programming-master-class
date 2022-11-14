// -------------------------------------------
// Stack use case "String reversal"
// -------------------------------------------
// - same basis like in course-5-1, but changed types

fn main() {
    let input_string = String::from("welcome to Rust");
    let size_stack = input_string.len();
    let mut stack = new_stack(size_stack);
    let mut reverse_string = String::new();

    for i in input_string.chars() {
        push(&mut stack, i, size_stack);
    }

    for i in 0..size(&stack) {
        reverse_string.push(pop(&mut stack).unwrap())
    }

    println!("input string is: {:?}", input_string);
    println!("reverserd string: {:?}", reverse_string);
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let popped_value = stack.pop();
    // println!("\nPopped value: {:?}", popped_value);
    popped_value
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if stack.len() == maxsize {
        // println!("Cannot add more elements, max size reached!");
    } else {
        stack.push(item);
        // println!("Pushed value, new Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn new_stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    vec
}
