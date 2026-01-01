#[cfg(target_os = "espidf")]
use esp_idf_hal::{gpio::*, i2s::*, peripherals::Peripherals};

pub struct AudioDriver {
    #[cfg(target_os = "espidf")]
    driver: I2sDriver<'static, I2sTx>,
    #[cfg(not(target_os = "espidf"))]
    driver: (),
}

impl AudioDriver {
    pub fn new_output(bclk: i32, ws: i32, dout: i32) -> Self {
        #[cfg(target_os = "espidf")]
        {
            let peripherals = Peripherals::take().unwrap();
            let i2s = peripherals.i2s0;

            // Standard config: 44.1kHz, 16-bit, Stereo
            let config = I2sConfig::new_std();

            let driver = I2sDriver::new_tx(i2s, &config, Default::default()).unwrap();
            Self { driver }
        }
        #[cfg(not(target_os = "espidf"))]
        {
            let _ = (bclk, ws, dout);
            Self { driver: () }
        }
    }

    pub fn play_buffer(&mut self, data: &[u8]) {
        #[cfg(target_os = "espidf")]
        self.driver.write(data, 100).unwrap();
    }
}
