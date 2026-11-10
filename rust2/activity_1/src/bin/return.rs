#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self { name: name.to_string(), age })
        }  else {
            Err("Age must be at least 21")
        }
    }
}

fn main () {

    let person = Adult::new(22, "Klein");
    let person_2 = Adult::new(17, "Aundrey");

    match person {
        Ok(person) => println!("Name: {:?}, Age {:?}", person.name, person.age),
        Err(e) => println!("Sory but You are not old enough{:?}", e),
    };

    match person_2 {
        Ok(person_2) => println!("Name: {:?}, Age: {:?}", person_2.name, person_2.age),
        Err(e) => println!("Sory but You are not old enough{:?}", e),
    };
    
}
