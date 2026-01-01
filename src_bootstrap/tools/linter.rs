// Linter for Universal Language (simplified version)

use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum LintLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub level: LintLevel,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub rule: String,
    pub auto_fix: Option<String>,
}

impl std::fmt::Display for LintIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}] Line {}: {} ({})",
            self.level, self.line, self.message, self.rule
        )
    }
}

// Linter module - checks code for common issues
#[allow(dead_code)]
pub struct Linter {
    issues: Vec<LintIssue>,
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}

impl Linter {
    pub fn new() -> Self {
        Linter { issues: Vec::new() }
    }

    pub fn lint_file(&mut self, content: &str) -> Vec<LintIssue> {
        self.issues.clear();

        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // v3 syntax checks - deprecated main syntax
            if trimmed.starts_with("mn main()") || trimmed.starts_with("mn main ") {
                self.add_issue(
                    LintLevel::Warning,
                    "Deprecated syntax: use 'mn:' instead of 'mn main():'".to_string(),
                    line_num + 1,
                    0,
                    "deprecated-main-syntax",
                    Some(
                        trimmed
                            .replace("mn main():", "mn:")
                            .replace("mn main:", "mn:"),
                    ),
                );
            }

            if trimmed.starts_with("main():") || trimmed == "main():" {
                self.add_issue(
                    LintLevel::Warning,
                    "Deprecated syntax: use 'mn:' instead of 'main():'".to_string(),
                    line_num + 1,
                    0,
                    "deprecated-main-syntax",
                    Some("mn:".to_string()),
                );
            }

            // v3 syntax checks - deprecated keywords
            if trimmed.starts_with("def ") && !trimmed.contains("(") {
                self.add_issue(
                    LintLevel::Warning,
                    "Deprecated: use 'let' for immutable or 'var' for mutable variables"
                        .to_string(),
                    line_num + 1,
                    0,
                    "deprecated-def-keyword",
                    Some(trimmed.replacen("def ", "let ", 1)),
                );
            }

            if trimmed.starts_with("imp ") {
                self.add_issue(
                    LintLevel::Info,
                    "Consider using '@imp' for v3 import syntax".to_string(),
                    line_num + 1,
                    0,
                    "deprecated-imp-keyword",
                    Some(trimmed.replacen("imp ", "@imp ", 1)),
                );
            }

            if trimmed.starts_with("asy ") {
                self.add_issue(
                    LintLevel::Warning,
                    "Deprecated: use 'async' instead of 'asy'".to_string(),
                    line_num + 1,
                    0,
                    "deprecated-asy-keyword",
                    Some(trimmed.replacen("asy ", "async ", 1)),
                );
            }

            if trimmed.starts_with("cs ") {
                self.add_issue(
                    LintLevel::Info,
                    "Consider using '@python', '@rust', '@sql' blocks instead of 'cs'".to_string(),
                    line_num + 1,
                    0,
                    "deprecated-cs-keyword",
                    None,
                );
            }

            // v3.1 syntax checks - deprecated const/mut keywords
            if trimmed.starts_with("const ") {
                self.add_issue(
                    LintLevel::Warning,
                    "Deprecated: use 'let' instead of 'const'".to_string(),
                    line_num + 1,
                    0,
                    "deprecated-const-keyword",
                    Some(trimmed.replacen("const ", "let ", 1)),
                );
            }

            if trimmed.starts_with("mut ") && !trimmed.contains("&mut") {
                self.add_issue(
                    LintLevel::Warning,
                    "Deprecated: use 'var' instead of 'mut'".to_string(),
                    line_num + 1,
                    0,
                    "deprecated-mut-keyword",
                    Some(trimmed.replacen("mut ", "var ", 1)),
                );
            }

            // Check for naming conventions
            if trimmed.starts_with("def ")
                || trimmed.starts_with("let ")
                || trimmed.starts_with("var ")
            {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[1].trim_end_matches(':');
                    if !self.is_snake_case(name)
                        && !name.chars().all(|c| c.is_uppercase() || c == '_')
                    {
                        self.add_issue(
                            LintLevel::Warning,
                            format!("Variable '{}' should use snake_case", name),
                            line_num + 1,
                            0,
                            "naming-convention",
                            Some(self.to_snake_case(name)),
                        );
                    }
                }
            }

            // Check for trailing whitespace
            if line.ends_with(' ') || line.ends_with('\t') {
                self.add_issue(
                    LintLevel::Info,
                    "Line has trailing whitespace".to_string(),
                    line_num + 1,
                    line.len(),
                    "trailing-whitespace",
                    Some(line.trim_end().to_string()),
                );
            }
        }

        self.issues.clone()
    }

    fn is_snake_case(&self, name: &str) -> bool {
        name.chars()
            .all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
    }

    fn to_snake_case(&self, name: &str) -> String {
        let mut result = String::new();
        for (i, c) in name.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else {
                result.push(c);
            }
        }
        result
    }

    fn add_issue(
        &mut self,
        level: LintLevel,
        message: String,
        line: usize,
        column: usize,
        rule: &str,
        auto_fix: Option<String>,
    ) {
        self.issues.push(LintIssue {
            level,
            message,
            line,
            column,
            rule: rule.to_string(),
            auto_fix,
        });
    }

    pub fn get_issues(&self) -> &[LintIssue] {
        &self.issues
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.level == LintLevel::Error)
    }
}

// Public API function for CLI
pub fn lint_file(file: &Path, fix: bool) -> Result<Vec<String>, String> {
    let source =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut linter = Linter::new();
    let issues = linter.lint_file(&source);

    if fix && !issues.is_empty() {
        // Apply auto-fixes if available
        let mut fixed_source = source.clone();
        for issue in &issues {
            if let Some(fix_text) = &issue.auto_fix {
                // Simple fix application (would need more sophisticated logic for real use)
                fixed_source = fixed_source
                    .replace(source.lines().nth(issue.line - 1).unwrap_or(""), fix_text);
            }
        }
        std::fs::write(file, fixed_source).map_err(|e| format!("Failed to write file: {}", e))?;
    }

    Ok(issues.iter().map(|i| i.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_creation() {
        let linter = Linter::new();
        assert_eq!(linter.issues.len(), 0);
    }

    #[test]
    fn test_naming_convention() {
        let mut linter = Linter::new();
        let code = "def BadName = 42";

        let issues = linter.lint_file(code);
        assert!(issues.iter().any(|i| i.rule == "naming-convention"));
    }

    #[test]
    fn test_snake_case_check() {
        let linter = Linter::new();
        assert!(linter.is_snake_case("valid_name"));
        assert!(linter.is_snake_case("another_valid_123"));
        assert!(!linter.is_snake_case("InvalidName"));
        assert!(!linter.is_snake_case("badName"));
    }

    #[test]
    fn test_to_snake_case() {
        let linter = Linter::new();
        assert_eq!(linter.to_snake_case("CamelCase"), "camel_case");
        assert_eq!(linter.to_snake_case("HTTPServer"), "h_t_t_p_server");
    }

    #[test]
    fn test_trailing_whitespace() {
        let mut linter = Linter::new();
        let code = "def x = 10  \ndef y = 20";

        let issues = linter.lint_file(code);
        assert!(issues.iter().any(|i| i.rule == "trailing-whitespace"));
    }
}
