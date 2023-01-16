use crate::drawer::draw_clustered_data;
use crate::nbc::nbc;
use crate::neighbourhood::RowId;
use crate::vector_reader::read_vectors_from_file;
use ndarray::Array2;
use std::collections::btree_map::BTreeMap;
use std::time::Instant;

mod drawer;
mod nbc;
mod neighbourhood;
mod vector_reader;

fn main() {
    let dimension = 2;
    let file_name = "dataset.txt";

    let vectors = read_vectors_from_file(file_name, dimension);

    let start = Instant::now();
    let nbc_res = nbc(&vectors, 20);
    let duration = start.elapsed();
    println!("NBC algorithm for file {} took: {:?}", file_name, duration);

    let merged_data = merge_data(&vectors, &nbc_res);
    draw_clustered_data(&merged_data, file_name);
}

fn merge_data(
    vectors: &Array2<f64>,
    clustered_data: &BTreeMap<RowId, i32>,
) -> Vec<(f64, f64, i32)> {
    let mut result: Vec<(f64, f64, i32)> = Vec::new();
    for (index, row) in vectors.rows().into_iter().enumerate() {
        let group = clustered_data.get(&(index as i32)).expect("EE");
        let coord_x = *row.get(0).expect("");
        let coord_y = *row.get(1).expect("");
        result.push((coord_x, coord_y, *group));
    }

    return result;
}

#[cfg(test)]
mod tests {}
