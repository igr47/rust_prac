use plotters::prelude::*;

// To add labels to axex we do so when creating chart context 
// After creating the chart context we use the set_label_area_size
// method to allocate space for the both axis. The parameters taken are 
// the enums LabelAreaPosition::Position and the the size we want to allocate 
// To create the title we use ChartBuilder caption which is a method after called 
// After the ChartBuilder constructor.

fn main() {
    let root_drawing_area = BitMapBackend::new("images/0.2.png", (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption("Figure Sample", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

}
