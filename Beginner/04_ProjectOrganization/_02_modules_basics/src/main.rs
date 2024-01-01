/*
    In this project we will create a Library to handle an Online Store.

    In particular we use Modules to separate logically the library: Products, Customers, etc.
    To create a Module we can use the 'mod' keyword follower by the name of the module. So a Module in Rust is code
    construct and not a file (as other programming languages).

    Note that the elements in a Module in Rust by default are private, so if some elements have to be accessed by other
    modules the elements' visibility modifier can be set explicit to public, using 'pub' keyword.
    Also, we can make an entire module public (if it have to expose all), but remember that, make a module public do not
    automatically convert all elements of it public too (for each element we still have to use the pub keyword).
 */
fn main() {
    println!("Hello, world!");
}
