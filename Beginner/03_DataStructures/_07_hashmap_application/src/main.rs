/*
    Usage of the HashMap in a real-world case.

    Goal: Get the frequencies of an HasMap elements.
 */
use std::collections::HashMap;

fn main() {
    let vec1 = vec![1,2,6,4,8,1,0,0,8,1,4,5,9];
    let mut hashmap1: HashMap<i32, i32> = HashMap::new();

    for i in &vec1 {
        let freq: &mut i32 = hashmap1.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("{:?}", hashmap1);
}
