// -------------------------------------------
// Stack use case "expression evaluation"
// -------------------------------------------

fn main() {
    let input_expression = String::from("(33+45/3*(2+9)-50");
    println!("orig expression: {}", input_expression);

    let input_expression_tokenized = individual_symbols(input_expression);
}

fn individual_symbols(input_expression: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();

    // collect to turn one collection into another collection
    let input_chars: Vec<char> = input_expression.chars().collect();

    let mut temp: Vec<char> = Vec::new();
    for i in input_chars {
        if i != '+' && i != '-' && i != '/' && i != '*' && i != '^' && i != '(' && i != ')' {
            // operand
            temp.push(i);
            continue;
        } else {
            // operation
            if temp.len() == 0 {
                tokenized_input.push(i.to_string());
            } else {
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(i.to_string());
                temp = vec![]; // flush
            }
        }
    }
    if temp.len() != 0 {
        // check if there is anything left and clean up by a push to the stack
        // otherwise, in expressions that end on an operant, the respecting operant would not be collected
        // e.g. 36 in "(44+33)*(34+39)-36"
        tokenized_input.push(temp.into_iter().collect());
    }
    println!("{:?}", tokenized_input);
    tokenized_input
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let popped_value = stack.pop();
    // println!("\nPopped value: {:?}", popped_value);
    popped_value
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() != maxsize {
        stack.push(item);
    }
}

fn size(stack: &Vec<String>) -> usize {
    stack.len()
}

fn new_stack(maxsize: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(maxsize);
    vec
}
