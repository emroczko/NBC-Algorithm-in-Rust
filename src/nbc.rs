use std::collections::{HashMap, HashSet};
use std::env::var;
use std::hash::Hash;
use std::ptr::null;
use ndarray::prelude::*;

fn nbc(dataset: Vec<String>, k: i32) {

    let mut clusters = HashMap::new();

    for point in dataset {
        clusters.insert(point, -1);
    }
}
