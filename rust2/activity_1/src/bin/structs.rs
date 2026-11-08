#[derive(Debug)]
enum Flavour {
    VANILLA,
    CHOCOLATE,
    STRAWBERRY,
    LEMON,
}
struct Drink {
    flavour: Flavour,
    ounce: String,
}

fn main() {
    let drink = Drink {
        flavour: Flavour::VANILLA,
        ounce:  "Mild".to_string(),
    };
    print_flavours(&drink);
}

fn print_flavours(drink: &Drink) {
    match drink.flavour {
        Flavour::VANILLA => println!("Flavour: {:?}, Ounce: {:?}", drink.flavour, drink.ounce),
        Flavour::CHOCOLATE => println!("Flavour: {:?}, Ounce: {:?}", drink.flavour, drink.ounce),
        Flavour::STRAWBERRY => println!("Flavour: {:?}, Ounce: {:?}", drink.flavour, drink.ounce),
        Flavour::LEMON => println!("Flavour: {:?}, Ounce: {:?}", drink.flavour, drink.ounce),
    }
}
