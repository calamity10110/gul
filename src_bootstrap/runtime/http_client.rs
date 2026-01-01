#![allow(dead_code)]
// HTTP client for making web requests
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderName, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    default_timeout: Duration,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        HttpClient {
            client,
            default_timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        HttpClient {
            client,
            default_timeout: Duration::from_secs(timeout_secs),
        }
    }

    /// Make a GET request
    pub fn get(&self, url: &str) -> Result<HttpResponse, String> {
        self.request("GET", url, None, None)
    }

    /// Make a POST request with JSON body
    pub fn post(&self, url: &str, body: Option<&str>) -> Result<HttpResponse, String> {
        self.request("POST", url, body, None)
    }

    /// Make a PUT request with JSON body
    pub fn put(&self, url: &str, body: Option<&str>) -> Result<HttpResponse, String> {
        self.request("PUT", url, body, None)
    }

    /// Make a DELETE request
    pub fn delete(&self, url: &str) -> Result<HttpResponse, String> {
        self.request("DELETE", url, None, None)
    }

    /// Make a request with custom headers
    pub fn request_with_headers(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
        headers: HashMap<String, String>,
    ) -> Result<HttpResponse, String> {
        self.request(method, url, body, Some(headers))
    }

    fn request(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, String> {
        let mut request = match method.to_uppercase().as_str() {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            _ => return Err(format!("Unsupported HTTP method: {}", method)),
        };

        // Add custom headers
        if let Some(header_map) = headers {
            for (key, value) in header_map {
                if let (Ok(name), Ok(val)) = (
                    HeaderName::from_bytes(key.as_bytes()),
                    HeaderValue::from_str(&value),
                ) {
                    request = request.header(name, val);
                }
            }
        }

        // Add body for POST/PUT
        if let Some(body_str) = body {
            request = request
                .header("Content-Type", "application/json")
                .body(body_str.to_string());
        }

        // Execute request
        let response = request
            .send()
            .map_err(|e| format!("Request failed: {}", e))?;

        HttpResponse::from_response(response)
    }

    /// Make a request with retry logic
    pub fn request_with_retry(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
        max_retries: u32,
    ) -> Result<HttpResponse, String> {
        let mut attempts = 0;
        let mut last_error = String::new();

        while attempts < max_retries {
            match self.request(method, url, body, None) {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = e;
                    attempts += 1;
                    if attempts < max_retries {
                        std::thread::sleep(Duration::from_secs(1 << attempts)); // Exponential backoff
                    }
                }
            }
        }

        Err(format!(
            "Request failed after {} retries: {}",
            max_retries, last_error
        ))
    }
}

pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    fn from_response(response: Response) -> Result<Self, String> {
        let status = response.status().as_u16();

        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(val_str) = value.to_str() {
                headers.insert(key.to_string(), val_str.to_string());
            }
        }

        let body = response
            .text()
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }

    /// Parse response body as JSON
    pub fn json(&self) -> Result<Value, String> {
        serde_json::from_str(&self.body).map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Check if response was successful (2xx status code)
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    /// Get a specific header value
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new();
        assert_eq!(client.default_timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_http_client_with_timeout() {
        let client = HttpClient::with_timeout(60);
        assert_eq!(client.default_timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_http_response_is_success() {
        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: "OK".to_string(),
        };
        assert!(response.is_success());

        let error_response = HttpResponse {
            status: 404,
            headers: HashMap::new(),
            body: "Not Found".to_string(),
        };
        assert!(!error_response.is_success());
    }

    #[test]
    fn test_http_response_json_parsing() {
        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: r#"{"key": "value"}"#.to_string(),
        };

        let json = response.json().unwrap();
        assert_eq!(json["key"], "value");
    }

    #[test]
    fn test_http_response_header_access() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let response = HttpResponse {
            status: 200,
            headers,
            body: "{}".to_string(),
        };

        assert_eq!(
            response.header("content-type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(response.header("missing"), None);
    }
}
