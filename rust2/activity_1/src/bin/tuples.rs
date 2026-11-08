fn main() {
    let (_x, y) = coords();

    if y > 5 {
        println!("Greater than 5.");
    } else if y < 5 {
        println!("Less then 5");
    } else {
        println!("Equal to 5");
    }
}

fn coords() -> (i32, i32) {
    (7, 9)
}
