use gul_sensors::{MockImu, Sensor};

fn main() {
    let mut imu = MockImu;
    println!("IMU: {:?}", imu.read().unwrap());
}
