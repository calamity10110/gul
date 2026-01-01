use gul_logging;
use log::{info, warn};

fn main() {
    gul_logging::init();
    info!("System starting");
    warn!("Low disk space simulation");
}
