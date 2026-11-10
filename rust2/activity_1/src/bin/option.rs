struct Student {
    name: String,
    assignment: Option<i32>,
}

fn main() {
    let locker_assignment = Student {
        name: "John".to_owned(),
        assignment: None,
    };

    match locker_assignment.assignment {
        Some(val) => println!("Locker assignement number {:?}", val),
        None => println!("Locker assignment number not given"),
    };
}
