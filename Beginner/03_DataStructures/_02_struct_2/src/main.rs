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

    Another kind of functions that we can use in a implementation block are Associated functions (
    static functions in other programming languages).

    A special function is 'new': the constructor function.
 */

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32
}

impl Car {  // extending functionality of Car
    fn new(name: String, fabric_year: u32) -> Self {  // new is a static function
        Self {
            owner: name,
            year: fabric_year,
            fuel_level: 0.0, // default value
            price: 0
        }
    }

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

    fn early_insurance() -> u32 {  // associated functions do not take self as parameter
        780
    }
    fn selling_prince(&self) -> u32 {
        self.price + Car::early_insurance()  // associated methods can be called using :: notation
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

    let car_constructor = Car::new("Simox".to_string(), 2023);  // this could avoid to write
                                                                               // the code between 61 and 66
}


