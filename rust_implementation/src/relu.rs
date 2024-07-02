use ndarray::Array2;
use crate::csv_writer::save_matrix;
use std::path::Path;

pub fn relu(matrix: &Array2<f64>, directory: String, index: i32) -> Array2<f64> {
    let mut maxx = matrix.mapv(|x| x.max(0.0));
    let filename = if index == -1 {
        format!("relu.csv")
    } else {
        format!("relu_{}.csv", index)
    };
    // save_matrix(&maxx, Path::new(&directory).join("relu").join(&filename).to_str().unwrap());
    maxx
}
