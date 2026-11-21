use plotters::prelude::*;
fn main() {
    let root = BitMapBackend::new("images/3d-surface.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D Surface", ("sans-serif", 40))
        .build_cartesian_3d(-3.0..3.0, -3.0..3.0, -3.0..3.0)
        .unwrap();

    chart.configure_axes().draw().unwrap();

    let mut data = vec![];
    
    for x in (-25..25).map(|v| v as f64 / 10.0) {
        let mut row = vec![];
        for z in (-25..25).map(|v| v as f64 / 10.0) {
            row.push((x, (x * x + z * z).cos(), z));
        }
        data.push(row);
    }

    chart.draw_series(
        (0..49)
            .map(|x| std::iter::repeat(x).zip(0..49))
            .flatten()
            .map(|(x,z)| {
                Polygon::new(vec![
                    data[x][z],
                    data[x+1][z],
                    data[x+1][z+1],
                    data[x][z+1],
                ], &BLUE.mix(0.3))
            })
    ).unwrap();

}

