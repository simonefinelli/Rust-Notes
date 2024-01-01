mod product {
    use category::Category; // relative path
    pub struct Product {
        id: u64,
        name: String,
        price: f64,
        // category: category::Category  // relative path - to avoid this approach when it is too long
        category: Category               // we can declare the relative path using 'use' keyword (see row number 2)
    }

    mod category {  // a sub-module because the category can be have other functionalities (we could use an external module too)
        pub enum Category {  // a public element of a sub-module can be only accessed by parent module
            Electronics,
            Clothing,
            Books
        }
    }

    impl Product {
        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }

        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }
    }
}

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String
    }
}


mod order {
    struct Order {
        id: u64,
        product: crate::product::Product,  // crate is the root of absolut project path
        customer: crate::customer::Customer,  // we can use 'use' keyword too
        quantity: u32
    }

    impl Order {
        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 {
                0.1
            } else {
                0.0
            }
        }

        fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
}
