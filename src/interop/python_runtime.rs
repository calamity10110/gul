// Python Runtime Integration using process execution
use std::collections::HashMap;
use std::process::Command;

/// Python runtime manager
pub struct PythonRuntime {}

impl PythonRuntime {
    pub fn new() -> Result<Self, String> {
        Ok(Self {})
    }

    /// Execute Python code
    pub fn execute(&self, code: &str) -> Result<String, String> {
        let output = Command::new("python3")
            .arg("-c")
            .arg(code)
            .output()
            .map_err(|e| format!("Failed to execute Python: {}", e))?;

        if output.status.success() {
            String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8: {}", e))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Python error: {}", stderr))
        }
    }

    /// Call Python function
    pub fn call_function(
        &self,
        module: &str,
        function: &str,
        args: Vec<&str>,
    ) -> Result<String, String> {
        let args_str = args
            .iter()
            .map(|a| format!("'{}'", a))
            .collect::<Vec<_>>()
            .join(", ");

        let code = format!("import {}\nprint({}({}))", module, function, args_str);

        self.execute(&code)
    }

    /// Import NumPy and convert array
    pub fn numpy_array_to_vec(&self, array_code: &str) -> Result<Vec<f64>, String> {
        let code = format!(
            "import numpy as np\nimport json\narray = {}\nprint(json.dumps(array.tolist()))",
            array_code
        );

        let result = self.execute(&code)?;
        serde_json::from_str(&result).map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Create Pandas DataFrame
    pub fn create_dataframe(&self, data: HashMap<String, Vec<String>>) -> Result<String, String> {
        let json_data =
            serde_json::to_string(&data).map_err(|e| format!("Failed to serialize data: {}", e))?;

        let code = format!(
            "import pandas as pd\nimport json\ndata = json.loads('{}')\ndf = pd.DataFrame(data)\nprint(df)",
            json_data
        );

        self.execute(&code)
    }
}

impl Default for PythonRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to initialize Python runtime")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_execute() {
        let runtime = PythonRuntime::new().unwrap();
        let result = runtime.execute("print(2 + 2)").unwrap();
        assert!(result.contains("4"));
    }

    #[test]
    fn test_python_eval_string() {
        let runtime = PythonRuntime::new().unwrap();
        let result = runtime.execute("print('hello'.upper())").unwrap();
        assert!(result.contains("HELLO"));
    }
}
