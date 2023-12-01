/*
    To take user input from terminal, in Rust we can use stdin().
 */
fn main() {
    println!("Enter a number: ");
    // get input
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read the input!");

    // check input
    let n:f64 = n.trim().parse().expect("The string is not a number!");

    // show entered input
    println!("The Number entered is: {n}");
}
