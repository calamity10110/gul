// Ownership error types per GUL 101 Appendix E

/// Error codes for ownership violations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorCode {
    // Ownership Errors (E001-E005)
    E001, // Unused own<T> value
    E002, // Multiple ownership-moving edges
    E003, // Borrow after ownership moved
    E004, // Invalid take from non-owner
    E005, // Gives on non-copy type

    // Type Errors (E101-E103)
    E101, // Type mismatch
    E102, // Trait not satisfied
    E103, // Ambiguous type resolution

    // Graph Errors (E201-E203)
    E201, // Cycle without loop declaration
    E202, // Unreachable node
    E203, // Dangling graph output
}

impl ErrorCode {
    pub fn message(&self) -> &'static str {
        match self {
            ErrorCode::E001 => "Unused own<T> value - all owned values must be consumed",
            ErrorCode::E002 => "Multiple ownership-moving edges from same value",
            ErrorCode::E003 => "Cannot borrow after ownership has been moved",
            ErrorCode::E004 => "Cannot take ownership from non-owner",
            ErrorCode::E005 => "Cannot use 'gives' on non-copy type",
            ErrorCode::E101 => "Type mismatch",
            ErrorCode::E102 => "Required trait not satisfied",
            ErrorCode::E103 => "Ambiguous type resolution",
            ErrorCode::E201 => "Graph contains cycle without explicit loop declaration",
            ErrorCode::E202 => "Unreachable node in graph",
            ErrorCode::E203 => "Dangling output - node output not connected",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            ErrorCode::E001 => "E001",
            ErrorCode::E002 => "E002",
            ErrorCode::E003 => "E003",
            ErrorCode::E004 => "E004",
            ErrorCode::E005 => "E005",
            ErrorCode::E101 => "E101",
            ErrorCode::E102 => "E102",
            ErrorCode::E103 => "E103",
            ErrorCode::E201 => "E201",
            ErrorCode::E202 => "E202",
            ErrorCode::E203 => "E203",
        }
    }
}

/// Ownership error with context
#[derive(Debug, Clone)]
pub struct OwnershipError {
    pub code: ErrorCode,
    pub node_name: Option<String>,
    pub port_name: Option<String>,
    pub message: String,
    pub suggestion: Option<String>,
}

impl OwnershipError {
    pub fn new(code: ErrorCode) -> Self {
        OwnershipError {
            code,
            node_name: None,
            port_name: None,
            message: code.message().to_string(),
            suggestion: None,
        }
    }

    pub fn with_node(mut self, node: &str) -> Self {
        self.node_name = Some(node.to_string());
        self
    }

    pub fn with_port(mut self, port: &str) -> Self {
        self.port_name = Some(port.to_string());
        self
    }

    pub fn with_message(mut self, msg: &str) -> Self {
        self.message = msg.to_string();
        self
    }

    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }

    /// Format error for display
    pub fn format(&self) -> String {
        let mut result = format!("[{}] {}", self.code.code(), self.message);

        if let Some(node) = &self.node_name {
            result.push_str(&format!("\n  → Node: {}", node));
        }
        if let Some(port) = &self.port_name {
            result.push_str(&format!("\n  → Port: {}", port));
        }
        if let Some(suggestion) = &self.suggestion {
            result.push_str(&format!("\n  💡 Suggestion: {}", suggestion));
        }

        result
    }
}

impl std::fmt::Display for OwnershipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl std::error::Error for OwnershipError {}
