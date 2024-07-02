import numpy as np
import os

def init_files():
    directory = f"seq_visualization_i{input_matrix.shape[0]}_f{filter_matrix.shape[0]}_s{stride}_p{padding}_ps{pool_size}_pstr{pool_stride}"
    os.makedirs(directory, exist_ok=True)
    os.makedirs(os.path.join(directory, "input"), exist_ok=True)
    os.makedirs(os.path.join(directory, "convolution"), exist_ok=True)
    os.makedirs(os.path.join(directory, "relu"), exist_ok=True)
    os.makedirs(os.path.join(directory, "pooling"), exist_ok=True)
    os.makedirs(os.path.join(directory, "output"), exist_ok=True)

    save_matrix(input_matrix, f"{directory}/input/input_matrix.csv")
    save_matrix(filter_matrix, f"{directory}/input/filter_matrix.csv")

    return directory
    
def load_matrix(filename):
    matrix = np.loadtxt(filename, delimiter=',')
    return matrix

def save_matrix(matrix, filename):
    with open(filename, "w") as f:
        np.savetxt(f, matrix, delimiter=',')

def convolution(input_matrix, filter_matrix, stride=1, padding=0, directory="seq_visualization"):
    input_padded = np.pad(input_matrix, ((padding, padding), (padding, padding)), mode='constant', constant_values=0)
    filter_size = filter_matrix.shape[0]
    output_size = ((input_padded.shape[0] - filter_size) // stride) + 1
    output_matrix = np.zeros((output_size, output_size))

    it = 0

    for i in range(0, input_padded.shape[0] - filter_size + 1, stride):
        for j in range(0, input_padded.shape[1] - filter_size + 1, stride):
            region_sum = 0
            for fi in range(len(filter_matrix)):
                for fj in range(len(filter_matrix[0])):
                    region_sum += input_padded[i + fi][j + fj] * filter_matrix[fi][fj]
            output_matrix[i // stride][j // stride] = region_sum
            save_matrix(output_matrix, f"{directory}/convolution/conv_{it}.csv")
            it += 1

    return output_matrix

def relu(matrix, directory):
    max_matrix = [[0 if element < 0 else element for element in row] for row in matrix]
    save_matrix(max_matrix, f"{directory}/relu/relu.csv")
    return max_matrix

def max_pooling(matrix, pool_size=2, stride=2, directory="seq_visualization"):
    output_size = ((len(matrix) - pool_size) // stride) + 1
    output_matrix = np.zeros((output_size, output_size))

    it = 0
    for i in range(0, len(matrix) - pool_size + 1, stride):
        for j in range(0, len(matrix) - pool_size + 1, stride):
            region = [
                matrix[i + di][j + dj]
                for di in range(pool_size)
                for dj in range(pool_size)
            ]
            output_matrix[i // stride][j // stride] = max(region)
            save_matrix(output_matrix, f"{directory}/pooling/pool_{it}.csv")
            it += 1
    return output_matrix

if __name__ == "__main__":
    import time
    start_time= time.time()
    input_matrix = load_matrix('../data/input_matrix_8.csv')
    filter_matrix = load_matrix('../data/filter_matrix_2.csv')
    mid_time = time.time()-start_time

    stride = 2
    padding = 0
    pool_size = 2
    pool_stride = 2

    directory = init_files()

    conv_output = convolution(input_matrix, filter_matrix, stride=stride, padding=padding, directory=directory)
    relu_output = relu(conv_output, directory=directory)
    pool_output = max_pooling(relu_output, pool_size=pool_size, stride=pool_stride, directory=directory)

    save_matrix(pool_output, f"{directory}/output/output_matrix.csv")
    print("Output:\n", pool_output)
