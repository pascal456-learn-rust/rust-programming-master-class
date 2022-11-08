// -------------------------------------------
// Break: for stopping a loop
// Continue: for skipping the current iteration
// ------------------------------------------
fn main() {
    // -------------------------------------------
    // Break
    // ------------------------------------------
    let mut var = 100;
    loop {
        var = var - 1;
        if var % 13 == 0 {
            break;
            // break will shift the control to the first statement outside the body
            // in this chase this is the print statement
        }
    }
    println!(
        "the highest number lesser than the given number and divisible by 13 is {}",
        var
    );

    // -------------------------------------------
    // Continue
    // ------------------------------------------
    let mut var = 0;
    let mut count = 0;
    loop {
        var += 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("{} -> divisible by both 3 and 5", var);
            count += 1;
            if count == 3 {
                break;
            } else {
                continue;
            }
        }
        println!("{}", var);
    }

    // -------------------------------------------
    // Break with return value#
    //      - only with simple "loop", not with while or for loops
    // ------------------------------------------
    let mut var = 0;
    let mut count = 0;
    let returnvalue = loop {
        var += 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("{} -> divisible by both 3 and 5", var);
            count += 1;
            if count == 3 {
                break var; // define return value here
            } else {
                continue;
            }
        }
        println!("{}", var);
    }; // semicolon!

    println!(
        "the required third highest number divisible by both 3 and 5 is {}",
        returnvalue
    );
}
