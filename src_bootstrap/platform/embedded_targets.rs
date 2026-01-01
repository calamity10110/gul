// Embedded Targets Support for GUL
// Provides support for ESP32, RP2040, STM32, Arduino, Nordic nRF52, and embedded HAL

use std::collections::HashMap;

/// Embedded target specification
#[derive(Debug, Clone, PartialEq)]
pub enum EmbeddedTarget {
    ESP32 {
        variant: ESP32Variant,
        flash_size: usize,
        psram: bool,
    },
    RP2040 {
        flash_size: usize,
        core_count: u8,
    },
    STM32 {
        series: STM32Series,
        flash_size: usize,
        ram_size: usize,
    },
    Arduino {
        board: ArduinoBoard,
    },
    NordicNRF52 {
        variant: NRF52Variant,
        flash_size: usize,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ESP32Variant {
    ESP32,
    ESP32S2,
    ESP32S3,
    ESP32C3,
    ESP32C6,
    ESP32H2,
}

#[derive(Debug, Clone, PartialEq)]
pub enum STM32Series {
    F0,
    F1,
    F2,
    F3,
    F4,
    F7,
    H7,
    L0,
    L1,
    L4,
    L5,
    G0,
    G4,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArduinoBoard {
    Uno,
    Mega,
    Nano,
    Due,
    MKR1000,
    Portenta,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NRF52Variant {
    NRF52832,
    NRF52833,
    NRF52840,
}

/// Embedded HAL (Hardware Abstraction Layer)
pub struct EmbeddedHAL {
    target: EmbeddedTarget,
    peripherals: HashMap<String, Peripheral>,
}

#[derive(Debug, Clone)]
pub struct Peripheral {
    pub name: String,
    pub kind: PeripheralKind,
    pub base_address: u32,
    pub interrupt: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PeripheralKind {
    GPIO,
    UART,
    SPI,
    I2C,
    ADC,
    DAC,
    PWM,
    Timer,
    RTC,
    DMA,
    USB,
    CAN,
    Ethernet,
    WiFi,
    Bluetooth,
}

impl EmbeddedHAL {
    pub fn new(target: EmbeddedTarget) -> Self {
        let mut hal = EmbeddedHAL {
            target: target.clone(),
            peripherals: HashMap::new(),
        };
        hal.initialize_peripherals();
        hal
    }

    fn initialize_peripherals(&mut self) {
        let target = self.target.clone();
        match target {
            EmbeddedTarget::ESP32 { variant, .. } => self.init_esp32_peripherals(&variant),
            EmbeddedTarget::RP2040 { .. } => self.init_rp2040_peripherals(),
            EmbeddedTarget::STM32 { series, .. } => self.init_stm32_peripherals(&series),
            EmbeddedTarget::Arduino { board } => self.init_arduino_peripherals(&board),
            EmbeddedTarget::NordicNRF52 { variant, .. } => self.init_nrf52_peripherals(&variant),
        }
    }

    fn init_esp32_peripherals(&mut self, variant: &ESP32Variant) {
        // GPIO
        self.peripherals.insert(
            "GPIO".to_string(),
            Peripheral {
                name: "GPIO".to_string(),
                kind: PeripheralKind::GPIO,
                base_address: 0x3FF44000,
                interrupt: Some(13),
            },
        );

        // UART0
        self.peripherals.insert(
            "UART0".to_string(),
            Peripheral {
                name: "UART0".to_string(),
                kind: PeripheralKind::UART,
                base_address: 0x3FF40000,
                interrupt: Some(34),
            },
        );

        // SPI2
        self.peripherals.insert(
            "SPI2".to_string(),
            Peripheral {
                name: "SPI2".to_string(),
                kind: PeripheralKind::SPI,
                base_address: 0x3FF64000,
                interrupt: Some(30),
            },
        );

        // I2C0
        self.peripherals.insert(
            "I2C0".to_string(),
            Peripheral {
                name: "I2C0".to_string(),
                kind: PeripheralKind::I2C,
                base_address: 0x3FF53000,
                interrupt: Some(49),
            },
        );

        // WiFi (ESP32 specific)
        self.peripherals.insert(
            "WiFi".to_string(),
            Peripheral {
                name: "WiFi".to_string(),
                kind: PeripheralKind::WiFi,
                base_address: 0x3FF73000,
                interrupt: Some(0),
            },
        );

        // Bluetooth (ESP32 specific)
        if matches!(
            variant,
            ESP32Variant::ESP32 | ESP32Variant::ESP32S3 | ESP32Variant::ESP32C3
        ) {
            self.peripherals.insert(
                "Bluetooth".to_string(),
                Peripheral {
                    name: "Bluetooth".to_string(),
                    kind: PeripheralKind::Bluetooth,
                    base_address: 0x3FF68000,
                    interrupt: Some(1),
                },
            );
        }
    }

    fn init_rp2040_peripherals(&mut self) {
        // GPIO
        self.peripherals.insert(
            "GPIO".to_string(),
            Peripheral {
                name: "GPIO".to_string(),
                kind: PeripheralKind::GPIO,
                base_address: 0x40014000,
                interrupt: Some(13),
            },
        );

        // UART0
        self.peripherals.insert(
            "UART0".to_string(),
            Peripheral {
                name: "UART0".to_string(),
                kind: PeripheralKind::UART,
                base_address: 0x40034000,
                interrupt: Some(20),
            },
        );

        // SPI0
        self.peripherals.insert(
            "SPI0".to_string(),
            Peripheral {
                name: "SPI0".to_string(),
                kind: PeripheralKind::SPI,
                base_address: 0x4003C000,
                interrupt: Some(18),
            },
        );

        // I2C0
        self.peripherals.insert(
            "I2C0".to_string(),
            Peripheral {
                name: "I2C0".to_string(),
                kind: PeripheralKind::I2C,
                base_address: 0x40044000,
                interrupt: Some(23),
            },
        );

        // ADC
        self.peripherals.insert(
            "ADC".to_string(),
            Peripheral {
                name: "ADC".to_string(),
                kind: PeripheralKind::ADC,
                base_address: 0x4004C000,
                interrupt: Some(22),
            },
        );

        // PWM
        self.peripherals.insert(
            "PWM".to_string(),
            Peripheral {
                name: "PWM".to_string(),
                kind: PeripheralKind::PWM,
                base_address: 0x40050000,
                interrupt: Some(4),
            },
        );

        // USB
        self.peripherals.insert(
            "USB".to_string(),
            Peripheral {
                name: "USB".to_string(),
                kind: PeripheralKind::USB,
                base_address: 0x50100000,
                interrupt: Some(5),
            },
        );
    }

    fn init_stm32_peripherals(&mut self, series: &STM32Series) {
        // GPIO (Port A)
        self.peripherals.insert(
            "GPIOA".to_string(),
            Peripheral {
                name: "GPIOA".to_string(),
                kind: PeripheralKind::GPIO,
                base_address: 0x40020000,
                interrupt: None,
            },
        );

        // USART1
        self.peripherals.insert(
            "USART1".to_string(),
            Peripheral {
                name: "USART1".to_string(),
                kind: PeripheralKind::UART,
                base_address: 0x40011000,
                interrupt: Some(37),
            },
        );

        // SPI1
        self.peripherals.insert(
            "SPI1".to_string(),
            Peripheral {
                name: "SPI1".to_string(),
                kind: PeripheralKind::SPI,
                base_address: 0x40013000,
                interrupt: Some(35),
            },
        );

        // I2C1
        self.peripherals.insert(
            "I2C1".to_string(),
            Peripheral {
                name: "I2C1".to_string(),
                kind: PeripheralKind::I2C,
                base_address: 0x40005400,
                interrupt: Some(31),
            },
        );

        // ADC1
        self.peripherals.insert(
            "ADC1".to_string(),
            Peripheral {
                name: "ADC1".to_string(),
                kind: PeripheralKind::ADC,
                base_address: 0x40012000,
                interrupt: Some(18),
            },
        );

        // TIM1
        self.peripherals.insert(
            "TIM1".to_string(),
            Peripheral {
                name: "TIM1".to_string(),
                kind: PeripheralKind::Timer,
                base_address: 0x40010000,
                interrupt: Some(25),
            },
        );

        // USB (if supported by series)
        if matches!(series, STM32Series::F4 | STM32Series::F7 | STM32Series::H7) {
            self.peripherals.insert(
                "USB_OTG_FS".to_string(),
                Peripheral {
                    name: "USB_OTG_FS".to_string(),
                    kind: PeripheralKind::USB,
                    base_address: 0x50000000,
                    interrupt: Some(67),
                },
            );
        }

        // Ethernet (if supported by series)
        if matches!(series, STM32Series::F4 | STM32Series::F7 | STM32Series::H7) {
            self.peripherals.insert(
                "ETH".to_string(),
                Peripheral {
                    name: "ETH".to_string(),
                    kind: PeripheralKind::Ethernet,
                    base_address: 0x40028000,
                    interrupt: Some(61),
                },
            );
        }
    }

    fn init_arduino_peripherals(&mut self, board: &ArduinoBoard) {
        // Digital I/O
        self.peripherals.insert(
            "Digital_IO".to_string(),
            Peripheral {
                name: "Digital_IO".to_string(),
                kind: PeripheralKind::GPIO,
                base_address: 0x00,
                interrupt: None,
            },
        );

        // UART
        self.peripherals.insert(
            "Serial".to_string(),
            Peripheral {
                name: "Serial".to_string(),
                kind: PeripheralKind::UART,
                base_address: 0xC0,
                interrupt: Some(18),
            },
        );

        // SPI
        self.peripherals.insert(
            "SPI".to_string(),
            Peripheral {
                name: "SPI".to_string(),
                kind: PeripheralKind::SPI,
                base_address: 0x4C,
                interrupt: Some(17),
            },
        );

        // I2C (Wire)
        self.peripherals.insert(
            "Wire".to_string(),
            Peripheral {
                name: "Wire".to_string(),
                kind: PeripheralKind::I2C,
                base_address: 0xB8,
                interrupt: Some(24),
            },
        );

        // Analog Input
        self.peripherals.insert(
            "Analog".to_string(),
            Peripheral {
                name: "Analog".to_string(),
                kind: PeripheralKind::ADC,
                base_address: 0x78,
                interrupt: Some(21),
            },
        );

        // PWM
        self.peripherals.insert(
            "PWM".to_string(),
            Peripheral {
                name: "PWM".to_string(),
                kind: PeripheralKind::PWM,
                base_address: 0x80,
                interrupt: Some(13),
            },
        );

        // USB (for boards that support it)
        if matches!(
            board,
            ArduinoBoard::Due | ArduinoBoard::MKR1000 | ArduinoBoard::Portenta
        ) {
            self.peripherals.insert(
                "USB".to_string(),
                Peripheral {
                    name: "USB".to_string(),
                    kind: PeripheralKind::USB,
                    base_address: 0x400E4000,
                    interrupt: Some(34),
                },
            );
        }
    }

    fn init_nrf52_peripherals(&mut self, variant: &NRF52Variant) {
        // GPIO (Port 0)
        self.peripherals.insert(
            "P0".to_string(),
            Peripheral {
                name: "P0".to_string(),
                kind: PeripheralKind::GPIO,
                base_address: 0x50000000,
                interrupt: None,
            },
        );

        // UART0
        self.peripherals.insert(
            "UART0".to_string(),
            Peripheral {
                name: "UART0".to_string(),
                kind: PeripheralKind::UART,
                base_address: 0x40002000,
                interrupt: Some(2),
            },
        );

        // SPI0
        self.peripherals.insert(
            "SPI0".to_string(),
            Peripheral {
                name: "SPI0".to_string(),
                kind: PeripheralKind::SPI,
                base_address: 0x40003000,
                interrupt: Some(3),
            },
        );

        // TWI0 (I2C)
        self.peripherals.insert(
            "TWI0".to_string(),
            Peripheral {
                name: "TWI0".to_string(),
                kind: PeripheralKind::I2C,
                base_address: 0x40003000,
                interrupt: Some(3),
            },
        );

        // ADC/SAADC
        self.peripherals.insert(
            "SAADC".to_string(),
            Peripheral {
                name: "SAADC".to_string(),
                kind: PeripheralKind::ADC,
                base_address: 0x40007000,
                interrupt: Some(7),
            },
        );

        // PWM0
        self.peripherals.insert(
            "PWM0".to_string(),
            Peripheral {
                name: "PWM0".to_string(),
                kind: PeripheralKind::PWM,
                base_address: 0x4001C000,
                interrupt: Some(28),
            },
        );

        // Bluetooth Radio
        self.peripherals.insert(
            "RADIO".to_string(),
            Peripheral {
                name: "RADIO".to_string(),
                kind: PeripheralKind::Bluetooth,
                base_address: 0x40001000,
                interrupt: Some(1),
            },
        );

        // USB (nRF52840 only)
        if matches!(variant, NRF52Variant::NRF52840) {
            self.peripherals.insert(
                "USBD".to_string(),
                Peripheral {
                    name: "USBD".to_string(),
                    kind: PeripheralKind::USB,
                    base_address: 0x40027000,
                    interrupt: Some(39),
                },
            );
        }

        // RTC
        self.peripherals.insert(
            "RTC0".to_string(),
            Peripheral {
                name: "RTC0".to_string(),
                kind: PeripheralKind::RTC,
                base_address: 0x4000B000,
                interrupt: Some(11),
            },
        );
    }

    // Public API methods

    pub fn get_peripheral(&self, name: &str) -> Option<&Peripheral> {
        self.peripherals.get(name)
    }

    pub fn list_peripherals(&self) -> Vec<&Peripheral> {
        self.peripherals.values().collect()
    }

    pub fn has_peripheral(&self, name: &str) -> bool {
        self.peripherals.contains_key(name)
    }

    pub fn get_target(&self) -> &EmbeddedTarget {
        &self.target
    }

    pub fn peripheral_count(&self) -> usize {
        self.peripherals.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esp32_hal() {
        let target = EmbeddedTarget::ESP32 {
            variant: ESP32Variant::ESP32S3,
            flash_size: 8 * 1024 * 1024,
            psram: true,
        };
        let hal = EmbeddedHAL::new(target);

        assert!(hal.has_peripheral("GPIO"));
        assert!(hal.has_peripheral("UART0"));
        assert!(hal.has_peripheral("WiFi"));
        assert!(hal.has_peripheral("Bluetooth"));
    }

    #[test]
    fn test_rp2040_hal() {
        let target = EmbeddedTarget::RP2040 {
            flash_size: 2 * 1024 * 1024,
            core_count: 2,
        };
        let hal = EmbeddedHAL::new(target);

        assert!(hal.has_peripheral("GPIO"));
        assert!(hal.has_peripheral("UART0"));
        assert!(hal.has_peripheral("SPI0"));
        assert!(hal.has_peripheral("USB"));
    }

    #[test]
    fn test_stm32_hal() {
        let target = EmbeddedTarget::STM32 {
            series: STM32Series::F4,
            flash_size: 512 * 1024,
            ram_size: 128 * 1024,
        };
        let hal = EmbeddedHAL::new(target);

        assert!(hal.has_peripheral("GPIOA"));
        assert!(hal.has_peripheral("USART1"));
        assert!(hal.has_peripheral("USB_OTG_FS"));
        assert!(hal.has_peripheral("ETH"));
    }

    #[test]
    fn test_arduino_hal() {
        let target = EmbeddedTarget::Arduino {
            board: ArduinoBoard::Uno,
        };
        let hal = EmbeddedHAL::new(target);

        assert!(hal.has_peripheral("Digital_IO"));
        assert!(hal.has_peripheral("Serial"));
        assert!(hal.has_peripheral("SPI"));
        assert!(hal.has_peripheral("Wire"));
    }

    #[test]
    fn test_nrf52_hal() {
        let target = EmbeddedTarget::NordicNRF52 {
            variant: NRF52Variant::NRF52840,
            flash_size: 1024 * 1024,
        };
        let hal = EmbeddedHAL::new(target);

        assert!(hal.has_peripheral("P0"));
        assert!(hal.has_peripheral("UART0"));
        assert!(hal.has_peripheral("RADIO"));
        assert!(hal.has_peripheral("USBD"));
    }

    #[test]
    fn test_peripheral_list() {
        let target = EmbeddedTarget::RP2040 {
            flash_size: 2 * 1024 * 1024,
            core_count: 2,
        };
        let hal = EmbeddedHAL::new(target);

        let peripherals = hal.list_peripherals();
        assert!(!peripherals.is_empty());
    }
}
