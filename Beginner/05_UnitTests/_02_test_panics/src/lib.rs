/*
    To RUN the tests we can simply type in the terminal this following Cargo command:
        $ cargo test

    We can test function that do not return anything using panic behaviour.
 */

mod shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn new1(radius: f32) -> Result<Circle, String> {
            if radius >= 0.0 {
                Ok(Circle{radius})
            } else {
                Err(String::from("Radius should be positive!"))
            }
        }

        pub fn new2(radius: f32) -> Circle {
            match radius {
                -10.0..=0.0 => panic!("Radius should be positive!"),
                ..= -10.0 => panic!("Radius is lesser than -10.0"),
                _ => Circle { radius }
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;  // super -> parent module

    #[test]
    fn check_smaller() {
        let c1 = shapes::Circle::new(2.0);
        let c2 = shapes::Circle::new(1.0);
        assert_eq!(c1.contains(&c2), true, "Message shown only if the test FAILS!");
    }

    #[test]
    fn check_not_bigger() {
        let c1 = shapes::Circle::new(1.0);
        let c2 = shapes::Circle::new(4.0);
        assert_eq!(!c1.contains(&c2), true);
    }

    #[test]
    fn not_creation() -> Result<(), String> {
        let c1 = shapes::Circle::new1(-1.0)?;  // ? is used to propagate the error
        Ok(())
    }

    #[test]
    #[should_panic]  // this test should panic when a negative radius is passed to the constructor
    fn not_creation_and_panic() {
        let some_circle = shapes::Circle::new2(-1.0);
    }

    #[test]
    #[should_panic(expected = "Radius is lesser than -10.0")]
    fn not_creation_and_panic1() {
        let some_circle = shapes::Circle::new2(-11.0);
    }
}