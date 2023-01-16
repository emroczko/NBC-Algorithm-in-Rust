use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

pub fn read_vectors_from_file(file_name: &String, dimension: usize) -> Vec<Vec<f64>> {
    let lines = read_lines(file_name).expect("File does not exist!");
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

pub fn write_to_file(vectors: &Vec<(f64, f64, i32)>) {
    let mut file = File::create("foo.txt").expect("Couldn't create file!");
    for tuple in vectors {
        let line = format!("{:?}\n", tuple);
        file.write_all(line.as_ref()).expect("Unable to write data");
    }
    println!("Results saved to file {}", "foo");
}

#[cfg(test)]
mod tests {
    use crate::vector_manager::read_vectors_from_file;

    #[test]
    fn test_read_vectors_from_file() {
        read_vectors_from_file(&"dataset1.txt".to_string(), 2);
    }
}
