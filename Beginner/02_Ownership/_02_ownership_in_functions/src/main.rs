/*
    Ownership in Functions.

 */

fn main() {
    /*
        In this case the ownership is moved to the function.
    */
    let v1 = vec![1, 2, 3, 4, 5]; // content allocated on the heap (on the stack only vector length, capacity and the pointer)
    // takes_ownership(v1);
    // println!("The vector contain: {:?}", v1);  // error: borrow of moved value: `v1` (same effect of assigning)

    takes_ownership1(v1.clone());  // using clone() we create a copy of the lvalue
    println!("1) The vector contain: {:?}", v1);  // now work!


    /*
        In this case the ownership is get from the function.
    */
    let v2 = gives_ownership();
    println!("4) The vector contain: {:?}", v2);  // now work!


    /*
        In this case the ownership is moved from and returned to the called function.
    */
    let v3 = takes_and_gives_ownership(v1);
    println!("5) The vector contain: {:?}", v3);  // now work!


    /*
        Basic data type as int, float, double, bool, et. are passed always by copy.
     */
    let x = 42;
    foo(x);
    println!("The number value in main is: {x}");

}

fn takes_ownership(mut vec: Vec<i32>) {
    /*
        In this case the ownership of the var v1 is MOVED to var vec (v1 reference is invalidated).
        Here, vec lives in this function, but when the function has done its job (pop from stack)
        either the vec is dropped (cleaned from heap).
     */
    vec[0] = 100;
    println!("2) The vector contain: {:?}", vec);
}

fn takes_ownership1(mut vec: Vec<i32>) {
    vec[0] = 100;
    println!("3) The vector contain: {:?}", vec);  // note the output (edits will be see only in the function scope)
}

fn gives_ownership() -> Vec<i32> {
    /*
        The ownership will be transfer to the called function (main)
     */
    return vec![10, 11, 12];  // tip: return can be omitted
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(1000);
    return vec;  // return and ; can be omitted
}

fn foo(mut n: i32) {
    /* the function as its own COPY of n in the stack. */
    n = 3;
    println!("The number value in foo function is: {n}");
}