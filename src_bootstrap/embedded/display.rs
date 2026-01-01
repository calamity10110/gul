// Embedded Display Support - Framebuffer, LCD, OLED, E-ink

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayType {
    LCD,
    OLED,
    EInk,
    Framebuffer,
}

#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub width: usize,
    pub height: usize,
    pub color_depth: usize, // bits per pixel
    pub display_type: DisplayType,
}

pub struct EmbeddedDisplay {
    config: DisplayConfig,
    framebuffer: Vec<u8>,
}

impl EmbeddedDisplay {
    pub fn new(config: DisplayConfig) -> Self {
        let buffer_size = (config.width * config.height * config.color_depth) / 8;
        EmbeddedDisplay {
            config,
            framebuffer: vec![0; buffer_size],
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(0);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) -> Result<(), String> {
        if x >= self.config.width || y >= self.config.height {
            return Err("Pixel coordinates out of bounds".to_string());
        }

        let index = (y * self.config.width + x) * (self.config.color_depth / 8);
        if index + (self.config.color_depth / 8) <= self.framebuffer.len() {
            // Simple color encoding (depends on color depth)
            match self.config.color_depth {
                1 => {
                    // Monochrome
                    let byte_index = index / 8;
                    let bit_index = index % 8;
                    if color > 0 {
                        self.framebuffer[byte_index] |= 1 << bit_index;
                    } else {
                        self.framebuffer[byte_index] &= !(1 << bit_index);
                    }
                }
                16 => {
                    // RGB565
                    let color_bytes = color.to_le_bytes();
                    self.framebuffer[index] = color_bytes[0];
                    self.framebuffer[index + 1] = color_bytes[1];
                }
                24 => {
                    // RGB888
                    let color_bytes = color.to_le_bytes();
                    self.framebuffer[index] = color_bytes[0];
                    self.framebuffer[index + 1] = color_bytes[1];
                    self.framebuffer[index + 2] = color_bytes[2];
                }
                _ => return Err("Unsupported color depth".to_string()),
            }
            Ok(())
        } else {
            Err("Buffer overflow".to_string())
        }
    }

    pub fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) -> Result<(), String> {
        for dy in 0..height {
            for dx in 0..width {
                self.set_pixel(x + dx, y + dy, color)?;
            }
        }
        Ok(())
    }

