/*
    Ownership is based on three simple rules:
     - Each value has a variable that is its "owner".
     - A value can have only one owner at a time.
     - If the owner goes out of the scope, the value is cleaned up.
 */

fn main() {
    /** First Rule **/
    let v1: String = String::from("Hello World!");  // v1 contains the pointer to the string
    let v2: String = v1;  // the pointer to the string in v1 is MOVED into v2 (v1 pointer is invalidated)
    // println!("v1 is: {v1}");  // Error: borrow of moved value: `v1`
                                 // in respect of the Ownership, the v1 can have only ONE owner, so
                                 // v1 do not have any pointer anymore.

    /** Second Rule **/
    // To avoid that behaviour we can use .clone() method to make a COPY of a value (pointer in this case)
    let v3: String = String::from("Hello World 2!");
    let v4: String = v3.clone();  // now we have 2 IDENTICAL strings on the Heap (duplicated info, with
                                  // each variable as single owner).
    println!("v3 and v4 are: {v3}, {v4}");
    // TIP: clone is similar to the deep copy in Python

    /** Third Rule **/
    let v5: String = String::from("Hello World 3!");
    {
        let v6: String = v5;
        println!("v6 is: {v6}");
    }
    // println!("v6 is: {v6}");  // v6 will be dropped when we exit from the inner scope
                                 // in this case also the Heap memory is free (with Stack), so we
                                 // do not have MEMORY LEAKS and DANGLING POINTERS.

    // NOTE: all this mechanism is done a compile time, so we have zero runtime cost.

    /**
        Recapitulation

        In Rust when we assign the value of one variable to another, the value is being MOVED, leading
        to a change of Ownership!

        That is not true for all types, indeed, for some primitive types (like ints, floats, bools, chars)
        the assign operator do a simply COPY.
    **/

    let x = 42;
    let y = x;  // no problems
    print!("x is: {x}\n");
    print!("y is: {y}");
}
