fn main() {
    // Integers
    //  - Signed: i8, i16, i32, i64
    //	- Unsigned: u8, u16,u32,u64
    let i1:i8 = 3;
    let i2:u128 = 100 * 1_000_000;
    println!("Max value for an unsigned integer of 8bit: {}", u8::MAX);

    // Floats
    //  - f32, f64
    let var = 1.10;  // by default f64 is used
    println!("Max value for a float of 64bit: {}", f64::MAX);
    println!("min value for a float of 64bit: {}", f64::MIN);

    // Booleans
    // - true, false
    println!("The value of the expression is: {}", 3 != 42);

    // Characters
    //  - Single letters
    //  - Digit
    //  - Unicode values
    //  - Escape sequences
    let c1 = 'F';
    let c2 = '0';
    let c3 = '+';
    let c4 = '\u{288A}';
    let c5 = '\"';
    println!("The value of c1 is {c1}, c2 is {c2}, c3 is {c3}, c4 is {c4} and c5 is {c5}");

    // Type Aliasing
    type Age = u8;
    let my_age: Age = 42;

    // Type Conversion (as operator)
    let float_var = 100.89;
    let int_var = float_var as i64;
    println!("The integer value is: {int_var}");
}
