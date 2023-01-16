use crate::drawer::draw_clustered_data;
use crate::nbc::nbc;
use crate::neighbourhood::RowId;
use crate::vector_manager::{read_vectors_from_file, write_to_file};
use clap::Parser;
use std::collections::btree_map::BTreeMap;
use std::time::Instant;

mod drawer;
mod nbc;
mod neighbourhood;
mod vector_manager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_with_dataset: String,
    #[arg(short, long)]
    dataset_dimension: usize,
    #[arg(short, long, default_value_t = 10)]
    k_value: i32,
    #[arg(short, long, default_value_t = true)]
    plot_data: bool,
}

fn main() {
    let args = Args::parse();

    let dimension = args.dataset_dimension;
    let file_name = args.file_with_dataset;
    let k_value = args.k_value;

    let vectors = read_vectors_from_file(&file_name, dimension);

    println!("Starting NBC...");
    let start = Instant::now();
    let nbc_res = nbc(&vectors, k_value);
    let duration = start.elapsed();
    println!("NBC algorithm for file {} took: {:?}", file_name, duration);

    let merged_data = merge_data(&vectors, &nbc_res);

    write_to_file(&merged_data);

    if args.plot_data && dimension == 2 {
        draw_clustered_data(&merged_data, &file_name);
    }
}

fn merge_data(
    vectors: &Vec<Vec<f64>>,
    clustered_data: &BTreeMap<RowId, i32>,
) -> Vec<(f64, f64, i32)> {
    let mut result: Vec<(f64, f64, i32)> = Vec::new();
    for (index, row) in vectors.iter().enumerate() {
        let group = clustered_data.get(&(index as i32)).expect("EE");
        let coord_x = *row.get(0).expect("");
        let coord_y = *row.get(1).expect("");
        result.push((coord_x, coord_y, *group));
    }

    return result;
}

#[cfg(test)]
mod tests {}
