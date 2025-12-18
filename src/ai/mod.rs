use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Supported AI providers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Google,
    Local,
    Custom(String),
}

impl fmt::Display for AIProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIProvider::OpenAI => write!(f, "openai"),
            AIProvider::Anthropic => write!(f, "anthropic"),
            AIProvider::Google => write!(f, "google"),
            AIProvider::Local => write!(f, "local"),
            AIProvider::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl FromStr for AIProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(AIProvider::OpenAI),
            "anthropic" => Ok(AIProvider::Anthropic),
            "google" => Ok(AIProvider::Google),
            "local" => Ok(AIProvider::Local),
            _ => Ok(AIProvider::Custom(s.to_string())),
        }
    }
}

/// AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub provider: AIProvider,
    pub model: String,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub temperature: f32,
    pub max_tokens: usize,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            provider: AIProvider::Local,
            model: "default".to_string(),
            api_key: None,
            endpoint: None,
            temperature: 0.7,
            max_tokens: 2048,
        }
    }
}

impl AIConfig {
    /// Create a new AI configuration
    pub fn new(provider: AIProvider, model: String) -> Self {
        Self {
            provider,
            model,
            ..Default::default()
        }
    }

    /// Set API key
    pub fn with_api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }

    /// Set custom endpoint
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, tokens: usize) -> Self {
        self.max_tokens = tokens;
        self
    }

    /// Load from environment variables
    pub fn from_env() -> Self {
        let provider = std::env::var("GUL_AI_PROVIDER")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(AIProvider::Local);

        let model = std::env::var("GUL_AI_MODEL").unwrap_or_else(|_| "default".to_string());

        let api_key = std::env::var("GUL_AI_API_KEY").ok();
        let endpoint = std::env::var("GUL_AI_ENDPOINT").ok();

        Self {
            provider,
            model,
            api_key,
            endpoint,
            temperature: 0.7,
            max_tokens: 2048,
        }
    }
}

/// AI Manager for handling AI operations
pub struct AIManager {
    config: AIConfig,
    context: HashMap<String, String>,
}

impl AIManager {
    /// Create a new AI manager with the given configuration
    pub fn new(config: AIConfig) -> Self {
        Self {
            config,
            context: HashMap::new(),
        }
    }

    /// Create from environment variables
    pub fn from_env() -> Self {
        Self::new(AIConfig::from_env())
    }

    /// Get current configuration
    pub fn config(&self) -> &AIConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: AIConfig) {
        self.config = config;
    }

    /// Set provider
    pub fn set_provider(&mut self, provider: AIProvider) {
        self.config.provider = provider;
    }

    /// Set model
    pub fn set_model(&mut self, model: String) {
        self.config.model = model;
    }

    /// Set API key
    pub fn set_api_key(&mut self, key: String) {
        self.config.api_key = Some(key);
    }

    /// Add context for AI operations
    pub fn add_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }

    /// Get context
    pub fn get_context(&self, key: &str) -> Option<&String> {
        self.context.get(key)
    }

    /// Clear context
    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    /// Check if AI is available
    pub fn is_available(&self) -> bool {
        match &self.config.provider {
            AIProvider::Local => true,
            AIProvider::OpenAI | AIProvider::Anthropic | AIProvider::Google => {
                self.config.api_key.is_some()
            }
            AIProvider::Custom(_) => self.config.endpoint.is_some(),
        }
    }

    /// Get provider status
    pub fn status(&self) -> String {
        format!(
            "Provider: {}, Model: {}, Available: {}",
            self.config.provider,
            self.config.model,
            self.is_available()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_provider_from_str() {
        assert_eq!("openai".parse::<AIProvider>(), Ok(AIProvider::OpenAI));
        assert_eq!("anthropic".parse::<AIProvider>(), Ok(AIProvider::Anthropic));
        assert_eq!("google".parse::<AIProvider>(), Ok(AIProvider::Google));
        assert_eq!("local".parse::<AIProvider>(), Ok(AIProvider::Local));
    }

    #[test]
    fn test_ai_config_default() {
        let config = AIConfig::default();
        assert_eq!(config.provider, AIProvider::Local);
        assert_eq!(config.model, "default");
        assert_eq!(config.temperature, 0.7);
    }

    #[test]
    fn test_ai_config_builder() {
        let config = AIConfig::new(AIProvider::OpenAI, "gpt-4".to_string())
            .with_api_key("test-key".to_string())
            .with_temperature(0.5)
            .with_max_tokens(4096);

        assert_eq!(config.provider, AIProvider::OpenAI);
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.api_key, Some("test-key".to_string()));
        assert_eq!(config.temperature, 0.5);
        assert_eq!(config.max_tokens, 4096);
    }

    #[test]
    fn test_ai_manager_creation() {
        let config = AIConfig::default();
        let manager = AIManager::new(config);
        assert_eq!(manager.config().provider, AIProvider::Local);
    }

    #[test]
    fn test_ai_manager_set_provider() {
        let mut manager = AIManager::from_env();
        manager.set_provider(AIProvider::OpenAI);
        assert_eq!(manager.config().provider, AIProvider::OpenAI);
    }

    #[test]
    fn test_ai_manager_context() {
        let mut manager = AIManager::from_env();
        manager.add_context("language".to_string(), "rust".to_string());
        assert_eq!(manager.get_context("language"), Some(&"rust".to_string()));

        manager.clear_context();
        assert_eq!(manager.get_context("language"), None);
    }

    #[test]
    fn test_ai_availability() {
        let config = AIConfig::default();
        let manager = AIManager::new(config);
        assert!(manager.is_available()); // Local is always available

        let mut manager = AIManager::new(AIConfig::new(AIProvider::OpenAI, "gpt-4".to_string()));
        assert!(!manager.is_available()); // No API key

        manager.set_api_key("test-key".to_string());
        assert!(manager.is_available()); // Has API key now
    }
}
