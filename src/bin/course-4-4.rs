// -------------------------------------------
// Loops
// - loops with no condition
// - While loop
// -------------------------------------------
fn main() {
    /* loop {
        println!("This is an infinite loop");
    } */

    let my_number = 5;
    let mut guess = false;

    println!("guess my number which is between 1 and 20");

    while guess != true {
        let mut number = String::new();
        // note: in vscode this runs only with the "rust-analyzer: Run" command
        std::io::stdin()
            .read_line(&mut number)
            .expect("failed to read input.");
        let number: u8 = number.trim().parse().expect("invalid input");

        if my_number == number {
            println!("you guessed correct!");
            guess = true;
        } else {
            println!("Try again.");
        }
    }

    println!("Enter a number and I will tell you the next number after your number divisible by both 2 and 5");
    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("failed to read input");
    let mut number: u8 = number.trim().parse().expect("invalid input");

    number = number + 1; // to prevent that the same number is returned first add 1
                         // to enforce another number
    while (number % 2 == 0 && number % 5 == 0) != true {
        number += 1;
    }
    println!(
        "The number after your number divisible by both 2 and 5 is {}",
        number
    );
}
