use ndarray::Array2;
use crate::convolution::convolution;
use crate::relu::relu;
use crate::max_pooling::max_pooling;

pub fn sequential_processing(input_matrix: &Array2<f64>, filter_matrix: &Array2<f64>, stride: usize, padding: usize, pool_size: usize, pool_stride: usize, directory: String) -> Array2<f64> {
    let conv_output = convolution(input_matrix, filter_matrix, stride, padding, true, directory.clone(), -1);
    let relu_output = relu(&conv_output, directory.clone(), -1);
    let pool_output = max_pooling(&relu_output, pool_size, pool_stride, directory.clone(), -1);
    pool_output
}
