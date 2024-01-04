use category::Category; // relative path
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    // category: category::Category  // relative path - to avoid this approach when it is too long
    category: Category               // we can declare the relative path using 'use' keyword (see row number 2)
}

mod category;

impl Product {
    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }

    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }
}