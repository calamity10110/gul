// GUL Trait System
// Defines semantic traits for data-flow nodes

pub mod builtin;
pub mod matcher;
pub mod inference;

use crate::ast::BuiltinTrait;
use std::collections::HashMap;

/// Trait registry
pub struct TraitRegistry {
    /// Registered traits
    traits: HashMap<String, TraitDefinition>,
    /// Trait inheritance relationships
    inheritance: HashMap<String, Vec<String>>,
}

/// Trait definition
#[derive(Debug, Clone)]
pub struct TraitDefinition {
    pub name: String,
    pub methods: Vec<String>,
    pub description: String,
}

impl TraitRegistry {
    pub fn new() -> Self {
        let mut registry = TraitRegistry {
            traits: HashMap::new(),
            inheritance: HashMap::new(),
        };
        
        // Register built-in traits
        registry.register_builtin_traits();
        registry
    }

    /// Register built-in traits
    fn register_builtin_traits(&mut self) {
        self.register_trait(TraitDefinition {
            name: "serialize".to_string(),
            methods: vec!["to_bytes".to_string(), "from_bytes".to_string()],
            description: "Can be serialized to/from bytes".to_string(),
        });

        self.register_trait(TraitDefinition {
            name: "trainable".to_string(),
            methods: vec!["train".to_string(), "predict".to_string()],
            description: "Can be trained (ML models)".to_string(),
        });

        self.register_trait(TraitDefinition {
            name: "stream".to_string(),
            methods: vec!["read".to_string(), "write".to_string()],
            description: "Streaming data source/sink".to_string(),
        });

        self.register_trait(TraitDefinition {
            name: "async".to_string(),
            methods: vec![],
            description: "Asynchronous execution".to_string(),
        });

        self.register_trait(TraitDefinition {
            name: "sync".to_string(),
            methods: vec![],
            description: "Synchronous execution".to_string(),
        });
    }

    /// Register a new trait
    pub fn register_trait(&mut self, def: TraitDefinition) {
        self.traits.insert(def.name.clone(), def);
    }

    /// Check if a trait exists
    pub fn has_trait(&self, name: &str) -> bool {
        self.traits.contains_key(name)
    }

    /// Get trait definition
    pub fn get_trait(&self, name: &str) -> Option<&TraitDefinition> {
        self.traits.get(name)
    }

    /// Add trait inheritance
    pub fn add_inheritance(&mut self, child: &str, parent: &str) {
        self.inheritance
            .entry(child.to_string())
            .or_insert_with(Vec::new)
            .push(parent.to_string());
    }

    /// Check if type implements trait (including inheritance)
    pub fn implements_trait(&self, type_traits: &[String], required: &str) -> bool {
        // Direct implementation
        if type_traits.contains(&required.to_string()) {
            return true;
        }

        // Check inheritance
        for trait_name in type_traits {
            if let Some(parents) = self.inheritance.get(trait_name) {
                if parents.contains(&required.to_string()) {
                    return true;
                }
            }
        }

        false
    }
}

impl Default for TraitRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_traits() {
        let registry = TraitRegistry::new();
        assert!(registry.has_trait("serialize"));
        assert!(registry.has_trait("trainable"));
        assert!(registry.has_trait("stream"));
        assert!(registry.has_trait("async"));
        assert!(registry.has_trait("sync"));
    }

    #[test]
    fn test_trait_implementation() {
        let registry = TraitRegistry::new();
        let type_traits = vec!["serialize".to_string()];
        assert!(registry.implements_trait(&type_traits, "serialize"));
        assert!(!registry.implements_trait(&type_traits, "trainable"));
    }
}
