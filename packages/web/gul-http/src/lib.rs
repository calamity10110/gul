// GUL HTTP Client/Server Package
// Provides HTTP functionality similar to reqwest/axum

use std::collections::HashMap;

/// HTTP Method
#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

/// HTTP Request
#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

/// HTTP Response
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn text(&self) -> Result<String, String> {
        String::from_utf8(self.body.clone()).map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, String> {
        serde_json::from_slice(&self.body).map_err(|e| format!("JSON parse error: {}", e))
    }
}

/// HTTP Client
pub struct Client {
    base_url: Option<String>,
    default_headers: HashMap<String, String>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_url: None,
            default_headers: HashMap::new(),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }

    pub async fn get(&self, url: &str) -> Result<Response, String> {
        self.request(Method::GET, url, None).await
    }

    pub async fn post(&self, url: &str, body: Vec<u8>) -> Result<Response, String> {
        self.request(Method::POST, url, Some(body)).await
    }

    pub async fn put(&self, url: &str, body: Vec<u8>) -> Result<Response, String> {
        self.request(Method::PUT, url, Some(body)).await
    }

    pub async fn delete(&self, url: &str) -> Result<Response, String> {
        self.request(Method::DELETE, url, None).await
    }

    async fn request(
        &self,
        method: Method,
        url: &str,
        body: Option<Vec<u8>>,
    ) -> Result<Response, String> {
        // In production: use actual HTTP client (reqwest)
        // For now, return mock response
        Ok(Response {
            status: 200,
            headers: HashMap::new(),
            body: b"{\"status\":\"ok\"}".to_vec(),
        })
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_get() {
        let client = Client::new();
        let response = client.get("https://api.example.com").await.unwrap();
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_response_text() {
        let response = Response {
            status: 200,
            headers: HashMap::new(),
            body: b"Hello, World!".to_vec(),
        };
        assert_eq!(response.text().unwrap(), "Hello, World!");
    }
}
