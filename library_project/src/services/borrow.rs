use crate::services::library::Library;

pub fn process_borrow(lib: &mut Library, member_id: u32, isbn: &str) -> Result<(), String> {
    let book_available = lib
        .books
        .iter()
        .find(|b| b.isbn == isbn)
        .map(|b| b.is_available())
        .ok_or_else(|| format!("Book {} not found", isbn))?;

    if !book_available {
        return Err(format!("Book {} is already borrowed", isbn));
    }

    if lib.members.iter().all(|m| m.id != member_id) {
        return Err(format!("Member {} not found", member_id));
    }

    if let Some(book) = lib.find_book_mut(isbn) {
        book.set_available(false);
    }

    if let Some(member) = lib.find_member_mut(member_id) {
        member.borrowed_books.push(isbn.to_string());
    }

    Ok(())
}
