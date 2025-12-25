pub struct customer {
    pub name: String,
    pub id: u32,
    pub age:u8,
}
impl customer {
    pub fn new(name: String, id: u32, age:u8) -> Self {
        customer { name, id, age }
    }

    pub fn print_customer(&self) {
        println!("Customer Name: {}, ID: {}, Age: {}", self.name, self.id, self.age);
    }
}