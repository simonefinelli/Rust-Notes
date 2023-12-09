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
     - Data race: occurs when there are multiple references to a the same data, with at least one
            reference updating the data, and there is no mechanism to synchronise access to the data;
     - Dangling references

    TIP: In Rust references are created using & symbol.
 */

fn main() {
    /** Only one mutable reference **/
    let mut vec1 = vec![1, 2, 3, 4, 5];
    let ref1 = &mut vec1;  // mutable reference allows to borrow the data and also change it.
    // let mut ref2 = &mut vec1;  // violate the first rule
    // println!("ref1: {:?}, ref2: {:?}", ref1, ref2);  // error: cannot borrow `vec1` as mutable more than once at a time

    // Note: if we comment the line 25 (and uncomment the line 24), the rust compiler will still compile:
    // this because the compiler counts the active period of reference (scope of reference). In fact,
    // without println in line 25, the scope of ref1 is limited to line 23 and the scope of ref2
    // is limited to line 24 (there is no overlapping - we have only one mutable reference).

    /** Many immutable references **/
    let vec2 = vec![10, 20, 30, 40, 50];
    let ref2 = &vec2;  // immutable reference
    let ref3 = &vec2;  // immutable reference
    let ref4 = &vec2;  // immutable reference
    println!("ref1: {:?}, ref2: {:?}, ref3: {:?}", ref2, ref3, ref4);  // no error

    /** Mutable and Immutable reference (Avoiding Data race) **/
    let mut vec3 = vec![100, 200, 300, 400, 500];
    let ref5 = &vec3;  // immutable reference
    let ref6 = &vec3;  // immutable reference
    // let ref7 = &mut vec3;  // mutable reference -> error: cannot borrow `vec3` as mutable because it is also borrowed as immutable
    // println!("ref1: {:?}, ref2: {:?}, ref3: {:?}", ref5, ref6, ref7);
    // This is a violation because at any given time we can have one mutable reference OR
    // any immutable references, but NOT at THE SAME TIME (avoid data race)

    /** Dangling reference **/
    let vec4: &Vec<i32> = {
        let vec5 = vec![1000, 2000];
        &vec5  // error: vec5 does not live long enough (dangling reference inside the inner scope):
               // at the end of the scope vec5 will be dropped from the heap and no more exist.
               // This could be a dangling reference, so Rust prevents it.
    };
    println!("vec4: {:?}", vec4);
}
