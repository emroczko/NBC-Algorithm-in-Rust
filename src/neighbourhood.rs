use ndarray::prelude::*;
use ndarray_stats::DeviationExt;
use std::collections::{HashMap, HashSet};

type RowId = i32;
type Knb = HashMap<RowId, HashSet<i32>>;
type Ndf = HashMap<RowId, f64>;
type Rknb = HashMap<RowId, HashSet<i32>>;

fn neighbourhood<T: Dimension>(vectors: Array<f64, T>, k: usize) -> (Knb, Rknb) {
    let (mut knb, mut r_knb) = init(&vectors); // init knb and r_knb dicts

    for (row_index_1, row_1) in vectors.rows().into_iter().enumerate() {
        let mut neighbour_candidates: Vec<(RowId, f64)> = Vec::new();

        for (row_index_2, row_2) in vectors.rows().into_iter().enumerate() {
            if row_index_1 != row_index_2 {
                let dist = row_1.l2_dist(&row_2);
                neighbour_candidates.push((row_index_2 as RowId, dist.ok().expect("Error")));
            }
        }

        neighbour_candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let eps = neighbour_candidates[0..k].last().expect("Set is empty").1;

        let mut neighbours: HashSet<RowId> = HashSet::new();

        for (row_id, distance) in neighbour_candidates {
            if distance > eps {
                break;
            }
            neighbours.insert(row_id);
        }

        for neighbour in &neighbours {
            let _ = r_knb
                .get_mut(&neighbour)
                .expect("Err")
                .insert(row_index_1 as i32);
        }
        knb.insert(row_index_1 as RowId, neighbours);
    }
    return (knb, r_knb);
}

fn init<T: Dimension>(vectors: &Array<f64, T>) -> (Knb, Rknb) {
    let knb = HashMap::new();
    let mut r_knb = HashMap::new();

    for i in 0..*vectors
        .shape()
        .get(1)
        .expect("Input vector dimension error!")
    {
        r_knb.insert(i as RowId, HashSet::new());
    }
    return (knb, r_knb);
}

#[cfg(test)]
mod tests {
    use crate::neighbourhood::{ndf, neighbourhood, Knb, Rknb};
    use ndarray::array;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_neighbours() {
        let k = 2 as usize;
        let vectors = array!([
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0, 1.0],
            [2.0, 2.0, 1.0, 1.0],
            [3.0, 4.0, 1.0, 1.0],
            [3.0, 4.0, 1.0, 1.0],
        ]);

        let expected_knb: Knb = HashMap::from([
            (0, HashSet::from_iter([1, 2])),
            (1, HashSet::from_iter([0, 2])),
            (2, HashSet::from_iter([1, 3, 4])),
            (3, HashSet::from_iter([2, 4])),
            (4, HashSet::from_iter([2, 3])),
        ]);

        let expected_r_knb: Rknb = HashMap::from([
            (0, HashSet::from_iter([1])),
            (1, HashSet::from_iter([0, 2])),
            (2, HashSet::from_iter([0, 1, 3, 4])),
            (3, HashSet::from_iter([2, 4])),
            (4, HashSet::from_iter([2, 3])),
        ]);

        let (knb, r_knb) = neighbourhood(vectors, k);

        assert_eq!(expected_knb, knb);
        assert_eq!(expected_r_knb, r_knb);
    }

        println!("{:?}", expected_knb);

        let (knb, r_knb) = neighbourhood(vectors, k);
        println!("{:?}", knb);
        println!("{:?}", r_knb);
    }
}
