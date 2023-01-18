use crate::drawer::{draw_clustering_data, PlotType};
use crate::file_manager::{
    read_vectors_from_file, write_clustering_result_to_file, write_times_result_to_file, VectorData,
};
use crate::nbc::nbc;
use crate::neighbourhood::RowId;
use clap::Parser;
use std::collections::btree_map::BTreeMap;
use std::collections::HashSet;
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
    /// File with dataset to perform NBC on
    #[arg(short, long)]
    file_with_dataset: Option<String>,
    /// Dataset dimension. Required
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

fn parse_args(args: &Args) {
    if args.test_mode_datasets_path.is_none() && args.file_with_dataset.is_none() {
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

fn convert_vector_data(data: &Vec<VectorData>) -> Vec<&[f64]> {
    let mut vectors = Vec::new();

    for vector_data in data {
        vectors.push(&vector_data.vector[..]);
    }

    return vectors;
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
    let nbc_result = nbc(&vectors, k_value);
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

fn get_number_of_groups(clustered_data: &Vec<VectorData>, original_vectors_data: &Vec<VectorData>) {
    let original_groups_count = get_unique_values_count(original_vectors_data);
    let nbc_groups_count = get_unique_values_count(clustered_data);

    println!(
        "Original dataset has {} groups, NBC found {}",
        original_groups_count, nbc_groups_count
    );
}

fn get_unique_values_count(data: &Vec<VectorData>) -> usize {
    return data
        .into_iter()
        .map(|vector_data| vector_data.class)
        .collect::<HashSet<i32>>()
        .len();
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
            let _ = nbc(&vectors, k_value);
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

fn rand_index(clustered_data: &Vec<VectorData>, original_vectors_data: &Vec<VectorData>) -> f64 {
    if clustered_data.len() == original_vectors_data.len() {
        let mut matching_classes = 0f64;
        let mut all_checked_classes = 0f64;
        for i in 0..clustered_data.len() - 1 {
            if clustered_data[i].vector == original_vectors_data[i].vector {
                all_checked_classes += 1f64;
                if clustered_data[i].class == original_vectors_data[i].class {
                    matching_classes += 1f64;
                }
            } else {
                println!("Error between clustered data and input structure!");
            }
        }
        return matching_classes / all_checked_classes;
    } else {
        println!("Clustered data length is not the same as data from input file!");
        exit(0);
    }
}

fn merge_data(
    original_vectors_data: &Vec<VectorData>,
    clustered_data: &BTreeMap<RowId, i32>,
) -> Vec<VectorData> {
    let mut result: Vec<VectorData> = Vec::new();
    for (index, row) in original_vectors_data.iter().enumerate() {
        let class = clustered_data
            .get(&(index as i32))
            .expect("Could not get group!");
        result.push(VectorData {
            vector: row.vector.to_vec(),
            class: *class,
        });
    }

    return result;
}

#[cfg(test)]
mod tests {}
