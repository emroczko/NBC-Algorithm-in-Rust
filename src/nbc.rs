use std::collections::HashMap;

fn nbc(dataset: Vec<String>, _k: i32) {
    let mut clusters = HashMap::new();

    for point in dataset {
        clusters.insert(point, -1);
    }
}
