// -------------------------------------------
// Loops
// - For loop
//      - Looping through elements using an immutable reference
//      - Looping through elements using a mutable reference
// -------------------------------------------
fn main() {
    let some_vec = vec![45, 30, 85, 90, 41, 39];

    for i in 0..=5 {
        // 0..=5
        println!("The {}th value in the vector is {}", i, some_vec[i]);
    }

    let some_vec = vec![45, 30, 85, 90, 41, 39];
    println!("len of vec: {}", some_vec.len());
    for i in 0..some_vec.len() {
        if i < (some_vec.len() - 1) {
            print!("{}, ", some_vec[i]);
        } else {
            print!("{}", some_vec[i]);
        }
    }
    println!("\nfull vector: {:?}", some_vec);

    let some_vec = vec![45, 30, 85, 90, 41, 39];
    for i in some_vec {
        println!("{}", i)
    }
    // println!("{:?}", some_vec); // not possible because with "for i in vector" the values are moved (ownership changes)

    // Alternative:
    // this is alternative to the borrowing with &
    let some_vec = vec![45, 30, 85, 90, 41, 39];
    for i in some_vec.iter() {
        println!("{}", i)
    }
    println!("after iterating with '.iter()': {:?}", some_vec);

    let some_vec = vec![45, 30, 85, 90, 41, 39];
    for i in &some_vec {
        // &some_vec = some_vec.iter()
        println!("{}", i)
    }
    println!("{:?}", some_vec);

    // -------------------------------------------
    // Looping through elements using a mutable reference
    // -------------------------------------------
    let mut some_vec = vec![45, 30, 85, 90, 41, 39];
    for i in some_vec.iter_mut() {
        // &mut some_vec = some_vec.iter_mut()
        *i += 5; //deref operator for access to the value instead of the reference
        println!("{}", i);
    }
    println!("{:?}", some_vec);

    let mut some_vec = vec![45, 30, 85, 90, 41, 39];
    for i in &mut some_vec {
        // &mut some_vec = some_vec.iter_mut()
        *i += 5; //deref operator for access to the value instead of the reference
        println!("{}", i);
    }
    println!("{:?}", some_vec);
}
