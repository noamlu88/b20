pub struct loan{
    pub member_id: u32,
    pub isbn: u32,
    pub days: u32,
    approved: bool,
}


impl loan {
    pub fn new(member_id: u32, isbn: u32, days: u32) -> Self {
        loan {
            member_id,
            isbn,
            days,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    pub fn print_loan(&self) {
        println!(
            "Member ID: {}, ISBN: {}, Days: {}, Approved: {}",
            self.member_id, self.isbn, self.days, self.approved
        );
    }
}