/*
    Like others languages we can use in Rust Enum type.
    Enums allow us to define a type by enumerating its variants.

    In Rust we can use implementation box to extend enum functionalities (as Structures)
 */

use crate::RentType::Airplane;
use crate::WeekDya::Wednesday;

enum WeekDya {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

enum RentType {
    Car(f32),
    Train(f32),
    Airplane(f32)
}

impl RentType {
    fn calculate_cost(&self) -> f32 {
        let total = match self {
            RentType::Car(km) => km * 1.20,
            RentType::Train(km) => km * 3.25,
            RentType::Airplane(km) => km * 50.0
        };
        total // return
    }
}

fn main() {
    let today = Wednesday;
    let yesterday = WeekDya::Tuesday;

    let rent = Airplane(100.0);
    println!("Renting an Airplane cost: {}", rent.calculate_cost());
}
