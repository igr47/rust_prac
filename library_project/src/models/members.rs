#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub id: u32,
    pub borrowed_books: Vec<String>,
}

impl Member {
    pub fn new (name: &str, id: u32) -> Self {
        Member {
            name: name.to_string(),
            id,
            borrowed_books: Vec::new(),
        }
    }
}
