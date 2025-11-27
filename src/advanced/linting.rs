// Advanced linting for Universal Language
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
    Performance,
    Security,
}

#[derive(Debug, Clone)]
pub struct AdvancedLintIssue {
    pub severity: LintSeverity,
    pub category: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
    pub auto_fixable: bool,
}

pub struct AdvancedLinter {
    issues: Vec<AdvancedLintIssue>,
    rules: HashMap<String, bool>,
}

impl Default for AdvancedLinter {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedLinter {
    pub fn new() -> Self {
        let mut linter = AdvancedLinter {
            issues: Vec::new(),
            rules: HashMap::new(),
        };

        // Enable all rules by default
        linter.enable_rule("performance_loop_invariant".to_string());
        linter.enable_rule("performance_unnecessary_clone".to_string());
        linter.enable_rule("security_sql_injection".to_string());
        linter.enable_rule("security_xss".to_string());
        linter.enable_rule("architecture_circular_dependency".to_string());
        linter.enable_rule("code_smell_long_function".to_string());
        linter.enable_rule("code_smell_too_many_parameters".to_string());

        linter
    }

    pub fn enable_rule(&mut self, rule: String) {
        self.rules.insert(rule, true);
    }

    pub fn disable_rule(&mut self, rule: String) {
        self.rules.insert(rule, false);
    }

    pub fn is_rule_enabled(&self, rule: &str) -> bool {
        self.rules.get(rule).copied().unwrap_or(false)
    }

    pub fn lint_code(&mut self, source: &str) {
        self.issues.clear();

        let lines: Vec<&str> = source.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            self.check_performance_issues(line, line_num + 1);
            self.check_security_issues(line, line_num + 1);
            self.check_code_smells(line, line_num + 1);
        }
    }

    fn check_performance_issues(&mut self, line: &str, line_num: usize) {
        // Check for loop invariant code motion
        if self.is_rule_enabled("performance_loop_invariant") {
            if line.contains("loop") && line.contains("=") {
                self.issues.push(AdvancedLintIssue {
                    severity: LintSeverity::Performance,
                    category: "performance".to_string(),
                    message: "Consider moving loop-invariant computation outside the loop"
                        .to_string(),
                    line: line_num,
                    column: 0,
                    suggestion: Some("Move constant expressions outside the loop".to_string()),
                    auto_fixable: false,
                });
            }
        }

        // Check for unnecessary clones
        if self.is_rule_enabled("performance_unnecessary_clone") {
            if line.contains(".clone()") && line.contains("&") {
                self.issues.push(AdvancedLintIssue {
                    severity: LintSeverity::Performance,
                    category: "performance".to_string(),
                    message: "Unnecessary clone detected, consider using a reference".to_string(),
                    line: line_num,
                    column: 0,
                    suggestion: Some("Use a reference instead of cloning".to_string()),
                    auto_fixable: true,
                });
            }
        }
    }

    fn check_security_issues(&mut self, line: &str, line_num: usize) {
        // Check for SQL injection vulnerabilities
        if self.is_rule_enabled("security_sql_injection") {
            if line.contains("execute") && line.contains("+") && line.contains("\"") {
                self.issues.push(AdvancedLintIssue {
                    severity: LintSeverity::Security,
                    category: "security".to_string(),
                    message: "Potential SQL injection vulnerability detected".to_string(),
                    line: line_num,
                    column: 0,
                    suggestion: Some(
                        "Use parameterized queries instead of string concatenation".to_string(),
                    ),
                    auto_fixable: false,
                });
            }
        }

        // Check for XSS vulnerabilities
        if self.is_rule_enabled("security_xss") {
            if line.contains("innerHTML") || line.contains("dangerouslySetInnerHTML") {
                self.issues.push(AdvancedLintIssue {
                    severity: LintSeverity::Security,
                    category: "security".to_string(),
                    message: "Potential XSS vulnerability detected".to_string(),
                    line: line_num,
                    column: 0,
                    suggestion: Some("Sanitize user input before rendering".to_string()),
                    auto_fixable: false,
                });
            }
        }
    }

