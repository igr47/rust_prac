/*struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    fn boiling() -> Self {
        Self { degrees_f: 212.0 }
    }
    fn show_temperature(&self) {
        println!("{:?} degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature{ degrees_f: 99.9 };
    hot.show_temperature();

    let cold = Temperature::freezing();
    cold.show_temperature();
    let boiling = Temperature::boiling();
    boiling.show_temperature();
}*/
#[derive(Debug)]
enum Color {
    RED,
    BLUE,
    BLACK,
    GREEN,
}

struct Box {
    color: Color,
    width: u32,
}

impl Box {
    fn create_new(col: Color, w: u32) -> Self {
        Self { color: col, width: w }
    }

    fn display(&self) {
        println!("Box dimension: {:?}, Box Color: {:?}", self.width, self.color)
    }
}

fn main() {
    let box_1 = Box::create_new(Color::BLUE, 12);
    box_1.display();
}
