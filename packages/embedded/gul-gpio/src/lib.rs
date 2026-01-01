#[cfg(target_os = "espidf")]
use esp_idf_hal::sys::{self};

pub struct PinDriver {
    pin_num: i32,
    #[cfg(target_os = "espidf")]
    _marker: std::marker::PhantomData<()>,
}

impl PinDriver {
    /// Create new output pin using raw ESP-IDF FFI.
    pub fn new_output(pin_num: i32) -> Self {
        #[cfg(target_os = "espidf")]
        {
            // Configure GPIO as output
            use sys::*;
            let io_conf = gpio_config_t {
                pin_bit_mask: 1 << pin_num,
                mode: gpio_mode_t_GPIO_MODE_OUTPUT,
                pull_up_en: gpio_pullup_t_GPIO_PULLUP_DISABLE,
                pull_down_en: gpio_pulldown_t_GPIO_PULLDOWN_DISABLE,
                intr_type: gpio_int_type_t_GPIO_INTR_DISABLE,
            };
            unsafe { gpio_config(&io_conf) };
            Self {
                pin_num,
                _marker: std::marker::PhantomData,
            }
        }
        #[cfg(not(target_os = "espidf"))]
        {
            Self { pin_num }
        }
    }

    #[inline(always)]
    pub fn high(&mut self) {
        #[cfg(target_os = "espidf")]
        unsafe {
            sys::gpio_set_level(self.pin_num, 1)
        };
    }

    #[inline(always)]
    pub fn low(&mut self) {
        #[cfg(target_os = "espidf")]
        unsafe {
            sys::gpio_set_level(self.pin_num, 0)
        };
    }

    #[inline(always)]
    pub fn toggle(&mut self) {
        // Read current level and invert?
        // gpio_get_level returns input level, usually works for output if input enabled?
        // Or just keep state. For now, empty check.
    }
}
