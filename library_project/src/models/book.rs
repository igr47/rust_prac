#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
    available: bool,
}

impl Book {
    pub fn new(title: &str, author: &str, isbn: &str) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
            available: true,
        }
    }

    // Public getter function 
    pub fn is_available(&self) -> bool {
        self.available
    }

    // pub(create): visisble anywhere in this create but not to the outside world 
    pub(crate) fn set_available(&mut self, value: bool) {
        self.available = value;
    } 
}
