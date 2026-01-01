pub struct Servo {
    pin: u8,
    angle: f32,
}

impl Servo {
    pub fn new(pin: u8) -> Self {
        log::info!("Initialized Servo on Pin {} (MOCK)", pin);
        Self { pin, angle: 0.0 }
    }

    pub fn set_angle(&mut self, angle: f32) {
        let clamped = angle.max(0.0).min(180.0);
        self.angle = clamped;

        #[cfg(feature = "real")]
        {
            // Real PWM implementation stub using rppal if available
            // let mut pwm = ...
        }

        println!(
            "MOCK SERVO [Pin {}] -> Angle {:.1} deg",
            self.pin, self.angle
        );
    }
}
