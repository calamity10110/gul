use gul_gpio::PinDriver;

pub struct Motor {
    pin_a: PinDriver,
    pin_b: PinDriver,
}

impl Motor {
    pub fn new(pin_a_num: i32, pin_b_num: i32) -> Self {
        Self {
            pin_a: PinDriver::new_output(pin_a_num),
            pin_b: PinDriver::new_output(pin_b_num),
        }
    }

    pub fn forward(&mut self) {
        self.pin_a.high();
        self.pin_b.low();
    }

    pub fn stop(&mut self) {
        self.pin_a.low();
        self.pin_b.low();
    }
}
