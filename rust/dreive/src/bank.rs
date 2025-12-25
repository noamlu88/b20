use crate::account::account;
use crate::transaction::transaction;

pub struct bank {
    pub name: String,
    pub accounts: Vec<account>,
    pub transactions: Vec<transaction>,
}

impl bank {
    pub fn new(name: String) -> Self {
        bank { name, accounts: Vec::new(), transactions: Vec::new() }
    }

    pub fn add_account(&mut self, account: account) {
        self.accounts.push(account);
    }
    pub fn print_bank(&self) {
        println!("Bank Name: {}", self.name);
        for account in &self.accounts {
            account.print_account();
        }
        for transaction in &self.transactions {
            transaction.print_transaction();
        }
    }
    pub fn find_account_index(&self, number: u32) -> i32{
        for (index, account) in self.accounts.iter().enumerate() {
            if account.number == number {
                return index as i32;
            }
        }
        -1
    }
    pub fn transfer(&mut self, from: u32, to: u32, amount: i32) -> bool{
        let from_index = self.find_account_index(from);
        let to_index = self.find_account_index(to);
        if from_index == -1 || to_index == -1 {
            return false;
        }
        let from_index = from_index as usize;
        let to_index = to_index as usize;

        if self.accounts[from_index].withdraw(amount) {
            self.accounts[to_index].deposit(amount);
            let mut transaction = transaction::new(from, to, amount);
            transaction.approve();
            self.transactions.push(transaction);
            return true;
        }
        false
    }

}