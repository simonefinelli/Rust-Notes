fn main() {
    /*
        Rules for naming variables:
         - Only letters, digits and underscores
         - Should begin with letter or underscore
         - Case Sensitive
     */

    // variables are created put the `let` keyword before the name of the variables itself.
    let x = 42;  // note that the Rust compiler have to know the type of the variable, so it trys to infer it.
    let y = 42.42;
    let z:f64 = 10E25;  // we can also specify the type
    println!("The values of variables are: {}, {}, {}", x, y, z);

    // MUTABILITY - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    /* ATTENTION: by default, variables are IMMUTABLE, so we can't change their values.
    To use MUTABLE variables we have to put `mut` keyword before the name of the variable. */
    // x = 42;  // ERROR!
    let mut j = 0;
    j = 20;

    // SCOPE - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    {
        let k = 10;
    }
    // let l = k;  // ERROR: k is only visible in the scope of the block above

    // SHADOWING - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    let tmp = 100;
    let tmp = tmp + 1000;  // tmp shadows the first tmp var: only the second variable is going to persist
    println!("tmp is {tmp}");

    let tmp1 = 1.0;
    {
       let tmp1 = 1;  // shadowing: a new var with a previous utilized name is created.
                           // This var is only visible in this scope, so no conflicts are created
    }

    // CONSTANTS - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    const PI:f32 = 3.14;  // the type must be explicated

    // The difference with normal var is that they cannot be redeclared as mutated using shadowing:
    // let mut PI = 3.1415;  // compilation error;
    println!("{PI}");
}
