use crate::drawer::draw_clustered_data;
use crate::nbc::nbc;
use crate::neighbourhood::RowId;
use crate::vector_reader::read_vectors_from_file;
use ndarray::Array2;
use std::collections::btree_map::BTreeMap;

mod drawer;
mod nbc;
mod neighbourhood;
mod vector_reader;

fn main() {
    println!("NBC clustering");
    let dimension = 2;
    let vectors = read_vectors_from_file("dataset.txt", dimension);
    let nbc_res = nbc(&vectors, 20);
    let merged_data = merge_data(&vectors, &nbc_res);
    draw_clustered_data(&merged_data);
    println!("{:?}", nbc_res);
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
