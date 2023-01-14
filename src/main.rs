use crate::drawer::draw_clustered_data;
use crate::nbc::nbc;
use crate::neighbourhood::RowId;
use crate::vector_reader::read_vectors_from_file;
use ndarray::Array2;
use std::collections::BTreeMap;

mod nbc;
mod neighbourhood;
mod vector_reader;

fn main() {
    println!("NBC clustering");
    let dimension = 2;
    let vectors = read_vectors_from_file("dataset.txt", dimension);
    let nbc_res = nbc(vectors, 20);
    println!("{:?}", nbc_res);
}

#[cfg(test)]
mod tests {}
