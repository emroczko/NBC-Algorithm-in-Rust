use crate::file_manager::VectorData;
use plotters::prelude::*;
use std::path::Path;

pub enum PlotType {
    OriginalDataset,
    NbcResult,
}

impl PlotType {
    fn as_str(&self) -> &'static str {
        match self {
            PlotType::OriginalDataset => "Original clustered dataset",
            PlotType::NbcResult => "NBC results",
        }
    }
}

pub fn draw_clustering_data(data: &Vec<VectorData>, file_name: &String, plot_type: PlotType) {
    let path = Path::new(file_name);
    println!("Drawing results to {:?}", path);
    let root_area = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let plot_ranges = find_ranges(data);

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(plot_type.as_str(), ("sans-serif", 40))
        .build_cartesian_2d(
            plot_ranges.min_x_coord..plot_ranges.max_x_coord,
            plot_ranges.min_y_coord..plot_ranges.max_y_coord,
        )
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(data.iter().map(|vector_data| {
        let color = if vector_data.class == -1 {
            BLACK.mix(1.0)
        } else {
            Palette99::pick(vector_data.class as usize).mix(0.9)
        };
        Circle::new(
            (vector_data.vector[0], vector_data.vector[1]),
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

fn find_ranges(data: &Vec<VectorData>) -> PlotRanges {
    let mut plot_ranges: PlotRanges = PlotRanges {
        max_x_coord: 0.0,
        min_x_coord: 0.0,
        max_y_coord: 0.0,
        min_y_coord: 0.0,
    };

    for vector_data in data.iter() {
        plot_ranges.min_x_coord = if vector_data.vector[0] < plot_ranges.min_x_coord {
            vector_data.vector[0]
        } else {
            plot_ranges.min_x_coord
        };
        plot_ranges.max_x_coord = if vector_data.vector[0] > plot_ranges.max_x_coord {
            vector_data.vector[0]
        } else {
            plot_ranges.max_x_coord
        };
        plot_ranges.min_y_coord = if vector_data.vector[1] < plot_ranges.min_y_coord {
            vector_data.vector[1]
        } else {
            plot_ranges.min_y_coord
        };
        plot_ranges.max_y_coord = if vector_data.vector[1] > plot_ranges.max_y_coord {
            vector_data.vector[1]
        } else {
            plot_ranges.max_y_coord
        };
    }

    return plot_ranges;
}
