use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

pub fn read_vectors_from_file(file_name: &String, dimension: usize) -> Vec<Vec<f64>> {
    let lines = read_lines(file_name).expect("File does not exist!");
    let mut coordinates = Vec::new();

    println!("Parsing file {}", file_name);
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
                    coordinates.push(coordinate);
                } else {
                    println!("Line corrupted: {:?}", line)
                }
            }
        }
    }

    println!("Loaded {} coordinates", coordinates.len());
    return coordinates;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn write_clustering_result_to_file(vectors: &Vec<(Vec<f64>, i32)>, file_path: &String) {
    let mut file = File::create(file_path).expect("Couldn't create file!");
    for (vector, cluster) in vectors {
        let mut coords: String = "".to_string();
        for coord in vector {
            coords = format!("{},{}", coords, coord);
        }
        let line = format!("{},{}\n", coords.strip_prefix(",").unwrap(), cluster);
        file.write_all(line.as_ref()).expect("Unable to write data");
    }
    println!("Results saved to file {}", file_path);
}

#[cfg(test)]
mod tests {
    use crate::file_manager::read_vectors_from_file;

    #[test]
    fn test_read_vectors_from_file() {
        read_vectors_from_file(&"dataset1.txt".to_string(), 2);
    }
}
