// Contract definitions and validation
// Validates port contracts and type compatibility

use crate::ast::{PortContract, TypeAnnotation};

/// Contract validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Contract validator
pub struct ContractValidator {
    /// Registered traits
    traits: Vec<String>,
}

impl ContractValidator {
    pub fn new() -> Self {
        ContractValidator {
            traits: vec![
                "serialize".to_string(),
                "trainable".to_string(),
                "stream".to_string(),
                "async".to_string(),
                "sync".to_string(),
            ],
        }
    }

    /// Check if two types are compatible
    pub fn types_compatible(&self, output: &TypeAnnotation, input: &TypeAnnotation) -> bool {
        // Exact match
        if output.base_type == input.base_type {
            return true;
        }

        // Covariant types allowed
        if self.is_covariant(output, input) {
            return true;
        }

        // Trait-based compatibility
        if self.traits_compatible(output, input) {
            return true;
        }

        false
    }

    /// Check if output type is covariant to input type
    fn is_covariant(&self, output: &TypeAnnotation, input: &TypeAnnotation) -> bool {
        // int -> float allowed
        if output.base_type == "int" && input.base_type == "float" {
            return true;
        }

        // any accepts everything
        if input.base_type == "any" {
            return true;
        }

        false
    }

    /// Check if traits are compatible
    fn traits_compatible(&self, output: &TypeAnnotation, input: &TypeAnnotation) -> bool {
        // Output must have all traits required by input
        for trait_name in &input.traits {
            if !output.traits.contains(trait_name) {
                return false;
            }
        }
        true
    }

    /// Check if a trait is registered
    pub fn is_valid_trait(&self, trait_name: &str) -> bool {
        self.traits.contains(&trait_name.to_string())
    }

    /// Get all registered traits
    pub fn get_traits(&self) -> &[String] {
        &self.traits
    }

    /// Validate port contract satisfaction
    pub fn validate_connection(
        &self,
        source: &PortContract,
        target: &PortContract,
    ) -> ValidationResult {
        let mut result = ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Check type compatibility
        if !self.types_compatible(&source.port_type, &target.port_type) {
            result.valid = false;
            result.errors.push(format!(
                "Type mismatch: {} -> {}",
                source.port_type.base_type, target.port_type.base_type
            ));
        }

        result
    }
}

impl Default for ContractValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_type_match() {
        let validator = ContractValidator::new();
        let out_type = TypeAnnotation::new("int");
        let in_type = TypeAnnotation::new("int");
        assert!(validator.types_compatible(&out_type, &in_type));
    }

    #[test]
    fn test_covariant_type() {
        let validator = ContractValidator::new();
        let out_type = TypeAnnotation::new("int");
        let in_type = TypeAnnotation::new("float");
        assert!(validator.types_compatible(&out_type, &in_type));
    }

    #[test]
    fn test_any_accepts_all() {
        let validator = ContractValidator::new();
        let out_type = TypeAnnotation::new("str");
        let in_type = TypeAnnotation::new("any");
        assert!(validator.types_compatible(&out_type, &in_type));
    }
}
