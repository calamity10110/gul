// JavaScript Runtime Integration using Node.js
// Provides JS execution with JSON, async, and utility functions

use std::process::Command;

/// JavaScript runtime manager
pub struct JavaScriptRuntime {
    node_path: String,
}

impl JavaScriptRuntime {
    pub fn new() -> Self {
        // Try to find node
        let node_path = if Command::new("node").arg("--version").output().is_ok() {
            "node".to_string()
        } else if Command::new("nodejs").arg("--version").output().is_ok() {
            "nodejs".to_string()
        } else {
            "node".to_string() // Default, will fail gracefully
        };

        Self { node_path }
    }

    /// Execute JavaScript code using Node.js
    pub fn execute(&self, code: &str) -> Result<String, String> {
        let output = Command::new(&self.node_path)
            .arg("-e")
            .arg(code)
            .output()
            .map_err(|e| format!("Failed to execute Node.js: {}", e))?;

        if output.status.success() {
            String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8: {}", e))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("JavaScript error: {}", stderr))
        }
    }

    /// Evaluate JavaScript expression and return result
    pub fn eval(&self, expr: &str) -> Result<String, String> {
        let code = format!("console.log(JSON.stringify({}))", expr);
        self.execute(&code)
    }

    /// Execute async JavaScript code
    pub fn execute_async(&self, code: &str) -> Result<String, String> {
        let wrapped = format!(
            r#"
(async () => {{
    {}
}})().catch(e => console.error(e));
"#,
            code
        );
        self.execute(&wrapped)
    }

    /// Execute JavaScript with await
    pub fn execute_with_await(&self, code: &str) -> Result<String, String> {
        let wrapped = format!(
            r#"
(async () => {{
    const result = await (async () => {{ {} }})();
    console.log(JSON.stringify(result));
}})();
"#,
            code
        );
        self.execute(&wrapped)
    }

    /// Parse JSON string
    pub fn parse_json(&self, json_str: &str) -> Result<String, String> {
        let code = format!(
            "console.log(JSON.stringify(JSON.parse('{}'), null, 2))",
            json_str.replace('\'', "\\'").replace('\n', "\\n")
        );
        self.execute(&code)
    }

    /// Stringify object to JSON
    pub fn stringify_json(&self, obj_code: &str) -> Result<String, String> {
        let code = format!("console.log(JSON.stringify({}, null, 2))", obj_code);
        self.execute(&code)
    }

    /// Array operations
    pub fn array_map(&self, array: &str, mapper: &str) -> Result<String, String> {
        let code = format!("console.log(JSON.stringify(({}).map({})))", array, mapper);
        self.execute(&code)
    }

    pub fn array_filter(&self, array: &str, predicate: &str) -> Result<String, String> {
        let code = format!(
            "console.log(JSON.stringify(({}).filter({})))",
            array, predicate
        );
        self.execute(&code)
    }

    pub fn array_reduce(
        &self,
        array: &str,
        reducer: &str,
        initial: &str,
    ) -> Result<String, String> {
        let code = format!(
            "console.log(JSON.stringify(({}).reduce({}, {})))",
            array, reducer, initial
        );
        self.execute(&code)
    }

    /// Object operations
    pub fn object_keys(&self, obj: &str) -> Result<String, String> {
        let code = format!("console.log(JSON.stringify(Object.keys({})))", obj);
        self.execute(&code)
    }

    pub fn object_values(&self, obj: &str) -> Result<String, String> {
        let code = format!("console.log(JSON.stringify(Object.values({})))", obj);
        self.execute(&code)
    }

    pub fn object_entries(&self, obj: &str) -> Result<String, String> {
        let code = format!("console.log(JSON.stringify(Object.entries({})))", obj);
        self.execute(&code)
    }

    /// HTTP fetch (requires node-fetch or Node 18+)
    pub fn fetch(&self, url: &str) -> Result<String, String> {
        let code = format!(
            r#"
(async () => {{
    const response = await fetch('{}');
    const data = await response.text();
    console.log(data);
}})();
"#,
            url
        );
        self.execute(&code)
    }

    pub fn fetch_json(&self, url: &str) -> Result<String, String> {
        let code = format!(
            r#"
(async () => {{
    const response = await fetch('{}');
    const data = await response.json();
    console.log(JSON.stringify(data));
}})();
"#,
            url
        );
        self.execute(&code)
    }

    /// File system operations (Node.js)
    pub fn read_file(&self, path: &str) -> Result<String, String> {
        let code = format!(
            "const fs = require('fs'); console.log(fs.readFileSync('{}', 'utf8'))",
            path
        );
        self.execute(&code)
    }

    pub fn write_file(&self, path: &str, content: &str) -> Result<String, String> {
        let code = format!(
            "const fs = require('fs'); fs.writeFileSync('{}', '{}'); console.log('Written')",
            path,
            content.replace('\'', "\\'").replace('\n', "\\n")
        );
        self.execute(&code)
    }

    /// Crypto operations
    pub fn hash_sha256(&self, input: &str) -> Result<String, String> {
        let code = format!(
            "const crypto = require('crypto'); console.log(crypto.createHash('sha256').update('{}').digest('hex'))",
            input.replace('\'', "\\'")
        );
        self.execute(&code)
    }

    pub fn random_uuid(&self) -> Result<String, String> {
        self.execute("const crypto = require('crypto'); console.log(crypto.randomUUID())")
    }

    /// Date/Time operations
    pub fn current_timestamp(&self) -> Result<String, String> {
        self.execute("console.log(Date.now())")
    }

    pub fn format_date(&self, format: &str) -> Result<String, String> {
        let code = format!(
            "console.log(new Date().toLocaleString('en-US', {}))",
            format
        );
        self.execute(&code)
    }

    /// Execute TypeScript (requires ts-node)
    pub fn execute_typescript(&self, code: &str) -> Result<String, String> {
        let output = Command::new("npx")
            .arg("ts-node")
            .arg("-e")
            .arg(code)
            .output()
            .map_err(|e| format!("Failed to execute TypeScript: {}", e))?;

        if output.status.success() {
            String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8: {}", e))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("TypeScript error: {}", stderr))
        }
    }
}

impl Default for JavaScriptRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function for interpreter use
pub fn execute_js(code: &str) -> Result<String, String> {
    let runtime = JavaScriptRuntime::new();
    runtime.execute(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_execute() {
        let runtime = JavaScriptRuntime::new();
        if let Ok(result) = runtime.execute("console.log(2 + 2)") {
            assert!(result.contains("4"));
        }
    }

    #[test]
    fn test_js_eval() {
        let runtime = JavaScriptRuntime::new();
        if let Ok(result) = runtime.eval("2 + 2") {
            assert!(result.contains("4"));
        }
    }

    #[test]
    fn test_js_array_map() {
        let runtime = JavaScriptRuntime::new();
        if let Ok(result) = runtime.array_map("[1, 2, 3]", "x => x * 2") {
            assert!(result.contains("[2,4,6]"));
        }
    }

    #[test]
    fn test_js_hash() {
        let runtime = JavaScriptRuntime::new();
        if let Ok(result) = runtime.hash_sha256("hello") {
            assert!(!result.is_empty());
        }
    }
}
