use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("images/split-drawing-area.png", (600, 400)).into_drawing_area();

    let (left, right) = root.split_horizontally((25).percent_width());

    left.fill(&YELLOW).unwrap();

    let (upper, lower) = right.split_vertically(240);

    upper.fill(&BLUE).unwrap();

    let lower_area = lower.split_evenly((2, 3));

    for (id, area) in lower_area.into_iter().enumerate() {
        area.fill(&Palette99::pick(id)).unwrap();
    }
}
