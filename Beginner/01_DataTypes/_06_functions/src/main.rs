/*
    Functions are the elements that can allow to isolate code (for organization) and/or reusing.
    Functions are declared using the 'fn' keyword. They can accept all sort of parameters and are
    defined like variables.

    Note: naming convention for functions' names is snake_case!

    TIP: Unlike C/C++ a function can be declared after the main function, without using prototype.
 */

fn main() {
    foo("hello", 42);  // to call the function
    let d = doo(3.5, 1.9);
    let r = boo(42, 7);
    println!("Division: {d}\nMultiplication: {}, {}", r.0, r.1);
}

fn foo(s: &str, n: i32) {
    println!("{s}, {n}");
}

fn doo(n1: f32, n2: f32) -> f64  {  // the return value is specified with ->
    (n1 / n2) as f64  // the last line in function can be used for return (and we do not are forced to use a ;)
    // return (n1 / n2) as f64;  // this work as other programming languages
}

fn boo(v1: i32, v2: i32) -> (i32, i32) {  // we can return multiple values using tuples
    (v1 * v2, v1 * v2)
}