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