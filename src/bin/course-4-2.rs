// -------------------------------------------
// Condition If
// - Nested If
// - If let
// - If let (in case of if else ladder)
// -------------------------------------------

fn main() {
    // -------------------------------------------
    // nested ifs
    // -------------------------------------------

    /*
    if outer_condition {
        // Statement to execute if the outer_condition is true

        if inner_condition {
            // Statements to execute if both the outer and inner conditions are true
        } else {
            // some statements to execute
        }
    } else {
        // some statements to execute
    }
    */

    println!("1. Enter a number:");
    let mut some_number = String::new();
    std::io::stdin()
        .read_line(&mut some_number)
        .expect("failed to read input.");
    let some_number: i32 = some_number.trim().parse().expect("invalid input.");

    if some_number != 0 {
        // !(some_number == 0) this is equal to some_number != 0
        // inner if condition
        if some_number % 2 == 0 {
            println!("1. The number is even.");
        } else {
            // inner else condition
            println!("1. The number is odd.");
        }
    } else {
        // outer else condition
        println!("1. The number is 0 and it is neither even nor odd.");
    }

    // -------------------------------------------
    // let if
    // -------------------------------------------
    /*
    let variable_name = if condition {
        // Statements to execute and
        // return value of variable without a semicolon
    } else {
        // Statements to execute and
        // value of variable of variable without a semicolon
    };  // semicolon!
    */

    let value = if true { 1 } else { 2 };
    println!("\n2. Value = {}", value);

    let marks = 50;
    let grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else if marks >= 60 {
        'D'
    } else {
        // Deleting the else block will not compile
        'F'
    };
    println!("\n3. The obtained grade is {:?}", grade);
}
