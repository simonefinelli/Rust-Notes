/*
    In Rust we have to types of condition:
     - If/else
     - Match (Switch)
*/

fn main() {
    // If - Else
    let num = 42;
    if num == 42 {
        println!("This is the response to all the questions!");
    } else {
        println!("Wrong answer!");
    }

    // nested If
    let choice: i32 = 2;
    let mut value = "";
    if choice == 0 {
        value = "neutral";
    } else if choice > 0 {
        value = "positive";
    } else {
        value = "negative";
    }

    // Match
    let mark = 9;
    let mut evaluation:&str = "";
    match mark {
        0..=5 => {evaluation = "bad"},  // curly brackets to have a code block
        6..=8 => evaluation = "good",
        9..=10 => evaluation = "excellent",
        _ => evaluation = "No evaluation"  // default arg
    }
}
