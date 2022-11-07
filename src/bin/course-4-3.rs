// -------------------------------------------
// Match
// - Simple match
// - If else ladder into a match
// - If let syntax style match
// -------------------------------------------
fn main() {
    /* General syntax

       match value {

           possible_value(s) => {Statements to execute},
           possible_value(s) => {Statements to execute},
           possible_value(s) => {Statements to execute},

           _ = { default_execution_statements },
       }
    */
    let some_number = 100;
    match some_number {
        1 => println!("The number is 1"),
        // note: single pipe here for the conditional OR
        2 | 3 => println!("number is either 2 or 3"),
        4..=100 => println!("number is between 4 and 100 inclusive"),
        _ => println!("the default satement => the number is greater than 100"),
        // create match cases where at least one of the arms is executed
    }

    // -------------------------------------------
    // - If else ladder into a match
    // -------------------------------------------
    let marks = 50;
    let grade;
    match marks {
        90..=100 => {
            grade = 'A';
            println!("another statement");
        }
        80..=89 => grade = 'B',
        70..=79 => grade = 'c',
        60..=69 => grade = 'D',
        _ => grade = 'F',
    }
    println!("the grade achieved is {:?}", grade);

    // -------------------------------------------
    // - If let syntax style match
    // -------------------------------------------
    /*    general syntax
        let variable = match value {
         possible_value(s) = {Statements to execute},
         possible_value(s) = {Statements to execute},
         possible_value(s) = {Statements to execute},

         _ = { default_execution_statements }
     };
    */
    let marks = 98;
    let grade = match marks {
        90..=100 => {
            let x = 5 * 5;
            let b = 'A';
            'A'
        }
        80..=89 => 'B',
        70..=79 => 'c',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("the grade achieved is {:?}", grade);
}