    fn check_code_smells(&mut self, line: &str, line_num: usize) {
        // Check for long functions
        if self.is_rule_enabled("code_smell_long_function") {
            if line.contains("fn ") && line.len() > 100 {
                self.issues.push(AdvancedLintIssue {
                    severity: LintSeverity::Warning,
                    category: "code_smell".to_string(),
                    message: "Function definition is too long, consider refactoring".to_string(),
                    line: line_num,
                    column: 0,
                    suggestion: Some("Break down into smaller functions".to_string()),
                    auto_fixable: false,
                });
            }
        }

        // Check for too many parameters
        if self.is_rule_enabled("code_smell_too_many_parameters") {
            let param_count = line.matches(',').count();
            if line.contains("fn ") && param_count > 5 {
                self.issues.push(AdvancedLintIssue {
                    severity: LintSeverity::Warning,
                    category: "code_smell".to_string(),
                    message: format!(
                        "Function has {} parameters, consider using a struct",
                        param_count + 1
                    ),
                    line: line_num,
                    column: 0,
                    suggestion: Some("Group related parameters into a struct".to_string()),
                    auto_fixable: false,
                });
            }
        }
    }

    pub fn get_issues(&self) -> &[AdvancedLintIssue] {
        &self.issues
    }

    pub fn get_issues_by_severity(&self, severity: LintSeverity) -> Vec<&AdvancedLintIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == severity)
            .collect()
    }

    pub fn get_issues_by_category(&self, category: &str) -> Vec<&AdvancedLintIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.category == category)
            .collect()
    }

    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == LintSeverity::Error)
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::from("Advanced Lint Report\n");
        report.push_str("====================\n\n");

        let errors = self.get_issues_by_severity(LintSeverity::Error);
        let warnings = self.get_issues_by_severity(LintSeverity::Warning);
        let performance = self.get_issues_by_severity(LintSeverity::Performance);
        let security = self.get_issues_by_severity(LintSeverity::Security);

        report.push_str(&format!("Errors: {}\n", errors.len()));
        report.push_str(&format!("Warnings: {}\n", warnings.len()));
        report.push_str(&format!("Performance: {}\n", performance.len()));
        report.push_str(&format!("Security: {}\n\n", security.len()));

        for issue in &self.issues {
            report.push_str(&format!(
                "[{:?}] Line {}: {}\n",
                issue.severity, issue.line, issue.message
            ));
            if let Some(suggestion) = &issue.suggestion {
                report.push_str(&format!("  Suggestion: {}\n", suggestion));
            }
            report.push('\n');
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_linter_creation() {
        let linter = AdvancedLinter::new();
        assert_eq!(linter.issues.len(), 0);
    }

    #[test]
    fn test_enable_disable_rule() {
        let mut linter = AdvancedLinter::new();

        linter.enable_rule("test_rule".to_string());
        assert!(linter.is_rule_enabled("test_rule"));

        linter.disable_rule("test_rule".to_string());
        assert!(!linter.is_rule_enabled("test_rule"));
    }

    #[test]
    fn test_performance_lint() {
        let mut linter = AdvancedLinter::new();
        let code = "loop { x = expensive_function(); }";

        linter.lint_code(code);

        let performance_issues = linter.get_issues_by_severity(LintSeverity::Performance);
        assert!(performance_issues.len() > 0);
    }

    #[test]
    fn test_security_sql_injection() {
        let mut linter = AdvancedLinter::new();
        let code = r#"execute("SELECT * FROM users WHERE id = " + user_input)"#;

        linter.lint_code(code);

        let security_issues = linter.get_issues_by_severity(LintSeverity::Security);
        assert!(security_issues.len() > 0);
    }

    #[test]
    fn test_security_xss() {
        let mut linter = AdvancedLinter::new();
        let code = "element.innerHTML = user_input";

        linter.lint_code(code);

        let security_issues = linter.get_issues_by_severity(LintSeverity::Security);
        assert!(security_issues.len() > 0);
    }

    #[test]
    fn test_code_smell_too_many_params() {
        let mut linter = AdvancedLinter::new();
        let code = "fn test(a, b, c, d, e, f, g) { }";

        linter.lint_code(code);

        let warnings = linter.get_issues_by_severity(LintSeverity::Warning);
        assert!(warnings.len() > 0);
    }

    #[test]
    fn test_get_issues_by_category() {
        let mut linter = AdvancedLinter::new();
        let code = r#"execute("SELECT * FROM users WHERE id = " + user_input)"#;

        linter.lint_code(code);

        let security_issues = linter.get_issues_by_category("security");
        assert!(security_issues.len() > 0);
    }

    #[test]
    fn test_has_errors() {
        let linter = AdvancedLinter::new();
        assert!(!linter.has_errors());
    }

    #[test]
    fn test_generate_report() {
        let mut linter = AdvancedLinter::new();
        let code = "loop { x = expensive_function(); }";

        linter.lint_code(code);

        let report = linter.generate_report();
        assert!(report.contains("Advanced Lint Report"));
        assert!(report.contains("Performance:"));
    }
}
