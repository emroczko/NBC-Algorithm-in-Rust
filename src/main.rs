use crate::drawer::draw_clustered_data;
use crate::file_manager::{read_vectors_from_file, write_clustering_result_to_file};
use crate::nbc::nbc;
use crate::neighbourhood::RowId;
use clap::Parser;
use std::collections::btree_map::BTreeMap;
use std::fs;
use std::process::exit;
use std::time::{Duration, Instant};

mod drawer;
mod file_manager;
mod nbc;
mod neighbourhood;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    file_with_dataset: String,
    #[arg(short, long)]
    dataset_dimension: usize,
    #[arg(short, long, default_value_t = 10)]
    k_value: i32,
    #[arg(short, long, default_value = "")]
    plot_data: String,
    #[arg(short, long, default_value = "")]
    out_file: String,
    #[arg(short, long)]
    test_mode_datasets_path: String,
}

fn parse_args(args: &Args) {
    if args.test_mode_datasets_path == "" && args.file_with_dataset == "" {
        println!("You must pass either dataset file or test datasets path!");
        exit(1)
    }

    if args.k_value <= 0 {
        println!("K value must be bigger than 0!");
        exit(1)
    }

    if args.dataset_dimension <= 0 {
        println!("Dataset dimension must be bigger than 0!");
        exit(1)
    }
}

fn perform_normal_nbc(args: &Args) {
    let dimension = args.dataset_dimension;
    let file_name = &args.file_with_dataset;
    let k_value = args.k_value;

    let vectors = read_vectors_from_file(&file_name, dimension);

    if vectors.len() < k_value as usize {
        println!("Number of coordinates must not be less than k value!");
        return;
    }

    println!("Starting NBC...");
    let start = Instant::now();
    let nbc_res = nbc(&vectors, k_value);
    let duration = start.elapsed();
    println!("NBC algorithm for file {} took: {:?}", file_name, duration);

    let merged_data = merge_data(&vectors, &nbc_res);

    if args.out_file != "" {
        write_clustering_result_to_file(&merged_data, &args.out_file);
    }

    if args.plot_data != "" && dimension == 2 {
        draw_clustered_data(&merged_data, &args.plot_data);
    }
}

fn perform_nbc_tests(args: &Args) {
    let files = fs::read_dir(&args.test_mode_datasets_path).unwrap();

    let dimension = args.dataset_dimension;
    let k_value = args.k_value;

    let mut time_results: Vec<(usize, Duration)> = Vec::new();

    for path in files {
        let existing_path = path.ok();

        if let Some(file) = existing_path {
            let file_name = &file.file_name().to_string_lossy().into_owned();
            let vectors = read_vectors_from_file(file_name, dimension);

            if vectors.len() < k_value as usize {
                println!("Number of coordinates must not be less than k value!");
                continue;
            }

            println!("Starting NBC...");
            let start = Instant::now();
            let _ = nbc(&vectors, k_value);
            let duration = start.elapsed();
            time_results.push((vectors.len(), duration));
            println!("NBC algorithm for file {} took: {:?}", file_name, duration);
        }
    }
}

fn main() {
    let args = Args::parse();
    parse_args(&args);

    if args.test_mode_datasets_path == "" {
        perform_normal_nbc(&args);
    } else {
        perform_nbc_tests(&args);
    }
}

fn merge_data(
    vectors: &Vec<Vec<f64>>,
    clustered_data: &BTreeMap<RowId, i32>,
) -> Vec<(Vec<f64>, i32)> {
    let mut result: Vec<(Vec<f64>, i32)> = Vec::new();
    for (index, row) in vectors.iter().enumerate() {
        let group = clustered_data
            .get(&(index as i32))
            .expect("Could not get group!");
        result.push((row.to_vec(), *group));
    }

    return result;
}

#[cfg(test)]
mod tests {}
