/*
    We can use external modules (packages) to re-use code, algorithms, and so on.
    External packages can be found on crates.io.
    To install external resource we can put his name after [dependencies] in Cargo.toml.

    For instance, here we use 'array_tool' packages to perform specific operation on vectors.
 */

use _06_use_external_packages::{Product, Category};
use array_tool::vec::Intersect;  // import interested function

fn main() {
    let product1 = Product::new(1001, String::from("Thing1"), 100.99, Category::Electronics);
    let product2 = Product::new(1002, String::from("Thing2"), 99.90, Category::Electronics);
    let product3 = Product::new(1003, String::from("Thing3"), 74.50, Category::Electronics);

    let group1: Vec<&Product> = vec![&product1, &product2];
    let group2: Vec<&Product> = vec![&product3, &product1];

    // use intersection functionality of the array_tool package
    let intersection = group1.intersect(group2);

    println!("The intersection is: {:?}", intersection);
}
