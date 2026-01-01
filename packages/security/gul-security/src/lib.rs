pub mod input_validation {
    use regex::Regex;

    pub fn sanitize(input: &str) -> String {
        input.replace("<", "&lt;").replace(">", "&gt;")
    }

    pub fn is_email(email: &str) -> bool {
        // Simple regex
        email.contains("@")
    }
}

pub mod rate_limit {
    pub struct RateLimiter;
}

pub mod secrets {
    pub struct SecretManager;
}

pub mod headers {
    pub fn get_security_headers() -> Vec<(String, String)> {
        vec![
            ("X-Frame-Options".to_string(), "DENY".to_string()),
            ("X-XSS-Protection".to_string(), "1; mode=block".to_string()),
        ]
    }
}
