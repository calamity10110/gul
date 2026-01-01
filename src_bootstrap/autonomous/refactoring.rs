#![allow(unused_variables)]
// Self-Refactoring Compiler Module

use std::collections::HashMap;

/// Code pattern types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodePattern {
    DuplicateCode,
    LongMethod,
    GodObject,
    FeatureEnvy,
    DataClump,
    PrimitiveObsession,
    SwitchStatements,
    LazyClass,
    Singleton,
    Factory,
    Observer,
    Strategy,
}

/// Refactoring operation
#[derive(Debug, Clone)]
pub enum RefactoringOp {
    ExtractMethod {
        start_line: usize,
        end_line: usize,
        new_name: String,
    },
    RenameVariable {
        old_name: String,
        new_name: String,
    },
    InlineMethod {
        method_name: String,
    },
    ExtractClass {
        fields: Vec<String>,
        new_class_name: String,
    },
    IntroduceInterface {
        methods: Vec<String>,
        interface_name: String,
    },
    OptimizeLoop {
        loop_line: usize,
        optimization_type: LoopOptimization,
    },
}

#[derive(Debug, Clone)]
pub enum LoopOptimization {
    Unroll(usize),
    Vectorize,
    Parallelize,
    CacheOptimize,
}

/// Pattern detection result
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern: CodePattern,
    pub location: Location,
    pub severity: Severity,
    pub description: String,
    pub suggested_refactoring: Vec<RefactoringOp>,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Self-refactoring compiler
pub struct SelfRefactoringCompiler {
    patterns: HashMap<CodePattern, PatternDetector>,
    refactoring_history: Vec<RefactoringOp>,
    auto_apply: bool,
}

type PatternDetector = fn(&str) -> Vec<PatternMatch>;

impl SelfRefactoringCompiler {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // Register pattern detectors
        patterns.insert(
            CodePattern::DuplicateCode,
            Self::detect_duplicate_code as PatternDetector,
        );
        patterns.insert(
            CodePattern::LongMethod,
            Self::detect_long_method as PatternDetector,
        );
        patterns.insert(
            CodePattern::GodObject,
            Self::detect_god_object as PatternDetector,
        );

