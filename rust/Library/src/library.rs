pub struct libary{
    pub name:String,
    members:Vec<member>,
    books:Vec<book>,
    loans:Vec<loan>,

}

impl libary {
    pub fn new(name: String) -> Self {
        libary {
            name,
            members: Vec::new(),
            books: Vec::new(),
            loans: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: member) {
        self.members.push(member);
    }

    pub fn add_book(&mut self, book: book) {
        self.books.push(book);
    }

    pub fn add_loan(&mut self, loan: loan) {
        self.loans.push(loan);
    }

    pub fn print_library(&self) {
        println!("Library Name: {}", self.name);
        println!("Members:");
        for member in &self.members {
            member.print_member();
        }
        println!("Books:");
        for book in &self.books {
            book.print_book();
        }
        println!("Loans:");
        for loan in &self.loans {
            loan.print_loan();
        }
    }
}