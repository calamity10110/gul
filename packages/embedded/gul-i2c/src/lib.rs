#[cfg(target_os = "espidf")]
use esp_idf_hal::{gpio::*, i2c::*, peripherals::Peripherals, units::FromValueType};
#[cfg(target_os = "espidf")]
use esp_idf_sys::EspError;

pub struct I2CDriver {
    #[cfg(target_os = "espidf")]
    driver: I2cDriver<'static>,
    #[cfg(not(target_os = "espidf"))]
    driver: (),
}

impl I2CDriver {
    /// Initialize Hardware I2C Master (up to 400kHz or 1MHz)
    pub fn new_master(sda: i32, scl: i32, freq_hz: u32) -> Self {
        #[cfg(target_os = "espidf")]
        {
            let peripherals = Peripherals::take().unwrap();
            let i2c = peripherals.i2c0;
            // Simplified dynamic pin mapping for GUL usage
            let sda_pin = unsafe { AnyIOPin::new(sda) };
            let scl_pin = unsafe { AnyIOPin::new(scl) };

            let config = I2cConfig::new().baudrate(freq_hz.Hz());
            let driver = I2cDriver::new(i2c, sda_pin, scl_pin, &config).unwrap();

            Self { driver }
        }
        #[cfg(not(target_os = "espidf"))]
        {
            let _ = (sda, scl, freq_hz);
            Self { driver: () }
        }
    }

    #[cfg(target_os = "espidf")]
    pub fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<(), EspError> {
        self.driver.write_read(addr, write, read, 100)
    }

    #[cfg(not(target_os = "espidf"))]
    pub fn write_read(&mut self, _addr: u8, _write: &[u8], _read: &mut [u8]) -> Result<(), String> {
        Ok(())
    }
}
