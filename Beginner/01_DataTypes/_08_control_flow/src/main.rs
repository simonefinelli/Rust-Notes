/*
    The are three principal structures to do loops in Rust.
       - Loop: simple way to have a cycling code.
       - For loop
       - While loop
 */

fn main() {
    /*** Loop ***/
    loop {
        println!("Hello, world!");
        break; // to exit from the loop
    }
    // exit form an nested loop
    'label: loop {
        loop {
            println!("Hello, world 2!");
            break 'label;  // to exit also from the inner loop
        }
    }
    // assign value from a loop
    let var = loop {
        break 5;
    };

    /*** For Loop ***/
    let vector = vec![10, 20, 30, 40, 50];
    for element in vector {
        print!("{element} ");
    }

    /*** while loop ***/
    let mut count = 0;
    while count < 10 {
        println!("Hello, world 3!");
        count = count + 1;
    }
}
