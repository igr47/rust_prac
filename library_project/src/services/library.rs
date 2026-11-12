use crate::models::book::Book;
use crate::models::members::Member;
use crate::services::borrow;

pub struct Library {
    pub books: Vec<Book>,
    pub members: Vec<Member>,
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new(), members: Vec::new(), }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.push(member);
    }

    // Delegates the actual borrow logic to services::borrow
    pub fn borrow_book(&mut self, member_id: u32, isbn: &str) -> Result<(), String> {
        borrow::process_borrow(self, member_id, isbn)
    }

    pub(crate) fn find_book_mut(&mut self, isbn: &str) -> Option<&mut Book> {
        self.books.iter_mut().find(|b| b.isbn == isbn)
    }

    pub(crate) fn find_member_mut(&mut self, id: u32) -> Option<&mut Member> {
        self.members.iter_mut().find(|m| m.id == id)
    }
}
