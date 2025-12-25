pub struct transaction {
    pub from_account: u32,
    pub to_account: u32,
    pub amount: i32,
    pub approved: bool,
}
impl transaction {
    pub fn new(from_account: u32, to_account: u32, amount: i32) -> Self {
        transaction { from_account, to_account, amount, approved: false }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    pub fn print_transaction(&self) {
        println!("Transaction from Account: {} to Account: {} of Amount: {}. Approved: {}", 
                 self.from_account, self.to_account, self.amount, self.approved);
    }
}