use gul_http;
use log::info;

fn main() {
    gul_logging::init();
    info!("gul-http - Basic Example");

    // Mock HTTP Request
    let url = "https://api.gul-lang.org/v1/status";
    info!("GET {}", url);

    // In real app: let resp = client.get(url).send().await?;
    info!("Response: 200 OK {{ 'status': 'healthy' }}");
}