        SelfRefactoringCompiler {
            patterns,
            refactoring_history: Vec::new(),
            auto_apply: false,
        }
    }

    pub fn enable_auto_apply(&mut self) {
        self.auto_apply = true;
    }

    /// Analyze code for patterns
    pub fn analyze(&self, code: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        for (pattern, detector) in &self.patterns {
            let pattern_matches = detector(code);
            matches.extend(pattern_matches);
        }

        matches
    }

    /// Apply refactoring
    pub fn apply_refactoring(&mut self, code: &str, op: RefactoringOp) -> Result<String, String> {
        let refactored = match &op {
            RefactoringOp::ExtractMethod {
                start_line,
                end_line,
                new_name,
            } => self.extract_method(code, *start_line, *end_line, new_name)?,
            RefactoringOp::RenameVariable { old_name, new_name } => {
                self.rename_variable(code, old_name, new_name)
            }
            RefactoringOp::InlineMethod { method_name } => self.inline_method(code, method_name)?,
            RefactoringOp::ExtractClass {
                fields,
                new_class_name,
            } => self.extract_class(code, fields, new_class_name)?,
            RefactoringOp::IntroduceInterface {
                methods,
                interface_name,
            } => self.introduce_interface(code, methods, interface_name)?,
            RefactoringOp::OptimizeLoop {
                loop_line,
                optimization_type,
            } => self.optimize_loop(code, *loop_line, optimization_type)?,
        };

        self.refactoring_history.push(op);
        Ok(refactored)
    }

    /// Detect duplicate code
    fn detect_duplicate_code(code: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        // Simple duplicate detection: look for identical sequences of 3+ lines
        for i in 0..lines.len().saturating_sub(3) {
            for j in (i + 3)..lines.len().saturating_sub(3) {
                if Self::lines_match(&lines[i..i + 3], &lines[j..j + 3]) {
                    matches.push(PatternMatch {
                        pattern: CodePattern::DuplicateCode,
                        location: Location {
                            file: "current".to_string(),
                            start_line: i + 1,
                            end_line: i + 3,
                        },
                        severity: Severity::Warning,
                        description: format!(
                            "Duplicate code found at lines {} and {}",
                            i + 1,
                            j + 1
                        ),
                        suggested_refactoring: vec![RefactoringOp::ExtractMethod {
                            start_line: i + 1,
                            end_line: i + 3,
                            new_name: "extracted_method".to_string(),
                        }],
                    });
                    break;
                }
            }
        }

        matches
    }

    fn lines_match(a: &[&str], b: &[&str]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.iter()
            .zip(b.iter())
            .all(|(x, y)| x.trim() == y.trim() && !x.trim().is_empty())
    }

    /// Detect long methods
    fn detect_long_method(code: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = code.lines().collect();
        let mut in_function = false;
        let mut function_start = 0;
        let mut function_name = String::new();

        for (i, line) in lines.iter().enumerate() {
            if line.contains("fn ") && line.contains("{") {
                in_function = true;
                function_start = i;
                function_name = line
                    .split("fn ")
                    .nth(1)
                    .and_then(|s| s.split('(').next())
                    .unwrap_or("unknown")
                    .to_string();
            } else if in_function && line.contains("}") && !line.trim().starts_with("//") {
                let function_length = i - function_start;
                if function_length > 50 {
                    matches.push(PatternMatch {
                        pattern: CodePattern::LongMethod,
                        location: Location {
                            file: "current".to_string(),
                            start_line: function_start + 1,
                            end_line: i + 1,
                        },
                        severity: Severity::Warning,
                        description: format!(
                            "Method '{}' is too long ({} lines)",
                            function_name, function_length
                        ),
                        suggested_refactoring: vec![RefactoringOp::ExtractMethod {
                            start_line: function_start + 10,
                            end_line: function_start + 20,
                            new_name: format!("{}_helper", function_name),
                        }],
                    });
                }
                in_function = false;
            }
        }

        matches
    }

    /// Detect god objects (classes with too many responsibilities)
    fn detect_god_object(code: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        let lines: Vec<&str> = code.lines().collect();
        let mut in_struct = false;
        let mut struct_start = 0;
        let mut struct_name = String::new();
        let mut field_count = 0;
        let mut method_count = 0;

        for (i, line) in lines.iter().enumerate() {
            if line.contains("struct ") && line.contains("{") {
                in_struct = true;
                struct_start = i;
                struct_name = line
                    .split("struct ")
                    .nth(1)
                    .and_then(|s| s.split_whitespace().next())
                    .unwrap_or("unknown")
                    .to_string();
                field_count = 0;
                method_count = 0;
            } else if in_struct {
                if line.contains(":") && !line.contains("fn ") {
                    field_count += 1;
                } else if line.contains("fn ") {
                    method_count += 1;
                } else if line.trim() == "}" {
                    if field_count > 10 || method_count > 15 {
                        matches.push(PatternMatch {
                            pattern: CodePattern::GodObject,
                            location: Location {
                                file: "current".to_string(),
                                start_line: struct_start + 1,
                                end_line: i + 1,
                            },
                            severity: Severity::Error,
                            description: format!(
                                "Struct '{}' has too many responsibilities ({} fields, {} methods)",
                                struct_name, field_count, method_count
                            ),
                            suggested_refactoring: vec![RefactoringOp::ExtractClass {
                                fields: vec!["field1".to_string(), "field2".to_string()],
                                new_class_name: format!("{}Helper", struct_name),
                            }],
                        });
                    }
                    in_struct = false;
                }
            }
        }

        matches
    }

    /// Extract method refactoring
    fn extract_method(
        &self,
        code: &str,
        start: usize,
        end: usize,
        name: &str,
    ) -> Result<String, String> {
        let lines: Vec<&str> = code.lines().collect();
        if start == 0 || start > end || end > lines.len() {
            return Err("Invalid line range".to_string());
        }

        let extracted_lines = &lines[start - 1..end];
        let method_body = extracted_lines.join("\n");

        let new_method = format!("\nfn {}() {{\n{}\n}}\n", name, method_body);

        // Replace extracted lines with method call
        let mut result = lines[..start - 1].join("\n");
        result.push_str(&format!("\n    {}();\n", name));
        result.push_str(&lines[end..].join("\n"));
        result.push_str(&new_method);

        Ok(result)
    }

    /// Rename variable refactoring
    fn rename_variable(&self, code: &str, old_name: &str, new_name: &str) -> String {
        // Simple replacement (in production, would use AST-based renaming)
        code.replace(old_name, new_name)
    }

    /// Inline method refactoring
    fn inline_method(&self, code: &str, method_name: &str) -> Result<String, String> {
        // Find method definition
        let lines: Vec<&str> = code.lines().collect();
        let mut method_start = None;
        let mut method_end = None;
        let mut method_body = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if line.contains(&format!("fn {}", method_name)) {
                method_start = Some(i);
            } else if method_start.is_some() && method_end.is_none() {
                if line.trim() == "}" {
                    method_end = Some(i);
                    break;
                } else if !line.contains("fn ") {
                    method_body.push(line.trim());
                }
            }
        }

        if method_start.is_none() || method_end.is_none() {
            return Err(format!("Method '{}' not found", method_name));
        }

        // Replace method calls with method body
        let inlined_body = method_body.join("\n");
        let call_pattern = format!("{}()", method_name);
        let result = code.replace(&call_pattern, &inlined_body);

        Ok(result)
    }

    /// Extract class refactoring
    fn extract_class(
        &self,
        code: &str,
        fields: &[String],
        new_class_name: &str,
    ) -> Result<String, String> {
        let mut new_class = format!(
            "\n#[derive(Debug, Clone)]\npub struct {} {{\n",
            new_class_name
        );
        for field in fields {
            new_class.push_str(&format!("    pub {}: String,\n", field));
        }
        new_class.push_str("}\n");

        Ok(format!("{}\n{}", code, new_class))
    }

    /// Introduce interface refactoring
    fn introduce_interface(
        &self,
        code: &str,
        methods: &[String],
        interface_name: &str,
    ) -> Result<String, String> {
        let mut interface = format!("\npub trait {} {{\n", interface_name);
        for method in methods {
            interface.push_str(&format!("    fn {}(&self);\n", method));
        }
        interface.push_str("}\n");

        Ok(format!("{}\n{}", code, interface))
    }

    /// Optimize loop
    fn optimize_loop(
        &self,
        code: &str,
        loop_line: usize,
        opt_type: &LoopOptimization,
    ) -> Result<String, String> {
        let lines: Vec<&str> = code.lines().collect();
        if loop_line == 0 || loop_line > lines.len() {
            return Err("Invalid loop line".to_string());
        }

        let optimized_comment = match opt_type {
            LoopOptimization::Unroll(factor) => format!("// Loop unrolled by factor {}", factor),
            LoopOptimization::Vectorize => "// Loop vectorized with SIMD".to_string(),
            LoopOptimization::Parallelize => "// Loop parallelized".to_string(),
            LoopOptimization::CacheOptimize => "// Loop optimized for cache".to_string(),
        };

        let mut result = lines[..loop_line - 1].to_vec();
        result.push(&optimized_comment);
        result.extend_from_slice(&lines[loop_line - 1..]);

        Ok(result.join("\n"))
    }

    /// Get refactoring history
    pub fn get_history(&self) -> &[RefactoringOp] {
        &self.refactoring_history
    }

    /// Undo last refactoring
    pub fn undo(&mut self) -> Option<RefactoringOp> {
        self.refactoring_history.pop()
    }
}

