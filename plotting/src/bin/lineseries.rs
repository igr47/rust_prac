use plotters::prelude::*;

fn main() {
    let root_drawing_area = BitMapBackend::new("images/0.3.png", (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Line Plot Demo", ("sans-serif", 40))
        .build_cartesian_2d(-10..10, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new((-10..=10).map(|x| (x, x * x)), &GREEN)
    ).unwrap();
}
