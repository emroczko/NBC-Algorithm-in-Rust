use crate::neighbourhood::{ndf, neighbourhood, Ndf, RowId};
use ndarray::{Array, Dimension};
use std::collections::HashMap;

pub fn nbc<T: Dimension>(vectors: Array<f64, T>, k: i32) -> HashMap<RowId, i32> {
    let mut clusters: HashMap<RowId, i32> = HashMap::new();

    for (point, _) in vectors.rows().into_iter().enumerate() {
        clusters.insert(point as RowId, -1);
    }

    let (knb, r_knb) = neighbourhood(&vectors, k);
    let ndf = ndf(&knb, &r_knb);
    let mut current_cluster_id = 0;

    for (idx, _) in vectors.rows().into_iter().enumerate() {
        if has_cluster(idx as RowId, &clusters) || is_dense_point(idx as RowId, &ndf) {
            continue;
        }

        clusters.insert(idx as i32, current_cluster_id);

        let mut dense_points = Vec::new();

        for n_idx in knb.get(&(idx as RowId)).expect("") {
            clusters.insert(*n_idx, current_cluster_id);
            if is_dense_point(*n_idx as RowId, &ndf) {
                dense_points.push(n_idx);
            }
        }

        while !dense_points.is_empty() {
            let dp = dense_points.pop().expect("CC");

            for n_idx in knb.get(dp).expect("DD") {
                if has_cluster(*n_idx as RowId, &clusters) {
                    continue;
                }
                clusters.insert(*n_idx as RowId, current_cluster_id);
                if is_dense_point(*n_idx, &ndf) {
                    dense_points.push(n_idx);
                }
            }
        }

        current_cluster_id += 1;
    }

    return clusters;
}

fn has_cluster(idx: RowId, clusters: &HashMap<RowId, i32>) -> bool {
    return *clusters.get(&idx).expect("AA") >= 1;
}

fn is_dense_point(idx: RowId, ndf: &Ndf) -> bool {
    return *ndf.get(&idx).expect("BB") != -1 as f64;
}

#[cfg(test)]
mod tests {}
