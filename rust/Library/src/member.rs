pub struct member {
    pub id: u32,
    pub name: String,
    pub is_active: bool,
}

impl member {
    pub fn new(id: u32, name: String, is_active: bool) -> Self {
        member { id, name, is_active }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn print_member(&self) {
        println!("Member ID: {}, Name: {}, Active: {}", self.id, self.name, self.is_active);
    }
}