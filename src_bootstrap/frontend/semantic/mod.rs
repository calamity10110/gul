//! Semantic analysis, type checking, and ownership validation

pub mod analyzer;
pub mod ownership;
pub mod traits;

// Re-export the main analysis function
pub use analyzer::{SemanticAnalyzer, SymbolTable, Symbol, analyze};
