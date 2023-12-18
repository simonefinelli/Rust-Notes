/*
    To group related data together we can use Structs.
 */

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32
}

fn main() {
    let new_car = Car {
        owner: String::from("SimoX"),
        year: 2023,
        fuel_level: 100.0,
        price: 30_000,
    };

    let year = new_car.year;
    // new_car.fuel_level = 99;  // error: by default elements are immutable

    let mut new_car = Car {
        owner: String::from("SimoX"),  // remember: heap allocated
        year: 2023,
        fuel_level: 100.0,
        price: 30_000,
    };
    new_car.fuel_level = 80.0;
    // let owner = new_car.owner;  // ATTENTION: change ownership (transferred to owner variable)
    // println!("The owner is: {}", new_car.owner);  // error: borrow of moved value: `new_car.owner`
                                                     // this phenomena is called Partial Move
    let owner1 = new_car.owner.clone();
    println!("The owner is: {}", new_car.owner);  // now works

    // we can create a new Struct starting from another
    // let new_car2 = Car {
    //     ..new_car
    // };
    // println!("{}", new_car2.owner);
    // println!("{}", new_car.owner);  // error: borrow of moved value: `new_car.owner`
    //                                 // heap allocate variables transfer the ownership

    // we can create a new Struct starting from another
    let new_car3 = Car {
        owner: "new_owner".to_string(),
        ..new_car
    };
    println!("{}", new_car3.owner);
    println!("{}", new_car.owner);  // no error now


}
