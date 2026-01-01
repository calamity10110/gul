use gul_algorithms::sort::quick_sort;

fn main() {
    let mut data = vec![5, 1, 4, 2, 8];
    quick_sort(&mut data);
    println!("Sorted: {:?}", data);
}
