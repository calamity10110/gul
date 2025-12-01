#![allow(dead_code)]
// Secrets manager - handles encrypted credentials

use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::collections::HashMap;
use std::fs;

pub struct SecretsManager {
    secrets: HashMap<String, String>,
    encrypted: bool,
}

impl SecretsManager {
    pub fn new() -> Self {
        SecretsManager {
            secrets: HashMap::new(),
            encrypted: false,
        }
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                self.secrets
                    .insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.secrets.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.secrets.insert(key, value);
    }

    pub fn validate_no_leakage(&self, code: &str) -> Vec<String> {
        let mut leaks = Vec::new();

        for (key, value) in &self.secrets {
            if code.contains(value) {
                leaks.push(format!("Secret '{}' may be leaked in code", key));
            }
        }

        leaks
    }

    pub fn encrypt(&self, data: &str) -> String {
        // Simple XOR-based encryption for demonstration
        // In production, use proper encryption like AES-256
        let key = b"gul_secret_key_32bytes_long!!!!";
        let encrypted: Vec<u8> = data
            .bytes()
            .enumerate()
            .map(|(i, b)| b ^ key[i % key.len()])
            .collect();
        STANDARD.encode(&encrypted)
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String, String> {
        // Decrypt using same XOR key
        let key = b"gul_secret_key_32bytes_long!!!!";
        let decoded = STANDARD
            .decode(encrypted)
            .map_err(|e| format!("Failed to decode base64: {}", e))?;

        let decrypted: Vec<u8> = decoded
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ key[i % key.len()])
            .collect();

        String::from_utf8(decrypted).map_err(|e| format!("Failed to convert to UTF-8: {}", e))
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let mut content = String::new();
        content.push_str("# Secret configuration file\n");
        content.push_str("# DO NOT COMMIT THIS FILE\n\n");

        for (key, value) in &self.secrets {
            content.push_str(&format!("{}={}\n", key, value));
        }

        fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn generate_annotations(&self) -> String {
        let mut annotations = String::new();
        annotations.push_str("# Secret annotations (safe to publish)\n\n");

        for key in self.secrets.keys() {
            annotations.push_str(&format!("{}=<redacted>\n", key));
        }

        annotations
    }
}

impl Default for SecretsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_secrets_manager() {
        let mut manager = SecretsManager::new();

        manager.set("API_KEY".to_string(), "secret123".to_string());
        assert_eq!(manager.get("API_KEY"), Some(&"secret123".to_string()));
    }

    #[test]
    fn test_leakage_detection() {
        let mut manager = SecretsManager::new();
        manager.set("API_KEY".to_string(), "secret123".to_string());

        let code = "let key = \"secret123\"";
        let leaks = manager.validate_no_leakage(code);

        assert_eq!(leaks.len(), 1);
        assert!(leaks[0].contains("API_KEY"));
    }

    #[test]
    fn test_save_and_load() {
        let temp_file = "/tmp/test_secrets.scrt";

        let mut manager = SecretsManager::new();
        manager.set("KEY1".to_string(), "value1".to_string());
        manager.set("KEY2".to_string(), "value2".to_string());

        manager.save_to_file(temp_file).unwrap();

        let mut loaded = SecretsManager::new();
        loaded.load_from_file(temp_file).unwrap();

        assert_eq!(loaded.get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(loaded.get("KEY2"), Some(&"value2".to_string()));

        // Cleanup
        let _ = fs::remove_file(temp_file);
    }

    #[test]
    fn test_generate_annotations() {
        let mut manager = SecretsManager::new();
        manager.set("API_KEY".to_string(), "secret".to_string());

        let annotations = manager.generate_annotations();
        assert!(annotations.contains("API_KEY=<redacted>"));
        assert!(!annotations.contains("secret"));
    }
}
