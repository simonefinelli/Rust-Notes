pub struct Customer {
    id: u64,
    name: String,
    email: String
}

impl Customer {
    pub fn new(id: u64, name: String, email: String) -> Customer {
        Customer {
            id,
            name,
            email
        }
    }
}
