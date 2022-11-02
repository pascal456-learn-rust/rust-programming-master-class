// ---------------------------------------
// Functions
// ---------------------------------------

fn main() {
    println!("\n===> Program entry.\n");

    basic_fn();
    function_with_inputs("John Doe", 1337);

    // functions with variables as input
    let full_name = "Does, John";
    let salary_info = 7331;
    function_with_inputs(full_name, salary_info);

    let answer = function_with_input_and_output(10, 15);
    println!("multiplication result: {}", answer);

    let (multiplication, addition, subtraction) = function_with_multiple_return_values(42, 2);
    println!(
        "multiplication: {}, addition: {}, subtraction: {}",
        multiplication, addition, subtraction
    );

    let results = function_with_multiple_return_values(2, 42);
    println!(
        "multiplication: {}, addition: {}, subtraction: {}",
        results.0, results.1, results.2
    );

    // code block
    let full_name = {
        let first = "firstname";
        let last = "lastname";
        format!("{}, {}", last, first)
    };
    println!("the codeblock formats the following: {}", full_name);

    // ---------------------------------------
    // user input
    // ---------------------------------------
    // with specific input type
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input) //return value is of type 'Enum': two possible values: 'error'and 'OK'
        .expect("failed to read input."); //mutable reference; a reference to the actual data
    let input_value: f64 = input.trim().parse().expect("invalid input");
    println!("input value was: {}", input_value);

    // alternative: from the rust documentation
    // no specific input type
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("{input}");
        }
        Err(error) => println!("error: {error}"),
    }

    println!("\n<=== Program exit.\n");
}

// basic function without input and output
fn basic_fn() {
    println!("Basic, printing function.");
}

fn function_with_inputs(name: &str, salary: i32) {
    println!("Name: {}, Salary {}", name, salary);
}

fn function_with_input_and_output(num1: i32, num2: i32) -> i32 {
    num1 * num2 //results from lines without ';' are interpreted as return values
}

fn function_with_multiple_return_values(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2) //tuple
}
