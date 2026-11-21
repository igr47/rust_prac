use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::gif("images/animated.gif", (320, 100), 1_000).unwrap().into_drawing_area();

    for i in 0..=10 {
        root.fill(&WHITE).unwrap();
        root.draw(
            &Text::new(
                format!("{}", 10 - i), (100, 20), ("sans-serif", 80))
        ).unwrap();
        root.present().unwrap();
    }
}
