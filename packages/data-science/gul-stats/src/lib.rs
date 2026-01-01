pub struct Series(Vec<f64>);

impl Series {
    pub fn new(data: Vec<f64>) -> Self {
        Self(data)
    }

    pub fn mean(&self) -> f64 {
        let sum: f64 = self.0.iter().sum();
        sum / self.0.len() as f64
    }

    pub fn median(&self) -> f64 {
        let mut sorted = self.0.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }

    pub fn variance(&self) -> f64 {
        let m = self.mean();
        let sum_sq_diff: f64 = self.0.iter().map(|x| (x - m).powi(2)).sum();
        sum_sq_diff / self.0.len() as f64
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }
}
