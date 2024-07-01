use ndarray::Array2;
use ndarray::s;
use std::path::Path;
use crate::csv_writer::save_matrix;

pub fn convolution(input_matrix: &Array2<f64>, filter_matrix: &Array2<f64>, stride: usize, padding: usize, is_seq: bool, directory: String, index: i32) -> Array2<f64> {
    let filter_size = filter_matrix.shape()[0];
    let padded_size = input_matrix.shape()[0] + 2 * padding;
    let output_size = (padded_size - filter_size) / stride + 1;

    let input_padded = if is_seq {
        let mut input_padded = Array2::zeros((padded_size, padded_size));
        input_padded.slice_mut(s![padding..padded_size-padding, padding..padded_size-padding])
            .assign(input_matrix);
        input_padded
    } else {
        input_matrix.clone()
    };

    let mut output_matrix = Array2::zeros((output_size, output_size));
    let mut k = 0;
    for i in 0..output_size {
        for j in 0..output_size {
            let region = input_padded.slice(s![
                i*stride..i*stride+filter_size,
                j*stride..j*stride+filter_size
            ]).to_owned();
            output_matrix[(i, j)] = (region * filter_matrix).sum();
            let filename = if index == -1 {
                format!("{}/convolution/conv_{}.csv", directory, k)
            } else {
                format!("{}/convolution/conv_{}_{}.csv", directory, index, k)
            };
            save_matrix(&output_matrix, &filename);
            // save_matrix(&output_matrix, Path::new(&directory).join("convolution").join(&filename).to_str().unwrap());
            k += 1;
        }
    }

    output_matrix
}
