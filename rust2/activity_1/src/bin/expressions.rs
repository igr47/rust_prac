fn main () {
    let value = 110;
    let is_greater = value > 100;
    print_message(is_greater);
}

fn print_message(cond: bool) {
    match cond {
        true => println!("it's big"),
        _ => println!("It's small"),
    }
}
