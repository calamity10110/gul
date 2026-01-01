use gul_mongodb;
use log::info;

fn main() {
    gul_logging::init();
    info!("gul-mongodb - Basic Example");

    // Mock connection
    let uri = "mongodb://localhost:27017";
    info!("Connecting to MongoDB at {}...", uri);

    // In a real app: let client = Client::with_uri_str(uri).await?;
    info!("Connection established (mock). Ready for queries.");
}
