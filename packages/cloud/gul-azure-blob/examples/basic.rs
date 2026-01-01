use gul_azure_blob::BlobClient;

fn main() {
    let client = BlobClient::new("myaccount", "mykey");
    client.upload("images", "logo.png", b"contents");
}
