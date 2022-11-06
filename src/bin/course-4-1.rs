// -------------------------------------------
// Conditional IF and its variants
// -------------------------------------------

fn main() {
    // 1. Simple if statement
    /* General Syntax

    if condition {
        // statements to execute if condition proves true
    }
    */

    let some_number = 40;
    if some_number < 50 {
        println!("1. The number is less than 40");
    }
    println!("2. This line will execute irrespective of the condition above");

    // -------------------------------------------
    // If with multiple conditions
    // -------------------------------------------

    // logical AND
    let marks = 65;
    if marks >= 60 && marks <= 70 {
        println!("3. the grade is between 60 and 70 and therefore satisfactory");
    }

    // logical OR
    let flag_1 = true;
    let flag_2 = false;

    if flag_1 == true || flag_2 == true {
        println!("4. one of the conditions is true");
    }

    // negation
    let flag_1 = true;
    if flag_1 != false {
        println!(
            "5. this will execute when flag_1 is true or in other words, when flag_1 is not false"
        );
    }

    // brackets for complex conditions
    let flag_1 = true;
    let flag_2 = false;
    let number = 60;
    if (flag_1 == true && flag_2 == false) || number < 50 {
        println!("6. executes when flag_1 is true and flag_2 is false, or if not then at least number should be lower than 50");
    }

    // -------------------------------------------
    // If ELSE
    // -------------------------------------------
    /* General syntax

    if condition {
        // Statements to execute if the condition is ture
    }
    else {
        // Statements to execite in case the condition is false
    }
    */

    let marks = 80;
    if marks > 50 {
        println!("You have passed the exam");
    } else {
        println!("You have failed the exam");
    }

    /* General syntax

    if condition {
        // Statements to execute if condition is true
    }
    else if condition_2 {
        // Statements to execute if condition is true
    }
    else if condition_3 {
        // Statements to execute if condition is true
    }
    else   // optional
    {
        // Statements to execute if all conditions are not true
    }
    */

    let marks = 95;
    let grade;
    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }
    // note: even when all the other conditions are true as well,
    // they are checked sequentially and
    // the first true condition will be executed and the others are skipped
    println!("The grade of the student is {}", grade);
}
