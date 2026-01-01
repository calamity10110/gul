use gul_wireless::{WifiManager, WirelessInterface};

fn main() {
    let wifi = WifiManager;
    wifi.connect("MyAP", "password").unwrap();
}
