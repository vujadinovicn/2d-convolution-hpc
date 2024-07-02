use ndarray::Array2;
use crate::convolution::convolution;
use crate::relu::relu;
use crate::max_pooling::max_pooling;
use rayon::prelude::*;
use ndarray::s;
use rayon::iter::IntoParallelIterator;
use num_integer::Roots;
use crate::csv_writer::save_matrix;

pub fn divide_matrix(matrix: &Array2<f64>, num_parts: usize) -> Vec<Array2<f64>> {
    let step = matrix.shape()[0] / num_parts;
    let mut submatrices = Vec::new();

    for i in 0..num_parts {
        for j in 0..num_parts {
            let submatrix = matrix.slice(s![
                i * step..(i + 1) * step,
                j * step..(j + 1) * step
            ]).to_owned();
            submatrices.push(submatrix.clone());
        }
    }

    submatrices
}

pub fn merge_matrices(submatrices: Vec<Array2<f64>>, num_parts: usize) -> Array2<f64> {
    let step = submatrices[0].shape()[0];
    let merged_size = step * num_parts;
    let mut merged_matrix = Array2::zeros((merged_size, merged_size));
    let mut index = 0;

    for i in 0..num_parts {
        for j in 0..num_parts {
            merged_matrix.slice_mut(s![
                i * step..(i + 1) * step,
                j * step..(j + 1) * step
            ]).assign(&submatrices[index]);
            index += 1;
        }
    }

    merged_matrix
}

pub fn process_submatrix(submatrix: Array2<f64>, filter_matrix: &Array2<f64>, stride: usize, padding: usize, pool_size: usize, pool_stride: usize, directory: String, index: i32) -> Array2<f64> {
    let filename = format!("{}/submatrices/submatrix_{}.csv", directory, index);
    // save_matrix(&submatrix, &filename);
    let conv_output = convolution(&submatrix, filter_matrix, stride, padding, false,  directory.clone(), index);
    let relu_output = relu(&conv_output,  directory.clone(), index);
    let pool_output = max_pooling(&relu_output, pool_size, pool_stride, directory.clone(), index);
    pool_output
}

pub fn parallel_processing(input_matrix: &Array2<f64>, filter_matrix: &Array2<f64>, num_parts: usize, stride: usize, padding: usize, pool_size: usize, pool_stride: usize, directory: String) -> Array2<f64> {
    let padded_size = input_matrix.shape()[0] + 2 * padding;
    let filter_size = filter_matrix.shape()[0];
    let output_size = (padded_size - filter_size) / stride + 1;

    let mut input_padded = Array2::zeros((padded_size, padded_size));
    input_padded.slice_mut(s![padding..padded_size-padding, padding..padded_size-padding])
        .assign(input_matrix);
        
    let submatrices = divide_matrix(&input_padded, (num_parts).nth_root(2));
    let results: Vec<Array2<f64>> = submatrices.into_par_iter()
        .enumerate()  // Add this to get the index
        .map(|(index, submatrix)| process_submatrix(submatrix, filter_matrix, stride, padding, pool_size, pool_stride, directory.clone(), index.try_into().unwrap()))  // Pass the index to the function
        .collect();

    merge_matrices(results, (num_parts).nth_root(2))
}
