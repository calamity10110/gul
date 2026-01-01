use gul_motors::Motor;

fn main() {
    let mut car = Motor::new(1, 2);
    car.forward();
    car.stop();
}
