/*
    Borrowing Rules recap:
     - At any time, we can have either one mutable reference or any number of immutable references.
     - References must always be valid.
 */


fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    borrow_ownership(&vec1);
    println!("Vector 1 into the Main is: {:?}", vec1);  // no problems

    let mut vec2 = vec![10, 20, 30, 40, 50];
    borrow_mut_ownership(&mut vec2);
    println!("Vector 2 into the Main is: {:?}", vec2);
}

fn borrow_ownership(vec: &Vec<i32>) {
    println!("The vector into the function borrow_ownership: {:?}", vec);
}

fn borrow_mut_ownership(vec: &mut Vec<i32>) {
    vec.push(60);
}

/**
    To create and transfer ownership using a function, we have to use the ownership method.
    In fact, wWhen we want to create an object and return it to the caller, we must transfer the
    ownership (and we cannot borrow it: a dangling pointer occurs because the heap memory is cleaned-up
    and the reference will not point to a valid memory location anymore).
*/
// Attention: Wrong approach!
// fn gives_ownership() -> &Vec<i32> {
//     let vec = vec![100, 200];
//     return &vec;
// }

// Right approach!
fn gives_ownership() -> Vec<i32> {
    let  vec = vec![100, 200];
    return vec;
}
