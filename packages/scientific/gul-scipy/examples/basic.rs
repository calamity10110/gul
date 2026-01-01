use gul_scipy::Matrix;

fn main() {
    let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let b = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

    let c = a.dot(&b);
    println!("Dot Product Result [0,0]: {}", c.get(0, 0));
}
