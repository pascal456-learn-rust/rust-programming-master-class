// -------------------------------------------
// Stack use case "expression evaluation"
//      PART 2 -> converting infix to postfix
// -------------------------------------------

// ich habe einige der funktionen reduziert (pop, push) werden nicht mehr genutzt
// daher einige auskommentierte zeilen

fn main() {
    let input_expression = String::from("(33+45/3*(2+9)-50)");
    println!("orig expression (infix): {}", input_expression);

    let input_expression_tokenized = individual_symbols(input_expression);
    let postfix = infix_to_postfix(input_expression_tokenized);
}

fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    /* Rules for converting infix expressions into postfix format
        1. priorities of operators (ascending order)
            -> "+", "-"
            -> "*", "/"
            -> "^"

        2. if scanned operator is <= than the top of the stack in priority,
        then pop operators until we have a lower priority. Add the popped
        elements to the postfix

        3. if "(", then push it to the stack

        4. if ")", then pop elements until "(" and add popped elements to the postfix

        5. if operand, then just add it to the postfix
    */

    let size_expression = input.len();
    let mut stack = new_stack(size_expression);
    let mut postfix: Vec<String> = Vec::new();

    for i in input {
        match i.as_str() {
            // rule 2
            "+" | "-" | "/" | "*" | "^" => {
                if size(&stack) == 0 {
                    //in case it is the first element, which would be None
                    // push(&mut stack, i, size_expression);
                    stack.push(i);
                } else {
                    if priority(&i) > priority(stack.last().unwrap()) {
                        // push(&mut stack, i, size_expression);
                        stack.push(i);
                    } else {
                        while priority(&i) <= priority(stack.last().unwrap()) {
                            // postfix.push(pop(&mut stack).unwrap());
                            postfix.push(stack.pop().unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }
                        // push(&mut stack, i, size_expression);
                        stack.push(i);
                    }
                }
            }

            // rule 3
            "(" => {
                // push(&mut stack, i, size_expression);
                stack.push(i);
            }

            // rule 4
            ")" => {
                while stack.last().unwrap() != "(" {
                    // postfix.push(pop(&mut stack).unwrap());
                    postfix.push(stack.pop().unwrap());
                }

                // finally remove the "(" from the stack by pop it
                // pop(&mut stack).unwrap();
                stack.pop();
            }

            // rule 5 (symbol is operand)
            _ => {
                // just add operand to postfix
                postfix.push(i);
            }
        }
    }

    while size(&stack) != 0 {
        // postfix.push(pop(&mut stack).unwrap());
        postfix.push(stack.pop().unwrap());
    }
    println!("postfix:\t{:?}", postfix);

    postfix
}

fn priority(x: &String) -> u8 {
    if ("+" == x) | ("-" == x) {
        1
    } else if ("/" == x) | ("*" == x) {
        2
    } else if "^" == x {
        3
    } else {
        0
    }
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
    println!("tokenized:\t{:?}", tokenized_input);
    tokenized_input
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let popped_value = stack.pop();
    // println!("\nPopped value: {:?}", popped_value);
    popped_value
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() < maxsize {
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
