/*
  Compound Data Types are structures that can hold one or more values.

  They are divided into:
   - &str and String
   - Arrays
   - Vectors
   - Tuples
   - Empty Tuple
 */

fn main() {
    // &str and String
    let string1:&str = "Immutable string!";
    let string2:String = String::from("Mutable string!"); // shrink and grow
    println!("{string1} and {string2}");

    // Arrays (fixed length)
    let arr:[i32; 5] = [1, 2, 3, 4, 5];  // elements of the same type
    let num = arr[0];
    println!("The first number in the array is: {num}", );

    let arr1 = [-1; 100]; // creates an array with length 100 and default value -1

    // Vectors (not fixed length)
    let vec1: Vec<f64> = vec![1., 3., 5.];  // elements of the same

    // Tuples
    let tuple = ("Age", 42, "Height", 179.9);
    let some_value = tuple.2;
    let (age, age_value, height, h_value) = tuple;  // unpack
    println!("{some_value} == {age_value}");



}
