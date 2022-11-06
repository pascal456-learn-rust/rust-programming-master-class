// -------------------------------------------
// References Rules
// - One mutable reference in a scope
// - Many immutable references
// - Mutable and immutable can not coexist
// - Scope of a reference
// - Data should not change when immutable references are in scope
// -------------------------------------------

fn main() {
    // -------------------------------------------
    // One mutable reference in a scope
    // -------------------------------------------

    /*
       let mut heap_num = vec![4, 5, 6];
       let ref1 = &mut heap_num;
       // a second reference in the following would lead to a race condition. Rust prohibits this
       let ref2 = &mut heap_num;

       println!(
           "The first reference is {:?} and the second one is {:?}",
           ref1, ref2
       );
    */

    // -------------------------------------------
    // One mutable reference in a scope
    // -------------------------------------------
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    // this is perfectly fine because we are using doing immutable references
    println!(
        "the first reference is: {:?}, and the second is: {:?}",
        ref1, ref2
    );

    // -------------------------------------------
    // Mutable and immutable can not coexist
    // -------------------------------------------
    /*
    There can only be a mutable or an immutable reference in a certain scope.
    This is because the immutable reference relies on the fact
    that the underlying data does not change.
     */

    /* // this will throw an error: "cannot borrow `heap_num` as mutable because it is also borrowed as immutablerustc(E0502)"
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    let ref3 = &mut heap_num;

    println!("{:?}, {:?}, {:?}", ref1, ref2, ref3) */

    // -------------------------------------------
    // Scope of a reference
    //      with the use of scopes mutable and immutable can coexist
    // -------------------------------------------

    /* scope rule: the scope starts with its first use and ends when it's last used */
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num; // scope of ref1 starts here
    let ref2 = &heap_num; // scope of ref2 starts here

    println!("immutable references are: {:?}, {:?}", ref1, ref2); // scopes of ref1 and ref2 end here

    // therefore ref3 is legal to create in this case because ref2 and ref1 are no longer in scope
    // because there are no more "readers" of the data
    // no one else is "busy" with the data
    let ref3 = &mut heap_num;

    // -------------------------------------------
    // Data should not change when immutable references are in scope
    // -------------------------------------------
    let mut heap_num = vec![4, 5, 6];
    heap_num.push(68);
    let ref1 = &heap_num;
    let ref2 = &heap_num;

    /* this throws an error: "cannot borrow `heap_num` as mutable because
    it is also borrowed as immutable mutable borrow occurs here rustc(E0502)".
    Hereby rust ensures that the data is always safe to use.
    you can outcomment the println to remove the scope, and ommit the error.
    Or alternatively remove the push / adding a new value before the read / println
    */
    // heap_num.push(86);
    println!("immutable references are: {:?}, {:?}", ref1, ref2);
    heap_num.push(86);
}
