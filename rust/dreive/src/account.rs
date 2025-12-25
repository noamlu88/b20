use crate::customer::customer;

pub struct account {
    pub number: u32,
    pub balance: i32,
    pub owner: customer,
}
impl account {
    pub fn new(number: u32, balance: i32, owner: customer) -> Self {
        account { number, balance, owner }
    }

    pub fn print_account(&self) {
        println!("Account Number: {}, Balance: {}, Owner: {}", self.number, self.balance, self.owner.name);
    }

    pub fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: i32) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
    pub fn change_owner(&mut self, new_owner: customer) {
        self.owner = new_owner;
    }
}