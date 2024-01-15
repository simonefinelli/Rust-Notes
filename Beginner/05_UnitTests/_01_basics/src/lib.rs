// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// #[cfg(test)]  // configuration attribute: this ensures that the following code is compiled ad used only in cargo test mode.
// mod test {
//     use super::*;
//
//     #[test]  // used to mark test functions
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

/*
    To RUN the tests we can simply type in the terminal this following Cargo command:
        $ cargo test

    TIP: only focus on the test section! We can view something like this:
         test tests::check_not_bigger ... ok
         test tests::check_smaller ... ok
         test tests::not_creation ... FAILED
 */

mod shapes {
    use std::sync::mpsc::RecvError;

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
}

/*
    We can use other assection statement, like:
        assert_ne!()  -> the opposite of assert_eq!()
        assert() -> check boolean value, true for OK, false for FAIL
 */