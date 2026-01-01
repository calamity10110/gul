pub struct BlobClient {
    account: String,
    key: String,
}

impl BlobClient {
    pub fn new(account: &str, key: &str) -> Self {
        Self {
            account: account.to_string(),
            key: key.to_string(),
        }
    }

    pub fn upload(&self, container: &str, blob_name: &str, _data: &[u8]) {
        println!("MOCK: Azure Upload to {}/{}", container, blob_name);
    }
}
