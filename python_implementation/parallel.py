import numpy as np
import h5py
import multiprocessing as mp

def load_matrix(filename):
    matrix = np.loadtxt(filename, delimiter=',')
    return matrix

def convolution(input_matrix, filter_matrix, stride=1, padding=0):
    input_padded = np.pad(input_matrix, ((padding, padding), (padding, padding)), mode='constant', constant_values=0)
    filter_size = filter_matrix.shape[0]
    output_size = ((input_padded.shape[0] - filter_size) // stride) + 1
    output_matrix = np.zeros((output_size, output_size))

    for i in range(0, input_padded.shape[0] - filter_size + 1, stride):
        for j in range(0, input_padded.shape[1] - filter_size + 1, stride):
            region = input_padded[i:i + filter_size, j:j + filter_size]
            output_matrix[i // stride, j // stride] = np.sum(region * filter_matrix)

    return output_matrix

def relu(matrix):
    return np.maximum(0, matrix)

def max_pooling(matrix, pool_size=2, stride=2):
    output_size = ((matrix.shape[0] - pool_size) // stride) + 1
    output_matrix = np.zeros((output_size, output_size))

    for i in range(0, matrix.shape[0] - pool_size + 1, stride):
        for j in range(0, matrix.shape[1] - pool_size + 1, stride):
            region = matrix[i:i + pool_size, j:j + pool_size]
            output_matrix[i // stride, j // stride] = np.max(region)

    return output_matrix

def divide_matrix(matrix, num_parts):
    submatrices = []
    size = matrix.shape[0]
    step = size // num_parts
    for i in range(0, size, step):
        for j in range(0, size, step):
            submatrix = matrix[i:i + step, j:j + step]
            submatrices.append(submatrix)
    return submatrices

def merge_matrices(submatrices, num_parts):
    step = submatrices[0].shape[0]
    merged_size = step * num_parts
    merged_matrix = np.zeros((merged_size, merged_size))
    index = 0
    for i in range(0, merged_size, step):
        for j in range(0, merged_size, step):
            merged_matrix[i:i + step, j:j + step] = submatrices[index]
            index += 1
    return merged_matrix

def process_submatrix(args):
    submatrix, filter_matrix, stride, padding, pool_size, pool_stride = args
    conv_output = convolution(submatrix, filter_matrix, stride, padding)
    relu_output = relu(conv_output)
    pool_output = max_pooling(relu_output, pool_size, pool_stride)
    return pool_output

def parallel_processing(input_matrix, filter_matrix, num_processes, stride=1, padding=0, pool_size=2, pool_stride=2):
    submatrices = divide_matrix(input_matrix, int(np.sqrt(num_processes)))
    args = [(submatrix, filter_matrix, stride, padding, pool_size, pool_stride) for submatrix in submatrices]

    ctx = mp.get_context('spawn')
    with ctx.Pool(num_processes) as pool:
        results = pool.map(process_submatrix, args)

    return results

if __name__ == "__main__":
    input_matrix = load_matrix('../data/input_matrix_1800.csv')
    filter_matrix = load_matrix('../data/filter_matrix_5.csv')

    num_processes = 25
    results = parallel_processing(input_matrix, filter_matrix, num_processes, stride=1, padding=1, pool_size=2, pool_stride=2)
    output_matrix = merge_matrices(results, int(np.sqrt(num_processes)))
    print("Output Matrix:\n", output_matrix)

