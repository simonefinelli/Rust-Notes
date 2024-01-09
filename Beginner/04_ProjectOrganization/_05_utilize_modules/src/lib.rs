mod product;
mod customer;
mod order;

// we use this notation to export something from a Module.
// Doing this we can use Product, in main
pub use product::{Product, Category};  // the 'pub' keyword is used to re-export an item from a module without the need to make him public.
pub use customer::Customer;
pub use order::Order;
