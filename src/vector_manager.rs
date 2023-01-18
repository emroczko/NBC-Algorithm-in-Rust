use crate::exit_with_message;
use crate::file_manager::VectorData;
use crate::neighbourhood::RowId;
use std::collections::{BTreeMap, HashSet};
use std::process::exit;

pub fn convert_vector_data(data: &Vec<VectorData>) -> Vec<&[f64]> {
    let mut vectors = Vec::new();

    for vector_data in data {
        vectors.push(&vector_data.vector[..]);
    }

    return vectors;
}

pub fn get_number_of_groups(
    clustered_data: &Vec<VectorData>,
    original_vectors_data: &Vec<VectorData>,
) {
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

pub fn rand_index(
    clustered_data: &Vec<VectorData>,
    original_vectors_data: &Vec<VectorData>,
) -> f64 {
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
    }
    println!("Clustered data length is not the same as data from input file!");
    exit(0);
}

pub fn merge_data(
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
