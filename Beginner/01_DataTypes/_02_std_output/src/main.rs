/*
    To show string using standard output we can use the Macro
    print!
 */
fn main() {
    println!("Hello World");  // ! symbol define a Macro (metaprogramming)
    // macros are substituted by other code before the program is being executed
    print!("Printing without a new line ");
    // println!(42); // generate an error
    println!("\n{}", 42);
    let number = 42;
    println!("{number}");
    println!("My name is {} and I am {}!", "Simox", 1000);
    println!("The third arg is {2}, the second is {1} and the first is {0}!", 10, 20, 30);
    println!("The variable var2 is {var1}, the variable var2 is {var2} and the first value is {0}!", 10, var2=120, var1=130);
}