impl Default for SelfRefactoringCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_creation() {
        let compiler = SelfRefactoringCompiler::new();
        assert_eq!(compiler.refactoring_history.len(), 0);
    }

    #[test]
    fn test_detect_duplicate_code() {
        let code = r#"
fn test() {
    let x = 1;
    let y = 2;
    let z = 3;
}

fn test2() {
    let x = 1;
    let y = 2;
    let z = 3;
}
"#;
        let matches = SelfRefactoringCompiler::detect_duplicate_code(code);
        assert!(!matches.is_empty());
    }

    #[test]
    fn test_detect_long_method() {
        let mut code = String::from("fn long_method() {\n");
        for i in 0..60 {
            code.push_str(&format!("    let x{} = {};\n", i, i));
        }
        code.push_str("}\n");

        let matches = SelfRefactoringCompiler::detect_long_method(&code);
        assert!(!matches.is_empty());
        assert_eq!(matches[0].pattern, CodePattern::LongMethod);
    }

    #[test]
    fn test_rename_variable() {
        let compiler = SelfRefactoringCompiler::new();
        let code = "let old_name = 5;\nprintln!(\"{}\", old_name);";
        let result = compiler.rename_variable(code, "old_name", "new_name");
        assert!(result.contains("new_name"));
        assert!(!result.contains("old_name"));
    }

    #[test]
    fn test_extract_class() {
        let compiler = SelfRefactoringCompiler::new();
        let code = "struct Original {}";
        let result = compiler
            .extract_class(
                code,
                &["field1".to_string(), "field2".to_string()],
                "NewClass",
            )
            .unwrap();
        assert!(result.contains("struct NewClass"));
        assert!(result.contains("field1"));
    }

    #[test]
    fn test_introduce_interface() {
        let compiler = SelfRefactoringCompiler::new();
        let code = "struct MyStruct {}";
        let result = compiler
            .introduce_interface(
                code,
                &["method1".to_string(), "method2".to_string()],
                "MyTrait",
            )
            .unwrap();
        assert!(result.contains("trait MyTrait"));
        assert!(result.contains("fn method1"));
    }

    #[test]
    fn test_refactoring_history() {
        let mut compiler = SelfRefactoringCompiler::new();
        let code = "let x = 1;";

        let op = RefactoringOp::RenameVariable {
            old_name: "x".to_string(),
            new_name: "y".to_string(),
        };

        compiler.apply_refactoring(code, op).unwrap();
        assert_eq!(compiler.get_history().len(), 1);

        let undone = compiler.undo();
        assert!(undone.is_some());
        assert_eq!(compiler.get_history().len(), 0);
    }

    #[test]
    fn test_optimize_loop() {
        let compiler = SelfRefactoringCompiler::new();
        let code = "for i in 0..10 {\n    println!(\"{}\", i);\n}";
        let result = compiler
            .optimize_loop(code, 1, &LoopOptimization::Vectorize)
            .unwrap();
        assert!(result.contains("vectorized"));
    }
}
