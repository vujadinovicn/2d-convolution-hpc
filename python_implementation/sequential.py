import numpy as np

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

if __name__ == "__main__":
    input_matrix = load_matrix('../data/input_matrix_1800.csv')
    filter_matrix = load_matrix('../data/filter_matrix_5.csv')

    conv_output = convolution(input_matrix, filter_matrix, stride=1, padding=1)
    print("Convolution Output:\n", conv_output)
    relu_output = relu(conv_output)
    print("ReLu Output:\n", relu_output)
    pool_output = max_pooling(relu_output, pool_size=2, stride=2)
    print("Max Pooling Output:\n", pool_output)


