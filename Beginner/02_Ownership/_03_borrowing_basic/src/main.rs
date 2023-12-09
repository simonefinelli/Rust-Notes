/*
    Borrowing in a concept that is tied with Ownership.
    Borrowing allows to interact with data in a safe and efficient manner.
    This concept can be summarized with:
     - Establish a reference to some data (like pointers, but with some specific rules);
     - References do not assume ownership of values (think at the word borrow).

    Borrowing is useful to prevent unnecessary memory usage (instead of copy every time the lvalue).

    There are two rules that references must follow:
     - At any time, we can have either one mutable reference or any number of immutable references.
     - References must always be valid.

    Using these rules, we resolve, in general:
     - Data race
     - Dangling references

    TIP: In Rust references are created using & symbol.
 */

fn main() {
    let mut vec1 = vec![1, 2, 3, 4, 5];
    let ref1 = &mut vec1;  // mutable reference allows to borrow the data and also change it.
    let ref2 = &mut vec1;  // violate the first rule
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);  // error: cannot borrow `vec1` as mutable more than once at a time

    // Note: if we comment the line 24, the rust compiler will still compile: this because the compiler
    // counts the active period of reference (scope of reference).
}
