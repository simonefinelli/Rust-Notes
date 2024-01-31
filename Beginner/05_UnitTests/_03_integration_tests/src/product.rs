pub use category::Category;

mod category;

// relative path
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    // category: category::Category  // relative path - to avoid this approach when it is too long
    category: Category               // we can declare the relative path using 'use' keyword (see row number 2)
}

impl Product {
    pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
        Product {
            id,  // we should write id::id, but the names are identical in these cases
            name,
            price,
            category,
        }
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }

    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }
}