use ndarray::Array2;
use ndarray::s;
use crate::csv_writer::save_matrix;
use std::path::{Path, PathBuf};

pub fn max_pooling(matrix: &Array2<f64>, pool_size: usize, stride: usize, directory: String, index: i32) -> Array2<f64> {
    let input_size = matrix.shape()[0];
    let output_size = ((input_size - pool_size) / stride) + 1;
    let mut output_matrix = Array2::zeros((output_size, output_size));

    let mut k = 0;
    for i in 0..output_size {
        for j in 0..output_size {
            let start_i = i * stride;
            let end_i = start_i + pool_size;
            let start_j = j * stride;
            let end_j = start_j + pool_size;

            if end_i <= input_size && end_j <= input_size {
                let region = matrix.slice(s![start_i..end_i, start_j..end_j]);
                output_matrix[(i, j)] = region.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let filename = if index == -1 {
                    format!("{}/pooling/pool_{}.csv", directory, k)
                } else {
                    format!("{}/pooling/pool_{}_{}.csv", directory, index, k)
                };
                save_matrix(&output_matrix, &filename);
                k += 1;
            }
        }
    }

    output_matrix
}
