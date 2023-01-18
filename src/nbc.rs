use crate::neighbourhood::{ndf, neighbourhood, Ndf, RowId};
use std::collections::btree_map::BTreeMap;
use std::time::Instant;

pub fn nbc(vectors: &Vec<&[f64]>, k: i32) -> BTreeMap<RowId, i32> {
    let mut clusters: BTreeMap<RowId, i32> = BTreeMap::new();

    for (point, _) in vectors.iter().enumerate() {
        clusters.insert(point as RowId, -1); // allocate memory for results
    }

    let start = Instant::now();
    let (knb, r_knb) = neighbourhood(vectors, k);
    let duration = start.elapsed();
    println!("Exploring neighbourhood took: {:?}", duration);

    let ndf = ndf(&knb, &r_knb);
    let mut current_cluster_id = 0;

    for (row_id, _) in vectors.iter().enumerate() {
        if has_cluster(row_id as RowId, &clusters) || !is_dense_point(&(row_id as RowId), &ndf) {
            continue;
        }

        clusters.insert(row_id as RowId, current_cluster_id);

        let mut dense_points = Vec::new();

        for neighbour in knb.get(&(row_id as RowId)).expect("") {
            clusters.insert(*neighbour, current_cluster_id);
            if is_dense_point(neighbour, &ndf) {
                dense_points.push(neighbour);
            }
        }

        while !dense_points.is_empty() {
            let dp = dense_points.pop().expect("CC");

            for neighbour in knb.get(dp).expect("DD") {
                if has_cluster(*neighbour as RowId, &clusters) {
                    continue;
                }
                clusters.insert(*neighbour as RowId, current_cluster_id);
                if is_dense_point(neighbour, &ndf) {
                    dense_points.push(neighbour);
                }
            }
        }

        current_cluster_id += 1;
    }

    return clusters;
}

fn has_cluster(row_id: RowId, clusters: &BTreeMap<RowId, i32>) -> bool {
    return clusters.get(&row_id).expect("AA") != &-1;
}

fn is_dense_point(row_id: &RowId, ndf: &Ndf) -> bool {
    return ndf.get(row_id).expect("BB") >= &(1 as f64);
}
