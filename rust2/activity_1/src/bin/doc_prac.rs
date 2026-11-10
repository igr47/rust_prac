fn main() {
    let name = "Klein".to_owned();
    return_upper(&name);
    return_lower(&name);
}

fn return_upper(name: &str) {
    println!("In small case{:?}", name.to_uppercase());
}

fn return_lower(name: &str) {
    println!("In lower case{:?}", name.to_lowercase());
}
