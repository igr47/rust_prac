struct Person {
    name: String,
    fav_color: String,
    age: u32,
}

fn main() {
    let person = vec![
        Person {
            name: "Klein Moretti".to_owned(),
            fav_color: "Yellow".to_owned(),
            age: 21,
        },
        Person {
            name: "Derick Berg".to_owned(),
            fav_color: "Orange".to_owned(),
            age: 18,
        },
        Person {
            name: "Aunrey Hall".to_owned(),
            fav_color: "Green".to_owned(),
            age: 10,
        },
    ];

    for p in person {
        if p.age <= 10 {
            println!("Name: {:?}, Favourite Color: {:?}, Age: {:?}", p.name, p.fav_color, p.age);
        }
    }
}
