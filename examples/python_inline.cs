extern python {
    fn matrix_multiply(a: list, b: list) -> list {
        import numpy as np
        return np.dot(a, b).tolist()
    }

    fn analyze_data(data: list) -> dict {
        import numpy as np
        arr = np.array(data)
        return {
            'mean': float(np.mean(arr)),
            'std': float(np.std(arr)),
            'min': float(np.min(arr)),
            'max': float(np.max(arr))
        }
    }
}

main():
    matrix_a = [[1, 2], [3, 4]]
    matrix_b = [[5, 6], [7, 8]]
    result = matrix_multiply(matrix_a, matrix_b)
    print("Matrix product:", result)

    data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    stats = analyze_data(data)
    print("Statistics:", stats)
