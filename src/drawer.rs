use plotters::prelude::*;
use std::path::Path;

pub fn draw_clustered_data(data: &Vec<(Vec<f64>, i32)>, file_name: &String) {
    println!("Drawing results...");
    let path = Path::new(file_name);
    println!("{:?}", path);
    let root_area = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let plot_ranges = find_ranges(data);

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Clustering results", ("sans-serif", 40))
        .build_cartesian_2d(
            plot_ranges.min_x_coord..plot_ranges.max_x_coord,
            plot_ranges.min_y_coord..plot_ranges.max_y_coord,
        )
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(data.iter().map(|(vec, group)| {
        let color = if *group == -1 {
            BLACK.mix(1.0)
        } else {
            Palette99::pick(*group as usize).mix(0.9)
        };
        Circle::new(
            (vec[0], vec[1]),
            4,
            ShapeStyle {
                color,
                filled: true,
                stroke_width: 0.25 as u32,
            },
        )
    }))
    .unwrap();
}

struct PlotRanges {
    max_x_coord: f64,
    min_x_coord: f64,
    max_y_coord: f64,
    min_y_coord: f64,
}

fn find_ranges(data: &Vec<(Vec<f64>, i32)>) -> PlotRanges {
    let mut plot_ranges: PlotRanges = PlotRanges {
        max_x_coord: 0.0,
        min_x_coord: 0.0,
        max_y_coord: 0.0,
        min_y_coord: 0.0,
    };

    for (row, _) in data.iter() {
        plot_ranges.min_x_coord = if row[0] < plot_ranges.min_x_coord {
            row[0]
        } else {
            plot_ranges.min_x_coord
        };
        plot_ranges.max_x_coord = if row[0] > plot_ranges.max_x_coord {
            row[0]
        } else {
            plot_ranges.max_x_coord
        };
        plot_ranges.min_y_coord = if row[1] < plot_ranges.min_y_coord {
            row[1]
        } else {
            plot_ranges.min_y_coord
        };
        plot_ranges.max_y_coord = if row[1] > plot_ranges.max_y_coord {
            row[1]
        } else {
            plot_ranges.max_y_coord
        };
    }

    return plot_ranges;
}
