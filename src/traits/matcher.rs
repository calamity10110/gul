// Trait matcher for type compatibility

use super::TraitRegistry;

/// Trait matcher for checking type compatibility
pub struct TraitMatcher<'a> {
    registry: &'a TraitRegistry,
}

impl<'a> TraitMatcher<'a> {
    pub fn new(registry: &'a TraitRegistry) -> Self {
        TraitMatcher { registry }
    }

    /// Check if source traits satisfy target requirements
    pub fn satisfies(&self, source_traits: &[String], target_traits: &[String]) -> bool {
        for required in target_traits {
            if !self.registry.implements_trait(source_traits, required) {
                return false;
            }
        }
        true
    }

    /// Check covariant trait compatibility
    pub fn is_covariant(&self, source: &[String], target: &[String]) -> bool {
        // Source must have AT LEAST all traits of target
        self.satisfies(source, target)
    }
}
