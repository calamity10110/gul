// Hardware Abstraction Layer (HAL) for Embedded Systems

use std::collections::HashMap;

// GPIO Abstraction
#[derive(Debug, Clone, PartialEq)]
pub enum PinMode {
    Input,
    Output,
    InputPullup,
    InputPulldown,
    Analog,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PinState {
    Low,
    High,
}

pub struct GpioPin {
    pin_number: u8,
    mode: PinMode,
    state: PinState,
}

impl GpioPin {
    pub fn new(pin_number: u8, mode: PinMode) -> Self {
        GpioPin {
            pin_number,
            mode,
            state: PinState::Low,
        }
    }

    pub fn set_high(&mut self) {
        self.state = PinState::High;
    }

    pub fn set_low(&mut self) {
        self.state = PinState::Low;
    }

    pub fn read(&self) -> PinState {
        self.state.clone()
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            PinState::Low => PinState::High,
            PinState::High => PinState::Low,
        };
    }

    pub fn get_mode(&self) -> &PinMode {
        &self.mode
    }
}

// I2C Abstraction
pub struct I2cBus {
    address: u8,
    speed_khz: u32,
    buffer: Vec<u8>,
}

impl I2cBus {
    pub fn new(address: u8, speed_khz: u32) -> Self {
        I2cBus {
            address,
            speed_khz,
            buffer: Vec::new(),
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), String> {
        self.buffer.extend_from_slice(data);
        Ok(())
    }

    pub fn read(&mut self, len: usize) -> Result<Vec<u8>, String> {
        if self.buffer.len() >= len {
            let data = self.buffer.drain(..len).collect();
            Ok(data)
        } else {
            Err("Not enough data in buffer".to_string())
        }
    }

    pub fn write_read(&mut self, write_data: &[u8], read_len: usize) -> Result<Vec<u8>, String> {
        self.write(write_data)?;
        self.read(read_len)
    }

    pub fn get_address(&self) -> u8 {
        self.address
    }
}

// SPI Abstraction
pub struct SpiBus {
    speed_mhz: u32,
    mode: SpiMode,
    buffer: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpiMode {
    Mode0, // CPOL=0, CPHA=0
    Mode1, // CPOL=0, CPHA=1
    Mode2, // CPOL=1, CPHA=0
    Mode3, // CPOL=1, CPHA=1
}

impl SpiBus {
    pub fn new(speed_mhz: u32, mode: SpiMode) -> Self {
        SpiBus {
            speed_mhz,
            mode,
            buffer: Vec::new(),
        }
    }

    pub fn transfer(&mut self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Simulate SPI transfer
        self.buffer.extend_from_slice(data);
        Ok(data.to_vec())
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), String> {
        self.buffer.extend_from_slice(data);
        Ok(())
    }

