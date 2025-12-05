// Example: HTTP Client Usage

use gul_http::Client;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("GUL HTTP Client Example\n");

    // Create client
    let client = Client::new()
        .with_base_url("https://api.github.com")
        .header("User-Agent", "GUL-HTTP/0.1.0");

    println!("Fetching GitHub API...");

    // GET request
    let response = client.get("/").await?;
    println!("Status: {}", response.status);
    println!("Response: {}", response.text()?);

    Ok(())
}
