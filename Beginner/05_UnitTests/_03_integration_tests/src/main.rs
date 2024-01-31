/*
    Here we show how to use the modules created in Project _03_organizing_modules.
 */

use _03_integration_tests::{Customer, Product, Category, Order};  // TIP: _05_utilize_modules is the package name

fn main() {
    // we can create an instance of a product using the struct
    // let product1 = Product {
    //     id: 1000,
    //     name: String::from("Thing1"),
    //     price: 550.50,
    //     category: Category::Electronics
    // }
    // using this approach we get some erros (Field '' private ...)
    // we can avoid this put 'pub' keyword to all elements in the library Product structure.
    // this is the NOT the right approach!

    // To keep struct members private, we have to implement 'new' method.
    let product1 = Product::new(1000, String::from("Thing1"), 550.50, Category::Electronics);
    let customer1 = Customer::new(2000, String::from("Customer One"), String::from("customer1@example.com"));
    let order = Order::new(3000, product1, customer1, 5);

    println!("The total is: ${}", order.total_bill());
}
