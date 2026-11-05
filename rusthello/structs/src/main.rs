// Structs are used to name and collect related data types together.
fn main() {
    struct Book {
        title : String,
        auther: String,
        pages:  u32,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user_1: User = User { active: true, username: String::from("Elvis Star"), email: String::from("Elvis"), sign_in_count: 1, };
    user_1.email = String::from("another_email.com");
    println!("New user email {}", user_1.email);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }
    println!("Hello, world!");
}
