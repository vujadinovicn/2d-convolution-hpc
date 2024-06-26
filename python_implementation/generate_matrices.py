import numpy as np

def generate_random_matrix(shape, filename):
    matrix = np.random.rand(*shape)
    np.savetxt(filename, matrix, delimiter=',')
    # with h5py.File(filename, 'w') as f:
    #     f.create_dataset('data', data=matrix)
    return matrix

if __name__ == "__main__":
    input_shape = (1536, 1536)
    filter_shape = (2, 2)

    input_matrix = generate_random_matrix(input_shape, 'input_matrix_1536.csv')
    # filter_matrix = generate_random_matrix(filter_shape, 'filter_matrix_2.csv')