    pub fn get_mode(&self) -> &SpiMode {
        &self.mode
    }
}

// UART Abstraction
pub struct UartPort {
    baud_rate: u32,
    rx_buffer: Vec<u8>,
    tx_buffer: Vec<u8>,
}

impl UartPort {
    pub fn new(baud_rate: u32) -> Self {
        UartPort {
            baud_rate,
            rx_buffer: Vec::new(),
            tx_buffer: Vec::new(),
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        self.tx_buffer.extend_from_slice(data);
        Ok(data.len())
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, String> {
        let len = buffer.len().min(self.rx_buffer.len());
        buffer[..len].copy_from_slice(&self.rx_buffer[..len]);
        self.rx_buffer.drain(..len);
        Ok(len)
    }

    pub fn available(&self) -> usize {
        self.rx_buffer.len()
    }

    pub fn get_baud_rate(&self) -> u32 {
        self.baud_rate
    }
}

// PWM Abstraction
pub struct PwmChannel {
    channel: u8,
    frequency_hz: u32,
    duty_cycle: f32, // 0.0 to 1.0
}

impl PwmChannel {
    pub fn new(channel: u8, frequency_hz: u32) -> Self {
        PwmChannel {
            channel,
            frequency_hz,
            duty_cycle: 0.0,
        }
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: f32) -> Result<(), String> {
        if !(0.0..=1.0).contains(&duty_cycle) {
            return Err("Duty cycle must be between 0.0 and 1.0".to_string());
        }
        self.duty_cycle = duty_cycle;
        Ok(())
    }

    pub fn get_duty_cycle(&self) -> f32 {
        self.duty_cycle
    }

    pub fn set_frequency(&mut self, frequency_hz: u32) {
        self.frequency_hz = frequency_hz;
    }

    pub fn get_frequency(&self) -> u32 {
        self.frequency_hz
    }
}

// ADC Abstraction
pub struct AdcChannel {
    channel: u8,
    resolution_bits: u8,
    reference_voltage_mv: u32,
}

impl AdcChannel {
    pub fn new(channel: u8, resolution_bits: u8, reference_voltage_mv: u32) -> Self {
        AdcChannel {
            channel,
            resolution_bits,
            reference_voltage_mv,
        }
    }

    pub fn read_raw(&self) -> u16 {
        // Simulate ADC reading
        0
    }

    pub fn read_voltage(&self) -> u32 {
        let raw = self.read_raw();
        let max_value = (1 << self.resolution_bits) - 1;
        (raw as u32 * self.reference_voltage_mv) / max_value as u32
    }

    pub fn get_resolution(&self) -> u8 {
        self.resolution_bits
    }
}

// DAC Abstraction
pub struct DacChannel {
    channel: u8,
    resolution_bits: u8,
    reference_voltage_mv: u32,
    current_value: u16,
}

impl DacChannel {
    pub fn new(channel: u8, resolution_bits: u8, reference_voltage_mv: u32) -> Self {
        DacChannel {
            channel,
            resolution_bits,
            reference_voltage_mv,
            current_value: 0,
        }
    }

    pub fn write_raw(&mut self, value: u16) -> Result<(), String> {
        let max_value = (1 << self.resolution_bits) - 1;
        if value > max_value {
            return Err(format!("Value exceeds maximum {}", max_value));
        }
        self.current_value = value;
        Ok(())
    }

    pub fn write_voltage(&mut self, voltage_mv: u32) -> Result<(), String> {
        let max_value = (1 << self.resolution_bits) - 1;
        let raw_value =
            ((voltage_mv as u64 * max_value as u64) / self.reference_voltage_mv as u64) as u16;
        self.write_raw(raw_value)
    }

    pub fn get_current_value(&self) -> u16 {
        self.current_value
    }
}

// HAL Manager
pub struct HalManager {
    gpio_pins: HashMap<u8, GpioPin>,
    i2c_buses: HashMap<u8, I2cBus>,
    spi_buses: HashMap<u8, SpiBus>,
    uart_ports: HashMap<u8, UartPort>,
}

impl HalManager {
    pub fn new() -> Self {
        HalManager {
            gpio_pins: HashMap::new(),
            i2c_buses: HashMap::new(),
            spi_buses: HashMap::new(),
            uart_ports: HashMap::new(),
        }
    }

    pub fn init_gpio(&mut self, pin: u8, mode: PinMode) {
        self.gpio_pins.insert(pin, GpioPin::new(pin, mode));
    }

    pub fn get_gpio(&mut self, pin: u8) -> Option<&mut GpioPin> {
        self.gpio_pins.get_mut(&pin)
    }

    pub fn init_i2c(&mut self, bus_id: u8, address: u8, speed_khz: u32) {
        self.i2c_buses
            .insert(bus_id, I2cBus::new(address, speed_khz));
    }

    pub fn get_i2c(&mut self, bus_id: u8) -> Option<&mut I2cBus> {
        self.i2c_buses.get_mut(&bus_id)
    }
}

impl Default for HalManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpio_pin() {
        let mut pin = GpioPin::new(5, PinMode::Output);
        assert_eq!(pin.read(), PinState::Low);
        pin.set_high();
        assert_eq!(pin.read(), PinState::High);
        pin.toggle();
        assert_eq!(pin.read(), PinState::Low);
    }

    #[test]
    fn test_i2c_bus() {
        let mut i2c = I2cBus::new(0x50, 100);
        assert!(i2c.write(&[0x01, 0x02]).is_ok());
        assert_eq!(i2c.get_address(), 0x50);
    }

    #[test]
    fn test_spi_bus() {
        let mut spi = SpiBus::new(10, SpiMode::Mode0);
        let result = spi.transfer(&[0x01, 0x02, 0x03]);
        assert!(result.is_ok());
        assert_eq!(*spi.get_mode(), SpiMode::Mode0);
    }

    #[test]
    fn test_uart_port() {
        let mut uart = UartPort::new(115200);
        assert!(uart.write(b"Hello").is_ok());
        assert_eq!(uart.get_baud_rate(), 115200);
    }

    #[test]
    fn test_pwm_channel() {
        let mut pwm = PwmChannel::new(0, 1000);
        assert!(pwm.set_duty_cycle(0.5).is_ok());
        assert_eq!(pwm.get_duty_cycle(), 0.5);
        assert!(pwm.set_duty_cycle(1.5).is_err());
    }

    #[test]
    fn test_adc_channel() {
        let adc = AdcChannel::new(0, 12, 3300);
        assert_eq!(adc.get_resolution(), 12);
    }

    #[test]
    fn test_dac_channel() {
        let mut dac = DacChannel::new(0, 12, 3300);
        assert!(dac.write_raw(2048).is_ok());
        assert_eq!(dac.get_current_value(), 2048);
    }

    #[test]
    fn test_hal_manager() {
        let mut hal = HalManager::new();
        hal.init_gpio(5, PinMode::Output);
        assert!(hal.get_gpio(5).is_some());

        hal.init_i2c(0, 0x50, 100);
        assert!(hal.get_i2c(0).is_some());
    }
}
