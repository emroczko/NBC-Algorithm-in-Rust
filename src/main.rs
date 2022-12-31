use crate::nbc::nbc;
use ndarray::array;

mod nbc;
mod neighbourhood;

fn main() {
    println!("NBC clustering");

    let vectors = array!([1.0, 2.0, 3.0, 4.0]);

    let k = 2;
    let _ = nbc(vectors, k);
}
