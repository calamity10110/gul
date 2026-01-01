use super::{Benchmark, BenchmarkResult};
use std::time::Instant;

pub struct RuntimeBenchmark {
    pub iterations: usize,
}

impl RuntimeBenchmark {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }
}

impl Benchmark for RuntimeBenchmark {
    fn name(&self) -> &str {
        "Runtime Benchmark"
    }

    fn run(&self) -> BenchmarkResult {
        let start = Instant::now();

        // Simulate runtime execution
        let mut sum = 0;
        for i in 0..self.iterations {
            sum += i;
        }

        // Prevent optimization
        if sum == 0 {
            println!("Sum is 0");
        }

        let duration = start.elapsed();

        BenchmarkResult {
            name: self.name().to_string(),
            duration,
            memory_usage: std::mem::size_of::<usize>(),
        }
    }
}
