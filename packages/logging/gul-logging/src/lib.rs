use std::io::Write;

pub fn init() {
    env_logger::builder()
        .format(|buf, record| {
            let timestamp = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S");
            writeln!(
                buf,
                "{}",
                serde_json::json!({
                    "timestamp": timestamp.to_string(),
                    "level": record.level().to_string(),
                    "target": record.target(),
                    "message": record.args().to_string(),
                })
            )
        })
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .ok(); // Ignore multiple init errors
}
