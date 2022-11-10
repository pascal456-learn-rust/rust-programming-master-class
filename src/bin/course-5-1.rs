// -------------------------------------------
// Stack
//      - Stack using vec
//      - LIFO principle
// -------------------------------------------
// exampes for stack usage:
// - string reversal
// - expression evaluation
// - back traicking

fn main() {
    println!("let us first create a stack for our use");
    println!("Please mention the size of the stack ");

    let size_stack = input_u32();
    let mut stack = new_stack(size_stack as usize);

    loop {
        println!("\n***** MENU *****");
        println!("1. Push\n2. Pop\n3. Display\n4. Size\n5. Exit");
        println!("Enter your choice: ");

        let choice = input_u32();
        match choice {
            1 => {
                println!("\nEnter the value to be inserted: ");
                let item = input_u32();
                push(&mut stack, item, size_stack as usize);
            }
            2 => {
                pop(&mut stack);
            }
            3 => {
                println!("\nCurrent stack: {:?}", stack);
            }
            4 => {
                println!("\nStack size: {}", size(&stack));
            }
            5 => {
                println!("\nExiting...");
                break;
            }
            0_u32 | 6_u32..=u32::MAX => {
                println!("\nOption not available! Choose another Option.");
            }
        }
        /* println!("Type (Y)es, to continue");
        let choice_continue = input_str(); //returns uppercase
        match choice_continue.as_str() {
            "Y" | "YES" => continue,

            _ => {
                println!("Exiting.");
                break;
            }
        } */
    }
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_value = stack.pop();
    println!("\nPopped value: {:?}", popped_value);
    popped_value
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Cannot add more elements, max size reached!");
    } else {
        stack.push(item);
        println!("Pushed value, new Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn input_u32() -> u32 {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");
    let n = n.trim().parse().expect("Invalid input.");
    n
}

/* fn input_str() -> String {
    let mut str_input = String::new();
    std::io::stdin()
        .read_line(&mut str_input)
        .expect("Failed to read input.");
    str_input.trim().to_string().to_uppercase()
} */
