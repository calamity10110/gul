// Built-in trait implementations

use crate::ast::BuiltinTrait;

/// Built-in trait behavior
pub trait TraitBehavior {
    fn name(&self) -> &'static str;
    fn is_compatible_with(&self, other: &BuiltinTrait) -> bool;
}

impl TraitBehavior for BuiltinTrait {
    fn name(&self) -> &'static str {
        match self {
            BuiltinTrait::Serialize => "serialize",
            BuiltinTrait::Trainable => "trainable",
            BuiltinTrait::Stream => "stream",
            BuiltinTrait::Async => "async",
            BuiltinTrait::Sync => "sync",
        }
    }

    fn is_compatible_with(&self, other: &BuiltinTrait) -> bool {
        // Same trait is compatible
        if self == other {
            return true;
        }

        // async and sync are mutually exclusive
        if matches!((self, other), 
            (BuiltinTrait::Async, BuiltinTrait::Sync) |
            (BuiltinTrait::Sync, BuiltinTrait::Async)) {
            return false;
        }

        true
    }
}
