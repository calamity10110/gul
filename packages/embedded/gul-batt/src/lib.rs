#[cfg(target_os = "espidf")]
use esp_idf_hal::{adc::*, gpio::*, peripherals::Peripherals};
#[cfg(target_os = "espidf")]
use std::sync::{Arc, Mutex};

pub struct Battery {
    #[cfg(target_os = "espidf")]
    driver: Arc<Mutex<AdcDriver<'static, ADC1>>>,
    #[cfg(target_os = "espidf")]
    channel: AdcChannelDriver<'static, { attenuation::DB_11 }, Gpio4>, // Default Pin 4
    #[cfg(target_os = "espidf")]
    charging_control: Option<PinDriver<'static, Gpio5, Output>>, // Hypothetical Charge Control
}

impl Default for Battery {
    fn default() -> Self {
        Self::new()
    }
}

impl Battery {
    pub fn new() -> Self {
        #[cfg(target_os = "espidf")]
        {
            let peripherals = Peripherals::take().unwrap();
            let adc1 = peripherals.adc1;
            let pin = peripherals.pins.gpio4;

            let config = AdcConfig::new().calibration(true);
            let driver = AdcDriver::new(adc1, &config).unwrap();
            let channel = AdcChannelDriver::new(pin).unwrap();

            Self {
                driver: Arc::new(Mutex::new(driver)),
                channel,
                charging_control: None,
            }
        }
        #[cfg(not(target_os = "espidf"))]
        {
            log::info!("Battery Manager (Host Stub)");
            Self {}
        }
    }

    pub fn voltage(&mut self) -> f32 {
        #[cfg(target_os = "espidf")]
        {
            let mut driver = self.driver.lock().unwrap();
            match driver.read(&mut self.channel) {
                Ok(raw) => {
                    // Raw is tens of mV usually with calibration?
                    // Approx: raw to mV * divider (2/1) -> / 1000.0
                    (raw as f32) / 1000.0 * 2.0
                }
                Err(_) => 0.0,
            }
        }
        #[cfg(not(target_os = "espidf"))]
        3.7
    }

    pub fn percent(&mut self) -> u8 {
        let v = self.voltage();
        // Linear approx for LiPo 3.0v - 4.2v
        if v >= 4.2 {
            100
        } else if v <= 3.0 {
            0
        } else {
            ((v - 3.0) / 1.2 * 100.0) as u8
        }
    }
}
