struct Groceries {
    id: i32,
    quantity: i32,
}
fn main() {
    let item = Groceries {
        id: 1,
        quantity: 32,
    };

    display_quantity(&item);
    display_id(&item);
}

fn display_quantity(qty: &Groceries) {
    println!("Quantity: {}", qty.quantity);
}

fn display_id(id: &Groceries) {
    println!("Id: {}", id.id);
}
