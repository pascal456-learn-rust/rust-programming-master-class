fn main() {
    // ---------------------------------------
    //              Compund Data Types
    //                          (scalar vs. compound)
    //                          (singleton data (number/character) vs. multiple (strings, list of numbers))
    //      Focus here:
    //          - Tuples
    //          - Arrays
    // ---------------------------------------

    // ---------------------------------------
    // Tuples
    // - destructing tuples
    // ---------------------------------------

    let my_information = ("Salary", 10_000);
    println!("{} is equal to {}", my_information.0, my_information.1);

    // destructure a tuple / unpacking
    let (salary, salary_value) = my_information;
    println!("the individual values are: {}, {}", &salary, salary_value);

    let salary = my_information.0;
    let salary_value = my_information.1;
    println!("the individual values are: {}, {}", salary, salary_value);

    // ---------------------------------------
    // Nested Tuples
    // Note: tuples can contain different types
    // ---------------------------------------
    let nested_tuple = (4, 5.0, (3, 2), "Hello");
    // access the nested tuple
    let element = nested_tuple.2 .0;
    println!("the nested tuplevalue is: {}", element);

    let empty_tuple = ();

    // ---------------------------------------
    // Array
    // Note: all elements must have a single common type
    // ---------------------------------------

    // implicit typing and size
    let mut number_array = [4, 5, 6, 8, 9];

    // explicit type
    let mut number_array: [i32; 5] = [4, 5, 6, 8, 9];

    // indexing
    println!("first element of array: {}", number_array[0]);

    println!("whole array: {:?}", number_array); // placeholder for more complex compound types {:?}

    // ---------------------------------------
    // Update Arrays
    // ---------------------------------------

    number_array[4] = 5;
    println!("updated array: {:?}", number_array);

    let array_with_same_elements = [0; 10];
    println!(
        "all the same elements in the array: {:?}",
        array_with_same_elements
    );

    // ---------------------------------------
    // Arrays: Strings and chars
    // ---------------------------------------
    let mut string_array_1 = ["apple", "tomato", "grapes"];
    let string_array_2 = ["unknown"; 6];

    string_array_1[0] = "changed value";

    let char_array = ['a', 'p', 'p', 'l', 'e'];

    // ---------------------------------------
    // Array Slices
    //  - references to some subset in the array (not the original values)
    //  - so: slices cannot be used for updating the array
    // ---------------------------------------

    let mut number_array_1: [i32; 5] = [4, 5, 6, 8, 9];

    // referencing the first 3 parts of the array:
    let subset_array = &number_array_1[0..3]; //'&' is a reference to the array
    println!("the subset is: {:?}", subset_array);

    println!("the subset is: {:?}", &number_array_1[0..=3]); // including the outer index bound

    // subset_array[0] = 16; // not possible

    println!("number of elements in the array: {}", number_array_1.len());

    // ---------------------------------------
    // Arrays: common functions
    // ---------------------------------------
    println!(
        "he array is occupying {} Bytes",
        std::mem::size_of_val(&number_array_1)
    );

    // number_array_1[10] = 5; // array is actually not that long therefore this throws an error
    let check_index = number_array_1.get(100);
    println!("{:?}", check_index); // returns 'None' if there is no such index

    let check_index = number_array_1.get(2);
    println!("{:?}", check_index); // returns 'Some(i)' if the index exists
}
