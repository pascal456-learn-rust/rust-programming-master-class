// -------------------------------------------
// Stack use case "expression evaluation"
//      PART 3 -> Evaluation
// -------------------------------------------

fn main() {
    /* Rules for postfix evaluation
       1. if operand -> push to stack
       2. if operation -> pop two elements; perform operation; then push result into stack
    */
    let input_expression = String::from("(33+45/3*(2+9)-50)");
    println!("orig expression (infix): {}", input_expression);

    let input_expression_tokenized = individual_symbols(input_expression);
    let postfix = infix_to_postfix(input_expression_tokenized);

    println!(
        "evaluated expression result: {}",
        postfix_evaluation(postfix)
    );
}

fn operation(operant1: String, operant2: String, operator: String) -> f32 {
    let operant1 = operant1.parse::<f32>().unwrap();
    let operant2 = operant2.parse::<f32>().unwrap();

    let result = match operator.as_str() {
        "+" => operant1 + operant2,
        "-" => operant1 - operant2,
        "*" => operant1 * operant2,
        "/" => operant1 / operant2,
        "^" => operant1.powf(operant2),
        _ => 0.0, // default case will never be true
    };
    result
}

fn postfix_evaluation(postfix: Vec<String>) -> f32 {
    let size_expression = postfix.len();
    let mut result_stack: Vec<String> = new_stack(size_expression);

    for i in postfix {
        match i.as_str() {
            // rule 2
            "+" | "-" | "*" | "/" | "^" => {
                let operator = i;

                // order is important here!
                // first popped element is the second operant for the oprator
                let operant2 = result_stack.pop().unwrap();
                let operant1 = result_stack.pop().unwrap();
                let result = operation(operant1, operant2, operator);
                result_stack.push(result.to_string());
            }

            _ => result_stack.push(i),
        }
    }
    // when all symbols have been scanned
    // the loop ends and the only entry left in the stack will be the result
    // therefore just pop the value and return it
    // result_stack.pop().unwrap().parse::<f32>()
    result_stack.pop().unwrap().parse::<f32>().unwrap()
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
                    stack.push(i);
                } else {
                    if priority(&i) > priority(stack.last().unwrap()) {
                        stack.push(i);
                    } else {
                        while priority(&i) <= priority(stack.last().unwrap()) {
                            postfix.push(stack.pop().unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }

                        stack.push(i);
                    }
                }
            }

            // rule 3
            "(" => stack.push(i),

            // rule 4
            ")" => {
                while stack.last().unwrap() != "(" {
                    postfix.push(stack.pop().unwrap());
                }

                // finally just remove the "(" from the stack by pop
                // (and not adding it to postfix)
                stack.pop();
            }

            // rule 5 (symbol is operand)
            // just add operand to postfix
            _ => postfix.push(i),
        }
    }

    while size(&stack) != 0 {
        postfix.push(stack.pop().unwrap());
    }
    println!("postfix:\t{:?}", postfix);

    postfix
}

fn priority(x: &String) -> u8 {
    // alternatively do this with a match expression
    if ("+" == x) | ("-" == x) {
        1
    } else if ("/" == x) | ("*" == x) {
        2
    } else if "^" == x {
        3
    } else {
        0
    }

    /* match x.as_str() {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        _ => 0,
    } */
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

fn size(stack: &Vec<String>) -> usize {
    stack.len()
}

fn new_stack(maxsize: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(maxsize);
    vec
}
