// GUL 101 Ownership Checker
// Validates ownership rules and mandatory consumption

pub mod checker;
pub mod errors;

pub use checker::OwnershipChecker;
pub use errors::{ErrorCode, OwnershipError};
