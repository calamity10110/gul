// GUL Test - Testing Framework

/// Test Result
pub type TestResult = Result<(), String>;

/// Test Case
pub struct TestCase {
    pub name: String,
    pub func: Box<dyn Fn() -> TestResult>,
}

/// Test Suite
pub struct TestSuite {
    name: String,
    tests: Vec<TestCase>,
}

impl TestSuite {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tests: Vec::new(),
        }
    }

    pub fn add_test<F>(&mut self, name: impl Into<String>, func: F)
    where
        F: Fn() -> TestResult + 'static,
    {
        self.tests.push(TestCase {
            name: name.into(),
            func: Box::new(func),
        });
    }

    pub fn run(&self) -> TestResults {
        let mut results = TestResults::new();

        for test in &self.tests {
            print!("Running test: {} ... ", test.name);
            match (test.func)() {
                Ok(()) => {
                    println!("✓ PASS");
                    results.passed += 1;
                }
                Err(e) => {
                    println!("✗ FAIL: {}", e);
                    results.failed += 1;
                    results.failures.push((test.name.clone(), e));
                }
            }
        }

        results
    }
}

/// Test Results
pub struct TestResults {
    pub passed: usize,
    pub failed: usize,
    pub failures: Vec<(String, String)>,
}

impl TestResults {
    fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            failures: Vec::new(),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Tests: {} passed, {} failed, {} total",
            self.passed,
            self.failed,
            self.passed + self.failed
        )
    }

    pub fn is_success(&self) -> bool {
        self.failed == 0
    }
}

/// Assertion Macros
#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            return Err(format!("assertion failed: `{:?}` != `{:?}`", $left, $right));
        }
    };
}

#[macro_export]
macro_rules! assert {
    ($cond:expr) => {
        if !$cond {
            return Err(format!("assertion failed: {}", stringify!($cond)));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suite_creation() {
        let suite = TestSuite::new("test_suite");
        assert_eq!(suite.tests.len(), 0);
    }

    #[test]
    fn test_add_test() {
        let mut suite = TestSuite::new("test_suite");
        suite.add_test("test1", || Ok(()));
        assert_eq!(suite.tests.len(), 1);
    }

    #[test]
    fn test_run_passing() {
        let mut suite = TestSuite::new("test_suite");
        suite.add_test("test1", || Ok(()));

        let results = suite.run();
        assert_eq!(results.passed, 1);
        assert_eq!(results.failed, 0);
    }

    #[test]
    fn test_run_failing() {
        let mut suite = TestSuite::new("test_suite");
        suite.add_test("test1", || Err("failed".to_string()));

        let results = suite.run();
        assert_eq!(results.passed, 0);
        assert_eq!(results.failed, 1);
    }
}
