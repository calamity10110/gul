pub mod compiler_bench;
pub mod runtime_bench;

pub struct BenchmarkResult {
    pub name: String,
    pub duration: std::time::Duration,
    pub memory_usage: usize,
}

pub trait Benchmark {
    fn name(&self) -> &str;
    fn run(&self) -> BenchmarkResult;
}
