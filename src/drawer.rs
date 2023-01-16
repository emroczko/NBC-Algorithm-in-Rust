use plotters::prelude::*;

pub fn draw_clustered_data(data: &Vec<(f64, f64, i32)>, file_name: &str) {
    let file_path = &format!("plotted/{}.png", file_name);
    let root_area = BitMapBackend::new(file_path, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Clustering results", ("sans-serif", 40))
        .build_cartesian_2d(-3.0..3.0, -1.5..2.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(data.iter().map(|(coord_x, coord_y, group)| {
        let color = if *group == -1 {
            BLACK.mix(1.0)
        } else {
            Palette99::pick(*group as usize).mix(0.9)
        };
        Circle::new(
            (*coord_x, *coord_y),
            5,
            ShapeStyle {
                color,
                filled: true,
                stroke_width: 0.25 as u32,
            },
        )
    }))
    .unwrap();
}
