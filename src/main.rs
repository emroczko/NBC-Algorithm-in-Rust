use crate::drawer::{draw_clustering_data, PlotType};
use crate::file_manager::{
    read_vectors_from_file, write_clustering_result_to_file, write_times_result_to_file,
};
use crate::nbc::neighbourhood_based_clustering;
use crate::vector_manager::{convert_vector_data, get_number_of_groups, merge_data, rand_index};
use clap::Parser;
use std::fs;
use std::process::exit;
use std::time::{Duration, Instant};

mod drawer;
mod file_manager;
mod nbc;
mod neighbourhood;
mod vector_manager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File with dataset to perform NBC on
    #[arg(short, long)]
    file_with_dataset: Option<String>,
    /// Dataset dimension
    #[arg(short, long)]
    dataset_dimension: usize,
    /// K value - to create k-neighbourhoods
    #[arg(short, long, default_value_t = 10)]
    k_value: i32,
    /// Path to plot file to create. If omitted no plot will be created
    #[arg(short, long)]
    plot_data: Option<String>,
    /// Path to result file with created clusters. If omitted app will not return result file.
    #[arg(short, long)]
    out_file: Option<String>,
    /// If passed, application runs in test mode and it runs every file in this directory. Result file with times is only created.
    #[arg(short, long)]
    test_mode_datasets_path: Option<String>,
}

pub fn exit_with_message(message: &str) {
    println!("{}", message);
    exit(1)
}

fn parse_args(args: &Args) {
    if args.test_mode_datasets_path.is_none() && args.file_with_dataset.is_none() {
        exit_with_message("You must pass either dataset file or test datasets path!");
    }

    if args.k_value <= 0 {
        exit_with_message("K value must be bigger than 0!");
    }

    if args.dataset_dimension <= 0 {
        exit_with_message("Dataset dimension must be bigger than 0!");
    }
}

fn perform_normal_nbc(args: &Args) {
    let dimension = args.dataset_dimension;
    let file_name = &args.file_with_dataset.as_ref().unwrap();
    let k_value = args.k_value;

    let vectors_data = read_vectors_from_file(&file_name, dimension);

    if vectors_data.len() < k_value as usize {
        println!("Number of coordinates must not be less than k value!");
        return;
    }

    println!("Starting NBC...");
    let vectors = convert_vector_data(&vectors_data);
    let start = Instant::now();
    let nbc_result = neighbourhood_based_clustering(&vectors, k_value);
    let duration = start.elapsed();
    println!("NBC algorithm for file {} took: {:?}", file_name, duration);

    let clustered_data = merge_data(&vectors_data, &nbc_result);

    get_number_of_groups(&clustered_data, &vectors_data);

    let rand_index = rand_index(&clustered_data, &vectors_data);
    println!("Rand index is {}", rand_index);

    if args.out_file.is_some() {
        write_clustering_result_to_file(&clustered_data, &args.out_file.as_ref().unwrap());
    }

    if args.plot_data.is_some() && dimension == 2 {
        let original_clustered_data_plot_file_name =
            format!("original_{}", args.plot_data.as_ref().unwrap());
        draw_clustering_data(
            &vectors_data,
            &original_clustered_data_plot_file_name,
            PlotType::OriginalDataset,
        );
        draw_clustering_data(
            &clustered_data,
            args.plot_data.as_ref().unwrap(),
            PlotType::NbcResult,
        );
    }
}

fn perform_nbc_tests(args: &Args) {
    let files = fs::read_dir(&args.test_mode_datasets_path.as_ref().unwrap()).unwrap();

    let dimension = args.dataset_dimension;
    let k_value = args.k_value;

    let mut time_results: Vec<(usize, Duration, String)> = Vec::new();

    for path in files {
        let existing_path = path.ok();

        if let Some(file) = existing_path {
            let file_name = &file.path().to_string_lossy().to_string();

            if file_name.contains("DS_Store") {
                continue;
            }

            let vectors_data = read_vectors_from_file(file_name, dimension);

            if vectors_data.len() < k_value as usize {
                println!("Number of coordinates must not be less than k value!");
                continue;
            }

            println!("Starting NBC...");
            let vectors = convert_vector_data(&vectors_data);
            let start = Instant::now();
            let _ = neighbourhood_based_clustering(&vectors, k_value);
            let duration = start.elapsed();
            time_results.push((vectors_data.len(), duration, file_name.to_string()));
            println!("NBC algorithm for file {} took: {:?}", file_name, duration);
        }
    }

    write_times_result_to_file(&time_results, &"times_results.csv".to_string());
}

fn main() {
    let args = Args::parse();
    parse_args(&args);

    if args.test_mode_datasets_path.is_none() {
        perform_normal_nbc(&args);
    } else {
        perform_nbc_tests(&args);
    }
}
