use serde::Serialize;

pub trait Sensor {
    type Data;
    fn read(&mut self) -> Result<Self::Data, String>;
}

#[derive(Debug, Serialize)]
pub struct ImuData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
}

pub struct MockImu;

impl Sensor for MockImu {
    type Data = ImuData;
    fn read(&mut self) -> Result<Self::Data, String> {
        Ok(ImuData {
            accel_x: 0.01,
            accel_y: 0.0,
            accel_z: 9.81,
            gyro_x: 0.0,
            gyro_y: 0.0,
            gyro_z: 0.0,
        })
    }
}

pub struct MockThermometer;
impl Sensor for MockThermometer {
    type Data = f32; // Celsius
    fn read(&mut self) -> Result<f32, String> {
        Ok(25.5) // Mock Room Temp
    }
}
