/*
    Rust allows us to extend Structures adding some extra functionalities.

    To extend the functionality of the struct Car we can use the 'impl' box.
    All the functions indicated into 'impl' box can be considered a instance method if respect the
    following rules:
     - the function must be inside an implementation block;
     - the first parameter must be self keyword. There are three forms of self:
        - immutable reference: &self (used to access the data, without modifying it);
        - mutable reference: &mut self (modify data);
        - take and return ownership (see sell method).
 */

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32
}

impl Car {  // extending functionality of Car
    fn display_car_info(&self) {  // now the function is a method of a Car instance, so we can use 'self' to access the instance.
        println!(
            "Owner: {}, Year: {}, fuel: {}, price: {}",
            self.owner, self.year, self.fuel_level, self.price);
    }

    fn refuel(&mut self, liters: f32) {
        self.fuel_level += liters;
    }

    fn sell(self) -> Self {  // Self with capital letter indicates the struct Car (we can use Car also)
        self
    }
}

fn main() {
    let mut personal_car = Car {
        owner: String::from("SimoX"),
        year: 2023,
        fuel_level: 50.0,
        price: 30_000,
    };

    personal_car.display_car_info();
    // display_car_info(&personal_car); // error: cannot find function `display_car_info` in this scope

    personal_car.refuel(20.0);

    let new_owner = personal_car.sell();
    new_owner.display_car_info();
    // personal_car.display_car_info();  // now not allowed anymore - error: borrow of moved value: `personal_car`
}


