#[derive(Debug)]
enum Ticket {
    Backstage(String, u32),
    Vip(String, u32), 
    Standard(u32),
}

fn main() {
    let ticket = vec![
        Ticket::Backstage("Klein".to_owned(), 30),
        Ticket::Vip("Triss".to_owned(), 20),
        Ticket::Standard(10),
    ];

    for tick in ticket {
        match tick {
            Ticket::Backstage(name, amount) => println!("{:?}, {:?}", name, amount ),
            Ticket::Vip(name_2, amount_2) => println!("{:?}, {:?}", name_2, amount_2),
            Ticket::Standard(amount_3) => println!("{:?}", amount_3),
        };
    }
}
