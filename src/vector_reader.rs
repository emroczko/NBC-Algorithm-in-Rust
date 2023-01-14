use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn read_vectors_from_file(file_name: &str, dimension: usize) -> Array2<f64> {
    let lines = read_lines(file_name).expect("File does not exist!");

    let mut nrows = 0;
    let mut coordinates = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            if !line.is_empty() && !(line.starts_with("@") || line.starts_with("%")) {
                let coordinate: Vec<f64> = line
                    .split(",")
                    .into_iter()
                    .take(dimension)
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if coordinate.len() == dimension {
                    coordinates.extend_from_slice(&coordinate);
                    nrows += 1;
                } else {
                    println!("Line corrupted: {:?}", line)
                }
            }
        }
    }

    let coordinates_array = Array2::from_shape_vec((nrows, dimension), coordinates)
        .ok()
        .expect("Cannot create array from input data!");
    println!("Loaded {} coordinates", coordinates_array.shape()[0]);
    return coordinates_array;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::vector_reader::read_vectors_from_file;

    #[test]
    fn test_read_vectors_from_file() {
        read_vectors_from_file("dataset1.txt", 2);
    }
}
