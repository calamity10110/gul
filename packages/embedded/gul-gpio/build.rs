fn main() {
    // Check if the target is espidf
    match std::env::var("CARGO_CFG_TARGET_OS") {
        Ok(os) if os == "espidf" => {
            embuild::espidf::sysenv::output();
        }
        _ => {}
    }
}
