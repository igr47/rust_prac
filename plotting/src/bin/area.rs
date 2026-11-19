use plotters::prelude::*;

fn main() {
    let root_drawing_area = BitMapBackend::new("images/0.32.png", (600, 400)).into_drawing_area();
    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption("Area Demo", ("sans-serif", 40))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .build_cartesian_2d(0..10, 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

    ctx.draw_series(AreaSeries::new((0..).zip(data.iter().map(|x| *x)), 0, &RED.mix(0.2)).border_style(&RED)).unwrap();

}