    pub fn draw_line(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        color: u32,
    ) -> Result<(), String> {
        // Bresenham's line algorithm
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1 as i32;
        let mut y = y1 as i32;

        loop {
            self.set_pixel(x as usize, y as usize, color)?;

            if x == x2 as i32 && y == y2 as i32 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        Ok(())
    }

    pub fn draw_circle(
        &mut self,
        center_x: usize,
        center_y: usize,
        radius: usize,
        color: u32,
    ) -> Result<(), String> {
        let mut x = radius as i32;
        let mut y = 0i32;
        let mut err = 0i32;

        while x >= y {
            self.set_pixel(
                (center_x as i32 + x) as usize,
                (center_y as i32 + y) as usize,
                color,
            )?;
            self.set_pixel(
                (center_x as i32 + y) as usize,
                (center_y as i32 + x) as usize,
                color,
            )?;
            self.set_pixel(
                (center_x as i32 - y) as usize,
                (center_y as i32 + x) as usize,
                color,
            )?;
            self.set_pixel(
                (center_x as i32 - x) as usize,
                (center_y as i32 + y) as usize,
                color,
            )?;
            self.set_pixel(
                (center_x as i32 - x) as usize,
                (center_y as i32 - y) as usize,
                color,
            )?;
            self.set_pixel(
                (center_x as i32 - y) as usize,
                (center_y as i32 - x) as usize,
                color,
            )?;
            self.set_pixel(
                (center_x as i32 + y) as usize,
                (center_y as i32 - x) as usize,
                color,
            )?;
            self.set_pixel(
                (center_x as i32 + x) as usize,
                (center_y as i32 - y) as usize,
                color,
            )?;

            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
        Ok(())
    }

    pub fn draw_text(
        &mut self,
        x: usize,
        y: usize,
        text: &str,
        color: u32,
        font_size: usize,
    ) -> Result<(), String> {
        // Simple bitmap font (5x7 pixels per character)
        let font = [
            [0x00, 0x00, 0x00, 0x00, 0x00], // space
            [0x00, 0x00, 0x5F, 0x00, 0x00], // !
            [0x00, 0x07, 0x00, 0x07, 0x00], // "
            [0x14, 0x7F, 0x14, 0x7F, 0x14], // #
            [0x24, 0x2A, 0x7F, 0x2A, 0x12], // $
            [0x23, 0x13, 0x08, 0x64, 0x62], // %
            [0x36, 0x49, 0x55, 0x22, 0x50], // &
            [0x00, 0x05, 0x03, 0x00, 0x00], // '
            [0x00, 0x1C, 0x22, 0x41, 0x00], // (
            [0x00, 0x41, 0x22, 0x1C, 0x00], // )
            [0x14, 0x08, 0x3E, 0x08, 0x14], // *
            [0x08, 0x08, 0x3E, 0x08, 0x08], // +
            [0x00, 0x50, 0x30, 0x00, 0x00], // ,
            [0x08, 0x08, 0x08, 0x08, 0x08], // -
            [0x00, 0x60, 0x60, 0x00, 0x00], // .
            [0x20, 0x10, 0x08, 0x04, 0x02], // /
            [0x3E, 0x51, 0x49, 0x45, 0x3E], // 0
            [0x00, 0x42, 0x7F, 0x40, 0x00], // 1
            [0x42, 0x61, 0x51, 0x49, 0x46], // 2
            [0x21, 0x41, 0x45, 0x4B, 0x31], // 3
            [0x18, 0x14, 0x12, 0x7F, 0x10], // 4
            [0x27, 0x45, 0x45, 0x45, 0x39], // 5
            [0x3C, 0x4A, 0x49, 0x49, 0x30], // 6
            [0x01, 0x71, 0x09, 0x05, 0x03], // 7
            [0x36, 0x49, 0x49, 0x49, 0x36], // 8
            [0x06, 0x49, 0x49, 0x29, 0x1E], // 9
            [0x00, 0x36, 0x36, 0x00, 0x00], // :
            [0x00, 0x56, 0x36, 0x00, 0x00], // ;
            [0x08, 0x14, 0x22, 0x41, 0x00], // <
            [0x14, 0x14, 0x14, 0x14, 0x14], // =
            [0x00, 0x41, 0x22, 0x14, 0x08], // >
            [0x02, 0x01, 0x51, 0x09, 0x06], // ?
            [0x32, 0x49, 0x79, 0x41, 0x3E], // @
            [0x7E, 0x11, 0x11, 0x11, 0x7E], // A
            [0x7F, 0x49, 0x49, 0x49, 0x36], // B
            [0x3E, 0x41, 0x41, 0x41, 0x22], // C
            [0x7F, 0x41, 0x41, 0x22, 0x1C], // D
            [0x7F, 0x49, 0x49, 0x49, 0x41], // E
            [0x7F, 0x09, 0x09, 0x09, 0x01], // F
            [0x3E, 0x41, 0x49, 0x49, 0x7A], // G
            [0x7F, 0x08, 0x08, 0x08, 0x7F], // H
            [0x00, 0x41, 0x7F, 0x41, 0x00], // I
            [0x20, 0x40, 0x41, 0x3F, 0x01], // J
            [0x7F, 0x08, 0x14, 0x22, 0x41], // K
            [0x7F, 0x40, 0x40, 0x40, 0x40], // L
            [0x7F, 0x02, 0x0C, 0x02, 0x7F], // M
            [0x7F, 0x04, 0x08, 0x10, 0x7F], // N
            [0x3E, 0x41, 0x41, 0x41, 0x3E], // O
            [0x7F, 0x09, 0x09, 0x09, 0x06], // P
            [0x3E, 0x41, 0x51, 0x21, 0x5E], // Q
            [0x7F, 0x09, 0x19, 0x29, 0x46], // R
            [0x46, 0x49, 0x49, 0x49, 0x31], // S
            [0x01, 0x01, 0x7F, 0x01, 0x01], // T
            [0x3F, 0x40, 0x40, 0x40, 0x3F], // U
            [0x1F, 0x20, 0x40, 0x20, 0x1F], // V
            [0x3F, 0x40, 0x38, 0x40, 0x3F], // W
            [0x63, 0x14, 0x08, 0x14, 0x63], // X
            [0x07, 0x08, 0x70, 0x08, 0x07], // Y
            [0x61, 0x51, 0x49, 0x45, 0x43], // Z
        ];

        let mut current_x = x;
        for ch in text.chars() {
            if ch == ' ' {
                current_x += 6 * font_size; // space width
                continue;
            }
            if ch.is_ascii() && ch.is_alphabetic() || ch.is_numeric() || ch.is_ascii_punctuation() {
                let char_index = if ch.is_ascii_uppercase() {
                    (ch as u8 - b'A' + 10) as usize
                } else if ch.is_ascii_lowercase() {
                    (ch as u8 - b'a' + 10) as usize
                } else if ch.is_numeric() {
                    (ch as u8 - b'0') as usize
                } else {
                    match ch {
                        '!' => 1,
                        '"' => 2,
                        '#' => 3,
                        '$' => 4,
                        '%' => 5,
                        '&' => 6,
                        '\'' => 7,
                        '(' => 8,
                        ')' => 9,
                        '*' => 10,
                        '+' => 11,
                        ',' => 12,
                        '-' => 13,
                        '.' => 14,
                        '/' => 15,
                        ':' => 16,
                        ';' => 17,
                        '<' => 18,
                        '=' => 19,
                        '>' => 20,
                        '?' => 21,
                        '@' => 22,
                        _ => 0,
                    }
                };

                #[allow(clippy::needless_range_loop)]
                if char_index < font.len() {
                    for row in 0..5 {
                        for col in 0..5 {
                            if font[char_index][row] & (1 << (4 - col)) != 0 {
                                for fy in 0..font_size {
                                    for fx in 0..font_size {
                                        self.set_pixel(
                                            current_x + col * font_size + fx,
                                            y + row * font_size + fy,
                                            color,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            current_x += 6 * font_size; // character width + space
        }
        Ok(())
    }

    pub fn refresh(&self) -> Result<(), String> {
        // Simulate display refresh
        match self.config.display_type {
            DisplayType::EInk => {
                // E-ink requires full refresh
                Ok(())
            }
            DisplayType::OLED | DisplayType::LCD | DisplayType::Framebuffer => {
                // Fast refresh
                Ok(())
            }
        }
    }

    pub fn get_framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }
}

// Touch input handling
#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub x: usize,
    pub y: usize,
    pub pressure: u8,
    pub event_type: TouchEventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TouchEventType {
    Press,
    Release,
    Move,
}

pub struct TouchController {
    calibration: HashMap<String, i32>,
}

impl TouchController {
    pub fn new() -> Self {
        TouchController {
            calibration: HashMap::new(),
        }
    }

    pub fn calibrate(&mut self, x_min: i32, x_max: i32, y_min: i32, y_max: i32) {
        self.calibration.insert("x_min".to_string(), x_min);
        self.calibration.insert("x_max".to_string(), x_max);
        self.calibration.insert("y_min".to_string(), y_min);
        self.calibration.insert("y_max".to_string(), y_max);
    }

    pub fn read_touch(&self) -> Option<TouchEvent> {
        // Simulate touch reading
        None
    }

    pub fn is_calibrated(&self) -> bool {
        self.calibration.contains_key("x_min")
            && self.calibration.contains_key("x_max")
            && self.calibration.contains_key("y_min")
            && self.calibration.contains_key("y_max")
    }
}

impl Default for TouchController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_creation() {
        let config = DisplayConfig {
            width: 128,
            height: 64,
            color_depth: 1,
            display_type: DisplayType::OLED,
        };
        let display = EmbeddedDisplay::new(config);
        assert_eq!(display.framebuffer.len(), (128 * 64) / 8);
    }

    #[test]
    fn test_clear_display() {
        let config = DisplayConfig {
            width: 128,
            height: 64,
            color_depth: 1,
            display_type: DisplayType::OLED,
        };
        let mut display = EmbeddedDisplay::new(config);
        display.clear();
        assert!(display.framebuffer.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_draw_rect() {
        let config = DisplayConfig {
            width: 128,
            height: 64,
            color_depth: 16,
            display_type: DisplayType::LCD,
        };
        let mut display = EmbeddedDisplay::new(config);
        assert!(display.draw_rect(10, 10, 20, 20, 0xFFFF).is_ok());
    }

    #[test]
    fn test_draw_line() {
        let config = DisplayConfig {
            width: 128,
            height: 64,
            color_depth: 16,
            display_type: DisplayType::LCD,
        };
        let mut display = EmbeddedDisplay::new(config);
        assert!(display.draw_line(0, 0, 50, 50, 0xFFFF).is_ok());
    }

    #[test]
    fn test_draw_circle() {
        let config = DisplayConfig {
            width: 128,
            height: 64,
            color_depth: 16,
            display_type: DisplayType::OLED,
        };
        let mut display = EmbeddedDisplay::new(config);
        assert!(display.draw_circle(64, 32, 10, 0xFFFF).is_ok());
    }

    #[test]
    fn test_draw_text() {
        let config = DisplayConfig {
            width: 128,
            height: 64,
            color_depth: 1,
            display_type: DisplayType::OLED,
        };
        let mut display = EmbeddedDisplay::new(config);
        assert!(display.draw_text(10, 10, "Hello", 1, 1).is_ok());
    }

    #[test]
    fn test_touch_controller() {
        let mut touch = TouchController::new();
        assert!(!touch.is_calibrated());
        touch.calibrate(0, 1024, 0, 768);
        assert!(touch.is_calibrated());
    }

    #[test]
    fn test_display_refresh() {
        let config = DisplayConfig {
            width: 128,
            height: 64,
            color_depth: 1,
            display_type: DisplayType::EInk,
        };
        let display = EmbeddedDisplay::new(config);
        assert!(display.refresh().is_ok());
    }
}
