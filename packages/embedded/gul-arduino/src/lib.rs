pub struct Arduino;

impl Arduino {
    pub fn delay(ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }

    pub fn digital_write(pin: u32, val: u8) {
        log::info!("Arduino: digitalWrite({}, {})", pin, val);
    }
}
