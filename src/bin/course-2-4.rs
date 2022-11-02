fn main() {
    // ---------------------------------------
    //              Compund Data Types
    //      Focus here:
    //          - Strings
    //              - string slices (&str): fixed in length / size
    //                  accessed through / referenced by '&'
    // ---------------------------------------

    let some_string = "fixed length string.";

    println!("the text inside the variable is: \"{some_string}\"");

    // ---------------------------------------
    //              Compund Data Types
    //      Focus here:
    //          - Strings
    //              - variable length strings
    //              - adding / removing characters
    //              - operations on strings
    //              - Formatting and concatenating strings
    // ---------------------------------------

    let mut growable_string = String::from("this string will grow");

    println!("the text inside the string is \"{growable_string}\"");

    growable_string.push('!');
    println!("there is a character added: \"{growable_string}\"");

    growable_string.pop();
    println!("the last character is removed: \"{growable_string}\"");

    growable_string.push_str(", which I will use");
    println!("the text inside the variable is \"{growable_string}\"");

    println!(
        "I will tell you some basic things about strings,
    Is the string empty: {},
    The length of th string is: {},
    How many bytes does the string occupy: {},
    Does the string contain the word 'use': {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use")
    );

    // operations
    // chaining functions
    growable_string.push_str("    ");
    println!(
        "the length of the string before the trim:  {}, 
    length afterwards: {}",
        growable_string.len(),
        growable_string.trim().len()
    );

    // converting number to string
    let number = 6;
    println!("the variable value is: {}", number.to_string());

    println!(
        "is the number really a string: {}",
        number.to_string() == "6" // Note: double quotes for strings!
    );

    // converting character to string
    // Note: single quotes for characters!
    let some_char = 'a';
    println!("the variable value is: {}", some_char.to_string());

    println!(
        "is the value really a string: {}",
        some_char.to_string() == "a" // Note: double quotes for strings!
    );

    //
    let my_name = "foo bar";
    println!("the string contains a name: {my_name}");

    let empty_string = String::new();
    println!("length of the empty string: {}", empty_string.len());

    // format strings with the format! macro
    let s_1 = "foo";
    let s_2 = "bar";
    let s_3 = format!("first: {}, last: {}", s_1, s_2);
    println!("string pre-formatted with format! macro: {}", s_3);

    // concatenate strings
    let string_1 = String::from("foo");
    let string_2 = String::from("bar");
    let string_3 = format!("{}{}", string_1, string_2);
    println!("concatenated string: {}", string_3);
}
