fn main() {
    // ---------------------------------------
    // Compound Data Types
    // Vectors
    //      - collection of similar elements
    //      - no fixed size
    //      - resizable (vs. arrays)
    // ---------------------------------------

    // delcaration
    let mut number_vec: Vec<i32> = vec![4, 5, 6, 8, 9, 10, 11, 12, 15, 16, 12, 10];
    println!("{}", number_vec[0]);

    // updating elements
    number_vec[4] = 5;
    println!("{:?}", number_vec);

    let _array_with_same_elements: Vec<i32> = vec![0; 10];

    // ---------------------------------------
    // Vectors of different types
    // ---------------------------------------
    let mut string_array_1: Vec<&str> = vec!["apple", "tomato", "grapes"];
    string_array_1[0] = "changed value";

    let _string_array_2: Vec<&str> = vec!["unknown"; 6];

    let _char: Vec<char> = vec!['a', 'p', 'p', 'l', 'e'];

    // ---------------------------------------
    // Vectors: slices
    // ---------------------------------------
    let subset_vec = &number_vec[0..3];
    println!("subset of the values in the array: {:?}", subset_vec);

    // ---------------------------------------
    // Vectors: common functions
    // ---------------------------------------

    println!("number of elements in the array: {}", number_vec.len());

    // get version with vectors
    // 'None', when the index is not in the vector
    let check_index = number_vec.get(100);
    println!("{:?}", check_index);

    let check_index = number_vec.get(2);
    println!("{:?}", check_index);

    // add elements
    number_vec.push(30);
    number_vec.push(40);
    println!("add numbers 30 and 40: {:?}", number_vec);

    // removing elements
    number_vec.remove(5);
    println!("remove number at index 5: {:?}", number_vec);

    // check if a value exists in the vector
    number_vec.remove(number_vec.len() - 1);
    println!("remove number at last index: {:?}", number_vec);
    println!("is 40 in there?: {}", number_vec.contains(&40));
}
