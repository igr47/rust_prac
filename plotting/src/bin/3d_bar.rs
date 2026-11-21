use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("images/3d-bar.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D BAR DEMO", ("sans-serif", 40))
        .build_cartesian_3d(0i32..10, -10i32..10, 0i32..10)
        .unwrap();

    chart.configure_axes().draw().unwrap();

    chart.draw_series(
        (0i32..10i32)
            .map(|x| std::iter::repeat(x).zip(0i32..10))
            .flatten()
            .map(|(x,z)| {
                Cubiod::new([(x, 0, z), (x + 1, x - z, z + 1)], BLUE.filled(), &BLACK)
            })
    ).unwrap();
}
