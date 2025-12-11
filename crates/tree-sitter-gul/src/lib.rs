/// Tree-sitter grammar for GUL (Universal Language)
/// Supports GUL v2.1 syntax with bracket equivalence and UI components
use tree_sitter::Language;

extern "C" {
    fn tree_sitter_gul() -> Language;
}

/// Get the GUL language grammar
pub fn language() -> Language {
    unsafe { tree_sitter_gul() }
}

/// Language name
pub const LANGUAGE_NAME: &str = "gul";

/// Highlight query for syntax highlighting
pub const HIGHLIGHTS_QUERY: &str = include_str!("../queries/highlights.scm");

/// Injections query for embedded languages
pub const INJECTIONS_QUERY: &str = include_str!("../queries/injections.scm");

/// Locals query for local variable tracking
pub const LOCALS_QUERY: &str = include_str!("../queries/locals.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_highlight_query_exists() {
        // Will fail at compile time if query files don't exist
        // This is just a placeholder until grammar is built
    }
}
