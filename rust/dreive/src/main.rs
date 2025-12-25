mod bank;
mod customer;
mod account;
mod transaction;

fn main() {
    use crate::bank::bank;
    use crate::customer::customer;
    use crate::account::account;
    use crate::transaction::transaction;

    let bank = bank::new(String::from("Madar Bank"));
    bank.print_bank();

    let customer = customer::new(String::from("Alice"), 1, 30);
    customer.print_customer();

    let account = account::new(1, 5000, customer);
    account.print_account();

    let transaction = transaction::new(1, 2, 1500);
    transaction.print_transaction();
}