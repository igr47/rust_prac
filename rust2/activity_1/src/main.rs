fn main() {
    display_first_name(&"Elvis".to_string());
    display_last_name(&"Njagi".to_string());
}

fn display_first_name(name: &String) {
    println!("My first name is {}", name);
}

fn display_last_name(last_name: &String) {
    println!("My last name is {}", last_name);
}
