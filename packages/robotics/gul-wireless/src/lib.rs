pub trait WirelessInterface {
    fn connect(&self, ssid: &str, pass: &str) -> Result<(), String>;
    fn scan(&self) -> Vec<String>;
}

pub struct WifiManager;

impl WirelessInterface for WifiManager {
    fn connect(&self, ssid: &str, _pass: &str) -> Result<(), String> {
        println!("MOCK WIFI: Connecting to '{}'...", ssid);
        Ok(())
    }

    fn scan(&self) -> Vec<String> {
        vec!["Home_Network".to_string(), "Guest".to_string()]
    }
}

pub struct EspNowManager;
impl EspNowManager {
    pub fn broadcast(&self, data: &[u8]) {
        println!("MOCK ESP-NOW: Broadcasting {} bytes", data.len());
    }
}
