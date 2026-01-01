use ndarray::Array2;

pub struct Matrix(Array2<f64>);

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        Self(Array2::from_shape_vec((rows, cols), data).unwrap())
    }

    pub fn dot(&self, other: &Matrix) -> Matrix {
        let result = self.0.dot(&other.0);
        Matrix(result)
    }

    pub fn transpose(&self) -> Matrix {
        Matrix(self.0.t().to_owned())
    }

    pub fn invert(&self) -> Option<Matrix> {
        // Simplified inversion (requires lapack/features usually, mocking simple cases or relying on ndarray_linalg)
        // For now, exposing stub that would use ndarray-linalg
        None
    }

    pub fn get(&self, r: usize, c: usize) -> f64 {
        self.0[[r, c]]
    }
}
