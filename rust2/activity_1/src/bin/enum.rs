enum Color {
    RED,
    BLUE,
    WHITE,
    BLACK,
    GREEN,
    YELLOW,
}

fn main() {
    let color = Color::YELLOW;
    which_color(color);
}

fn which_color(color: Color) {
    match color {
        Color::RED => println!("\x1b[31mUsing color red\x1b[0m"),
        Color::BLUE => println!("\x1b[34mUsing color blue\x1b[0m"),
        Color::WHITE => println!("Using color white"),
        Color::BLACK => println!("Using black. Does not have ANSI"),
        Color::GREEN => println!("\x1b[32mUsing color green\x1b[0m"),
        Color::YELLOW => println!("\x1b[33mUsing color yellow\x1b[0m"),
    }
}
