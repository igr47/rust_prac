use library_project::{Book, Library, Member};
use library_project::utils::titles_match;

fn main() {
    let mut lib = Library::new();

    lib.add_book(Book::new("The Rust Book", "Steve Klabnick", "978-17185"));
    lib.add_book(Book::new("Programming Rust", "Jim Blandy", "978-41907"));

    lib.add_member(Member::new("Alice", 1));
    lib.add_member(Member::new("Bob", 2));

    match lib.borrow_book(1, "978-17185") {
        Ok(_) => print!("Alice borrowed The Rust Book"),
        Err(e) => print!("Error: {}", e),
    }

    match lib.borrow_book(2, "978-1718500440") {
        Ok(_) => println!("Bob borrowed it"),
        Err(e) => println!("Expected failure: {}", e),
    }

    // Uses a private helper indirectly through a public function
    println!(
        "Titles match: {}",
        titles_match("  the rust book ", "The Rust Book")
    );
}
