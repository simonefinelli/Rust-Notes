/*
    HashMaps are structures that contain key-value pairs.
    Keys are unique and no duplicates are allowed, but the value can be duplicated.
    To have a logical similarity, we can think of HasMaps as dictionary: we could have more word (keys)
    with similar meaning (values).

    HashMaps are part of the Rust Standard Library.
 */

use std::collections::{hash_map, HashMap};

fn main() {
    // declare an HashMap
    let mut person: HashMap<&str, i32> = HashMap::new();
    // insert pairs
    person.insert("Gandalf", 300);
    person.insert("Bilbo", 111);
    person.insert("Frodo", 40);
    // access to elements
    println!("The age is {:?}", person.get("Bilbo").unwrap());  // unwarap because the get function will return an option enum type

    // check if a key exists using contains_key
    let find: bool = person.contains_key("Saruman");
    println!("The value of find is: {find}");
    // or using match
    match person.get("Frodo") {
        Some(value) => println!("The value of find is: {value}"),
        None => println!("The value not exist")
    }

    // iterates through keys
    for (name, age) in &person {  // we use the reference to ensure that the HashMap will be valid after the end of the loop
        println!("Person: {}, Age: {}", name, age);
    }

    // update a value
    person.insert("Frodo", 90);  // in this case the 40 is overwritten
    println!("The age is {:?}", person.get("Frodo").unwrap());

    // to avoid the overwriting behaviour we can use entry function
    let mut job: HashMap<&str, &str> = HashMap::new();
    job.entry("Bob").or_insert("Doctor");  // Doctor is insert only if for Bob entry no value already exists (otherwise no value updates)
    job.entry("Bob").or_insert("Engineer");  // no effects

}
