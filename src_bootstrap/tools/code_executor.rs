// Code Executor for GUL Course
// Safely executes student code and validates output

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct CodeExecutor {
    _timeout: Duration,
    _max_output_size: usize,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time: Duration,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub test_results: Vec<TestResult>,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub expected: String,
    pub actual: String,
    pub error_message: Option<String>,
}

impl CodeExecutor {
    pub fn new() -> Self {
        CodeExecutor {
            _timeout: Duration::from_secs(5),
            _max_output_size: 10_000, // 10KB max output
        }
    }

    /// Execute GUL code safely
    pub fn execute(&self, code: &str) -> ExecutionResult {
        let start_time = std::time::Instant::now();

        // Simulate execution (temporary until full interpreter is ready)
        let result = self.simulate_execution(code);

        let execution_time = start_time.elapsed();

        ExecutionResult {
            success: result.is_ok(),
            output: result.as_ref().unwrap_or(&String::new()).clone(),
            error: result.err(),
            execution_time,
        }
    }

    /// Simulate code execution (temporary)
    fn simulate_execution(&self, code: &str) -> Result<String, String> {
        let mut output = String::new();

        for line in code.lines() {
            let trimmed = line.trim();

            // Handle print statements
            if trimmed.starts_with("print(") && trimmed.ends_with(")") {
                let content = &trimmed[6..trimmed.len() - 1];
                let cleaned = content.trim_matches('"').trim_matches('\'');
                output.push_str(cleaned);
                output.push('\n');
            }
        }

        if output.is_empty() && !code.trim().is_empty() {
            return Ok("(no output)".to_string());
        }

        Ok(output.trim_end().to_string())
    }

    /// Validate code against test cases
    pub fn validate(&self, code: &str, test_cases: &[TestCase]) -> ValidationResult {
        let mut test_results = Vec::new();
        let mut passed_count = 0;

        for test_case in test_cases {
            let result = self.execute(code);

            let actual_output = result.output.trim();
            let expected_output = test_case.expected_output.trim();

            let passed = actual_output == expected_output;
            if passed {
                passed_count += 1;
            }

            test_results.push(TestResult {
                test_name: test_case.description.clone(),
                passed,
                expected: expected_output.to_string(),
                actual: actual_output.to_string(),
                error_message: result.error,
            });
        }

        let score = if test_cases.is_empty() {
            0.0
        } else {
            (passed_count as f32 / test_cases.len() as f32) * 100.0
        };

        ValidationResult {
            passed: passed_count == test_cases.len(),
            test_results,
            score,
        }
    }

    /// Check if code has syntax errors
    pub fn check_syntax(&self, code: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Check for unmatched quotes
            let quote_count = trimmed.matches('"').count();
            if quote_count % 2 != 0 {
                errors.push(format!("Line {}: Unmatched quotes", i + 1));
            }

            // Check for unmatched parentheses
            let open_paren = trimmed.matches('(').count();
            let close_paren = trimmed.matches(')').count();
            if open_paren != close_paren {
                errors.push(format!("Line {}: Unmatched parentheses", i + 1));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Provide helpful error messages
    pub fn explain_error(&self, error: &str) -> String {
        if error.contains("unmatched quotes") {
            return "💡 Tip: Strings need matching quotes. Use \"text\" or 'text'".to_string();
        }

        if error.contains("unmatched parentheses") {
            return "💡 Tip: Every ( needs a ). Check your print() statements!".to_string();
        }

        format!("Error: {}", error)
    }
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub description: String,
    pub input: String,
    pub expected_output: String,
}

impl Default for CodeExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_print() {
        let executor = CodeExecutor::new();
        let code = r#"main:
    print("Hello, World!")"#;

        let result = executor.execute(code);
        assert!(result.success);
        assert_eq!(result.output.trim(), "Hello, World!");
    }

    #[test]
    fn test_syntax_check_valid() {
        let executor = CodeExecutor::new();
        let code = r#"main:
    print("Hello")"#;

        assert!(executor.check_syntax(code).is_ok());
    }

    #[test]
    fn test_validation() {
        let executor = CodeExecutor::new();
        let code = r#"main:
    print("Hello")"#;

        let test_cases = vec![TestCase {
            description: "Should print Hello".to_string(),
            input: String::new(),
            expected_output: "Hello".to_string(),
        }];

        let result = executor.validate(code, &test_cases);
        assert!(result.passed);
        assert_eq!(result.score, 100.0);
    }
}
