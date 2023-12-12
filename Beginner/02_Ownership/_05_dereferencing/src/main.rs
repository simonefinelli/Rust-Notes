/*
    De-referencing is the process of accessing the value pointed to by a reference or pointer.

    This concept is related to the Borrowing. In fact, to manipulate the data, we need to de-reference
    the pointer/reference first.
 */

fn main() {
    // Allocated data on the Stack
    let mut data = 42;
    let ref1 = &mut data;
    let deref_ref1 = *ref1; // Attention: the var 'deref_ref1' now contains a COPY of the data
                                 // pointed by the ref (one at a different memory location)
    *ref1 = 24;  // we can use de-referencing to change data
    println!("Some data: {data}, Deref data: {deref_ref1}");


    // Allocated data on Heap behaves differently
    let mut heap_data = vec![1, 2, 3, 4, 5];
    let ref2 = &mut heap_data;
    // let deref_heap_data = *ref2; // error: cannot move out of `*ref2` which is behind a mutable reference
                                    // error occurs because a change of ownership
                                    // secondly, moving a value from a mutable reference could potentially leave the reference invalid
    // use .clone() to avoid this situation
    let deref_heap_data = ref2.clone();  // using clone() on a reference creates a own copy
                                                   // (whether the reference is mutable or immutable)


    // A mutable reference to some data can be copied only once
    let move_out = ref2;
    // let move_out_again = ref2;  // error: use of moved value: `ref2`. This mean that
                                   // the mutable reference cannot be copied, but can only be moved
                                   // Remember that references (pointers of addresses) reside in stack,
                                   // (data in stack, when assigned, are copied not moved), but mutable references
                                   // are an exception (only moved, but not copied - according to borrowing rule:
                                   // we cannot have multiple mutable references).

    // no problems with immutable references
    let ref2 = &heap_data;
    let ref3 = ref2;
    let ref4 = ref2;

    // In conclusion: mutable references when assigned are moved, while immutable references are copied!
}
