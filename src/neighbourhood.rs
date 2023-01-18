use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub type RowId = i32;
pub type Knb = HashMap<RowId, Vec<i32>>;
pub type Ndf = HashMap<RowId, f64>;
pub type Rknb = HashMap<RowId, HashSet<i32>>;

fn calculate_euclidean_distance(v1: &[f64], v2: &[f64]) -> f64 {
    let mut distance = 0.0;
    for i in 0..v1.len() {
        distance += (v1[i] - v2[i]).powf(2.0);
    }
    return distance.sqrt();
}

pub fn calculate_neighbourhood(vectors: &Vec<&[f64]>, k: i32) -> (Knb, Rknb) {
    let start = Instant::now();
    let (mut knb, mut r_knb) = init_neighbourhood(vectors.len()); // allocate knb and r_knb dicts
    let duration = start.elapsed();
    println!("Init neighbourhood took: {:?}", duration);

    for (row_index_1, row_1) in vectors.iter().enumerate() {
        let mut neighbour_candidates: Vec<(RowId, f64)> = Vec::new();

        for (row_index_2, row_2) in vectors.iter().enumerate() {
            if row_index_1 != row_index_2 {
                // for each different object count euclidean distance and add it to neighbour candidates
                let dist = calculate_euclidean_distance(&row_1, &row_2);
                neighbour_candidates.push((row_index_2 as RowId, dist));
            }
        }

        // sorting candidates by distance ascending
        neighbour_candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // eps is distance of k-th candidate in sorted candidates
        let furthest_neighbour_distance = neighbour_candidates[(k - 1) as usize].1;

        let mut neighbours: Vec<RowId> = Vec::new();

        // if distance is smaller than or equal to eps then it is a nearest neighbour. There may be more neighbours than k value
        for (row_id, distance) in neighbour_candidates {
            if distance > furthest_neighbour_distance {
                break; // breaking so for is stopping earlier
            }
            neighbours.push(row_id);
        }

        for neighbour in &neighbours {
            let _ = r_knb
                .entry(*neighbour)
                .or_insert(HashSet::new())
                .insert(row_index_1 as i32);
        }
        knb.insert(row_index_1 as RowId, neighbours);
    }
    return (knb, r_knb);
}

pub fn calculate_ndf(knb: &Knb, r_knb: &Rknb) -> Ndf {
    let mut ndf = HashMap::new();

    for k in knb.keys() {
        let k_objects = knb.get(k).expect("A").len() as f64;
        let r_objects = r_knb.get(k).expect("R").len() as f64;
        let _ = ndf.insert(*k, r_objects / k_objects);
    }
    return ndf;
}

fn init_neighbourhood(vectors_number: usize) -> (Knb, Rknb) {
    let knb = HashMap::new();
    let mut r_knb = HashMap::new();

    for i in 0..vectors_number {
        r_knb.insert(i as RowId, HashSet::new());
    }
    return (knb, r_knb);
}

#[cfg(test)]
mod tests {
    use crate::neighbourhood::{calculate_ndf, calculate_neighbourhood, Knb, Ndf, Rknb};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_neighbours() {
        let k = 2;

        let vectors: Vec<&[f64]> = vec![
            &[0.0, 0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0, 1.0],
            &[2.0, 2.0, 1.0, 1.0],
            &[3.0, 4.0, 1.0, 1.0],
            &[3.0, 4.0, 1.0, 1.0],
        ];
        //let expected_r_knb: Rknb = HashMap::from([(0, 1), (1, 2), (2, 4), (3, 2), (4, 2)]);
        let expected_knb: Knb = HashMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2]),
            (2, vec![1, 3, 4]),
            (3, vec![2, 4]),
            (4, vec![2, 3]),
        ]);

        let expected_r_knb: Rknb = HashMap::from([
            (0, HashSet::from_iter([1])),
            (1, HashSet::from_iter([0, 2])),
            (2, HashSet::from_iter([0, 1, 3, 4])),
            (3, HashSet::from_iter([2, 4])),
            (4, HashSet::from_iter([2, 3])),
        ]);

        let (knb, r_knb) = calculate_neighbourhood(&vectors, k);

        assert_eq!(expected_knb, knb);
        assert_eq!(expected_r_knb, r_knb);
    }

    #[test]
    fn test_ndf() {
        let k = 2;
        let vectors: Vec<&[f64]> = vec![
            &[0.0, 0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0, 1.0],
            &[2.0, 2.0, 1.0, 1.0],
            &[3.0, 4.0, 1.0, 1.0],
            &[3.0, 4.0, 1.0, 1.0],
        ];

        let expected_ndf: Ndf = HashMap::from([
            (0, 0.5),
            (1, 1.0),
            (2, 1.3333333333333333),
            (3, 1.0),
            (4, 1.0),
        ]);

        let (knb, r_knb) = calculate_neighbourhood(&vectors, k);
        let ndf = calculate_ndf(&knb, &r_knb);

        assert_eq!(expected_ndf, ndf);
    }
}
