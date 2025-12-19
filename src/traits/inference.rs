// Trait inference for automatic trait detection

use super::TraitRegistry;

/// Trait inference engine
pub struct TraitInference<'a> {
    registry: &'a TraitRegistry,
}

impl<'a> TraitInference<'a> {
    pub fn new(registry: &'a TraitRegistry) -> Self {
        TraitInference { registry }
    }

    /// Infer traits from type name
    pub fn infer_traits(&self, type_name: &str) -> Vec<String> {
        let mut traits = Vec::new();

        // Basic type inference
        match type_name {
            "int" | "float" | "str" | "bool" => {
                traits.push("serialize".to_string());
            }
            "list" | "dict" => {
                traits.push("serialize".to_string());
            }
            _ => {}
        }

        traits
    }
}
