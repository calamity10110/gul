#![cfg(target_os = "espidf")]
use esp_idf_sys::*;
use std::ffi::CString;
use std::ptr;

#[derive(Debug, Clone)]
pub struct CameraConfig {
    pub pin_pwdn: i32,
    pub pin_reset: i32,
    pub pin_xclk: i32,
    pub pin_siod: i32,
    pub pin_sioc: i32,
    pub pin_d7: i32,
    pub pin_d6: i32,
    pub pin_d5: i32,
    pub pin_d4: i32,
    pub pin_d3: i32,
    pub pin_d2: i32,
    pub pin_d1: i32,
    pub pin_d0: i32,
    pub pin_vsync: i32,
    pub pin_href: i32,
    pub pin_pclk: i32,
    pub xclk_freq_hz: i32,
    pub ledc_timer: ledc_timer_t,
    pub ledc_channel: ledc_channel_t,
    pub pixel_format: pixformat_t,
    pub frame_size: framesize_t,
    pub jpeg_quality: i32,
    pub fb_count: usize,
    pub fb_location: camera_fb_location_t,
    pub grab_mode: camera_grab_mode_t,
}

pub struct CameraDriver;

impl CameraDriver {
    pub fn new(config: CameraConfig) -> Result<Self, EspError> {
        let c_config = camera_config_t {
            pin_pwdn: config.pin_pwdn,
            pin_reset: config.pin_reset,
            pin_xclk: config.pin_xclk,
            pin_sscb_sda: config.pin_siod,
            pin_sscb_scl: config.pin_sioc,
            pin_d7: config.pin_d7,
            pin_d6: config.pin_d6,
            pin_d5: config.pin_d5,
            pin_d4: config.pin_d4,
            pin_d3: config.pin_d3,
            pin_d2: config.pin_d2,
            pin_d1: config.pin_d1,
            pin_d0: config.pin_d0,
            pin_vsync: config.pin_vsync,
            pin_href: config.pin_href,
            pin_pclk: config.pin_pclk,
            xclk_freq_hz: config.xclk_freq_hz,
            ledc_timer: config.ledc_timer,
            ledc_channel: config.ledc_channel,
            pixel_format: config.pixel_format,
            frame_size: config.frame_size,
            jpeg_quality: config.jpeg_quality,
            fb_count: config.fb_count,
            fb_location: config.fb_location,
            grab_mode: config.grab_mode,
            sccb_i2c_port: 0,
            __bindgen_anon_1: camera_config_t__bindgen_ty_1 {
                pin_sccb_sda: config.pin_siod,
            },
        };

        unsafe {
            let err = esp_camera_init(&c_config);
            if err != ESP_OK {
                return Err(EspError::from(err).unwrap());
            }
        }

        Ok(Self)
    }

    pub fn capture(&self) -> Option<Vec<u8>> {
        unsafe {
            let fb = esp_camera_fb_get();
            if fb.is_null() {
                return None;
            }

            // Copy data to Vec
            let data = std::slice::from_raw_parts((*fb).buf, (*fb).len);
            let vec_data = data.to_vec();

            // Return buffer to driver
            esp_camera_fb_return(fb);

            Some(vec_data)
        }
    }
}

// Default config for Waveshare ESP32-S3 AMOLED
impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            pin_pwdn: -1,
            pin_reset: -1,
            pin_xclk: 10,
            pin_siod: 40,
            pin_sioc: 39,
            pin_d7: 48,
            pin_d6: 11,
            pin_d5: 12,
            pin_d4: 14,
            pin_d3: 16,
            pin_d2: 18,
            pin_d1: 17,
            pin_d0: 15,
            pin_vsync: 38,
            pin_href: 47,
            pin_pclk: 13,
            xclk_freq_hz: 20000000,
            ledc_timer: ledc_timer_t_LEDC_TIMER_0,
            ledc_channel: ledc_channel_t_LEDC_CHANNEL_0,
            pixel_format: pixformat_t_PIXFORMAT_JPEG,
            frame_size: framesize_t_FRAMESIZE_VGA,
            jpeg_quality: 12,
            fb_count: 1,
            fb_location: camera_fb_location_t_CAMERA_FB_IN_PSRAM,
            grab_mode: camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY,
        }
    }
}
