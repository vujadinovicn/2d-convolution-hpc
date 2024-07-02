import numpy as np
import os
import multiprocessing as mp

def init_files():
    directory = f"par_visualization_np{num_processes}_i{input_matrix.shape[0]}_f{filter_matrix.shape[0]}_s{stride}_p{padding}_ps{pool_size}_pstr{pool_stride}"
    os.makedirs(directory, exist_ok=True)
    os.makedirs(os.path.join(directory, "input"), exist_ok=True)
    os.makedirs(os.path.join(directory, "submatrices"), exist_ok=True)
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

def convolution(input_matrix, filter_matrix, stride=1, padding=0, idx=-1,  directory="par_visualization"):
    input_padded = input_matrix
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
            save_matrix(output_matrix, f"{directory}/convolution/conv_{idx}_{it}.csv")
            it += 1
    return output_matrix

def relu(matrix, idx=-1, directory="par_visualization"):
    max = [[0 if element < 0 else element for element in row] for row in matrix]
    save_matrix(max, f"{directory}/relu/relu_{idx}.csv")
    return max

def max_pooling(matrix, pool_size=2, stride=2, idx=-1, directory="par_visualization"):
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
            save_matrix(output_matrix, f"{directory}/pooling/pool_{idx}_{it}.csv")
            it += 1

    return output_matrix

def divide_matrix(matrix, num_parts):
    submatrices = []
    size = len(matrix)
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
    submatrix, filter_matrix, stride, padding, pool_size, pool_stride, idx, directory = args
    save_matrix(submatrix, f"{directory}/submatrices/submatrix_{idx}.csv")

    conv_output = convolution(submatrix, filter_matrix, stride, padding, idx, directory)
    relu_output = relu(conv_output, idx, directory)
    pool_output = max_pooling(relu_output, pool_size, pool_stride, idx, directory)
    return pool_output

def parallel_processing(input_matrix, filter_matrix, num_processes, stride=1, padding=0, pool_size=2, pool_stride=2, directory="par_visualization"):
    input_matrix = np.pad(input_matrix, ((padding, padding), (padding, padding)), mode='constant', constant_values=0)
    submatrices = divide_matrix(input_matrix, int(np.sqrt(num_processes)))
    args = [(submatrix, filter_matrix, stride, padding, pool_size, pool_stride, idx, directory) for idx, submatrix in enumerate(submatrices)]

    ctx = mp.get_context('spawn')
    with ctx.Pool(num_processes) as pool:
        results = pool.map(process_submatrix, args)

    return results

if __name__ == "__main__":
    import time
    start_time= time.time()
    input_matrix = load_matrix('../data/input_matrix_8.csv')
    filter_matrix = load_matrix('../data/filter_matrix_2.csv')

    stride = 2
    padding = 0
    pool_size = 2
    pool_stride = 2
    num_processes = 4

    directory = init_files()

    results = parallel_processing(input_matrix, filter_matrix, num_processes, stride=stride, 
                                padding=padding, pool_size=pool_size, pool_stride=pool_stride, directory=directory)
    output_matrix = merge_matrices(results, int(np.sqrt(num_processes)))
    save_matrix(output_matrix, f"{directory}/output/output_matrix.csv")
    print("Output:\n", output_matrix)

