/*
    To RUN the tests we can simply type in the terminal this following Cargo command:
        $ cargo test
 */

mod helpers;

use _03_integration_tests::{Category, Customer, Order, Product};

#[test]
fn test_total_bill_without_discount() {
    helpers::helper_function();
    let product = Product::new(1, String::from("Product1"), 42.0, Category::Electronics);
    let customer = Customer::new(1, String::from("Customer1"), String::from("email@service.com"));
    let order = Order::new(7, product, customer, 1);

    assert_eq!(format!("{:.2}", order.total_bill()), "46.20");
}

#[test]
fn test_total_bill_with_discount() {
    let product = Product::new(1, String::from("Product1"), 42.0, Category::Electronics);
    let customer = Customer::new(1, String::from("Customer1"), String::from("email@service.com"));
    let order = Order::new(8, product, customer, 10);

    assert_eq!(format!("{:.2}", order.total_bill()), "415.80");
}