fn main() {
    // first program of the course
    // and the second line of a one-line comment
    println!("Hello, world!");

    /* this is a block comment
    there can be a lot of characters in here
    */

    println!("demonstration of the println function.");

    // learning some basic output commants
    println!("the value I want to print is: {}", 10);

    // learning to print strings
    println!(
        "my first name is: {}, my last name is: {}.",
        "Flying", "Pizza"
    );

    // learning the print command
    print!("this is a single print command.");
    print!("this is going to be printed on the Same line.");
    println!("");

    // learning to write on lultiple lines using the print command
    println!(
        "this is going 
    to be printed
    on multiple lines"
    );

    // learning the use of escape sequences
    println!("\n\n this is printed after two lines.\t this has a tab of space before it.");

    println!("this will print as normal characters: \\n");

    // learning some uses of backslashes
    println!("this will print single quote \' and this double quote\"");
    println!("this prints even a backslash \\");

    print!("this is text which will be overwritten\rthis text will overwrite the first because the carriage is returned.");

    // learning positional arguments
    println!(
        "\n I do {2}, from {1} years and I {0}",
        "like it.", 20, "programming"
    );

    // named arguments
    println!(
        "{language} is a systems programming language, good for {activity}",
        activity = "learning",
        language = "Rust"
    );

    // print basic math
    println!("The summation of {} + {} is {}", 20, 25, 20 + 25);
}
