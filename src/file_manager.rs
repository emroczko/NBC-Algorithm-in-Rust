use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::time::Duration;

pub struct VectorData {
    pub vector: Vec<f64>,
    pub class: i32,
}

pub fn read_vectors_from_file(file_name: &String, dimension: usize) -> Vec<VectorData> {
    let lines = read_lines(file_name).expect("Dataset file does not exist!");
    let mut vectors_data: Vec<VectorData> = Vec::new();

    println!("Parsing file {}", file_name);
    for line in lines {
        if let Ok(line) = line {
            if !line.is_empty() && !(line.starts_with("@") || line.starts_with("%")) {
                let mut vector: Vec<f64> = line
                    .split(",")
                    .into_iter()
                    .filter_map(|mut s| {
                        if s == "noise" {
                            s = "-1";
                        }
                        s.trim().parse().ok()
                    })
                    .collect();
                let class = vector.pop();
                if vector.len() == dimension {
                    vectors_data.push(VectorData {
                        vector,
                        class: class.expect("Could not resolve class!") as i32,
                    });
                } else {
                    println!("Line corrupted: {:?}", line)
                }
            }
        }
    }

    println!("Loaded {} coordinates", vectors_data.len());
    return vectors_data;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn write_clustering_result_to_file(vectors: &Vec<VectorData>, file_path: &String) {
    let mut file = File::create(file_path).expect("Couldn't create file!");
    for vector_data in vectors {
        let mut coords: String = "".to_string();
        for coord in &vector_data.vector {
            coords = format!("{},{}", coords, coord);
        }
        let line = format!(
            "{},{}\n",
            coords.strip_prefix(",").unwrap(),
            vector_data.class
        );
        file.write_all(line.as_ref()).expect("Unable to write data");
    }
    println!("Results saved to file {}", file_path);
}

pub fn write_times_result_to_file(times: &Vec<(usize, Duration, String)>, file_path: &String) {
    let mut file = File::create(file_path).expect("Couldn't create file!");
    for (dataset_size, execution_time, dataset_name) in times {
        let line = format!("{},{:?},{}\n", dataset_size, execution_time, dataset_name);
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
