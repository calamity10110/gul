use gul_servo::Servo;

fn main() {
    let mut arm = Servo::new(18);
    arm.set_angle(90.0);
}
