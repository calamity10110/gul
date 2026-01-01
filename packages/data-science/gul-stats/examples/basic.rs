use gul_stats::Series;

fn main() {
    let data = Series::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    println!("Mean: {}", data.mean());
    println!("StdDev: {}", data.standard_deviation());
}
