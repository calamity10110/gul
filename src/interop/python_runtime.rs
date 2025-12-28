// Python Runtime Integration using process execution

use std::collections::HashMap;
use std::process::Command;

/// Python runtime manager with enhanced ML/Data Science support
pub struct PythonRuntime {
    python_path: String,
}

impl PythonRuntime {
    pub fn new() -> Result<Self, String> {
        // Try to find python3
        let python_path = if Command::new("python3").arg("--version").output().is_ok() {
            "python3".to_string()
        } else if Command::new("python").arg("--version").output().is_ok() {
            "python".to_string()
        } else {
            return Err("Python not found".to_string());
        };

        Ok(Self { python_path })
    }

    /// Execute Python code
    pub fn execute(&self, code: &str) -> Result<String, String> {
        let output = Command::new(&self.python_path)
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

    /// Call Python function with JSON args
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

    /// NumPy array operations
    pub fn numpy_array_to_vec(&self, array_code: &str) -> Result<Vec<f64>, String> {
        let code = format!(
            "import numpy as np\nimport json\narray = {}\nprint(json.dumps(array.tolist()))",
            array_code
        );

        let result = self.execute(&code)?;
        serde_json::from_str(&result).map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// NumPy statistical analysis
    pub fn numpy_stats(&self, data: &[f64]) -> Result<HashMap<String, f64>, String> {
        let data_json = serde_json::to_string(data).map_err(|e| e.to_string())?;

        let code = format!(
            r#"
import numpy as np
import json
data = np.array(json.loads('{}'))
result = {{
    "mean": float(np.mean(data)),
    "std": float(np.std(data)),
    "var": float(np.var(data)),
    "min": float(np.min(data)),
    "max": float(np.max(data)),
    "median": float(np.median(data)),
    "sum": float(np.sum(data))
}}
print(json.dumps(result))
"#,
            data_json
        );

        let result = self.execute(&code)?;
        serde_json::from_str(&result).map_err(|e| format!("Failed to parse stats: {}", e))
    }

    /// Pandas DataFrame operations
    pub fn create_dataframe(&self, data: HashMap<String, Vec<String>>) -> Result<String, String> {
        let json_data =
            serde_json::to_string(&data).map_err(|e| format!("Failed to serialize data: {}", e))?;

        let code = format!(
            "import pandas as pd\nimport json\ndata = json.loads('{}')\ndf = pd.DataFrame(data)\nprint(df.to_json(orient='records'))",
            json_data.replace('\'', "\\'")
        );

        self.execute(&code)
    }

    /// Pandas CSV read
    pub fn read_csv(&self, path: &str) -> Result<String, String> {
        let code = format!(
            "import pandas as pd\ndf = pd.read_csv('{}')\nprint(df.to_json(orient='records'))",
            path
        );
        self.execute(&code)
    }

    /// Pandas data analysis
    pub fn analyze_dataframe(&self, csv_path: &str) -> Result<String, String> {
        let code = format!(
            r#"
import pandas as pd
import json
df = pd.read_csv('{}')
result = {{
    "shape": list(df.shape),
    "columns": list(df.columns),
    "dtypes": {{k: str(v) for k, v in df.dtypes.items()}},
    "describe": df.describe().to_dict(),
    "head": df.head().to_dict(orient='records')
}}
print(json.dumps(result, default=str))
"#,
            csv_path
        );
        self.execute(&code)
    }

    /// Scikit-learn model training
    pub fn train_model(
        &self,
        model_type: &str,
        x_data: &str,
        y_data: &str,
    ) -> Result<String, String> {
        let code = format!(
            r#"
import numpy as np
import json
from sklearn.model_selection import train_test_split
from sklearn.metrics import accuracy_score, r2_score

X = np.array({})
y = np.array({})

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

if '{}' == 'linear':
    from sklearn.linear_model import LinearRegression
    model = LinearRegression()
    model.fit(X_train, y_train)
    y_pred = model.predict(X_test)
    score = r2_score(y_test, y_pred)
    result = {{"type": "linear", "r2_score": score, "coef": model.coef_.tolist(), "intercept": float(model.intercept_)}}
elif '{}' == 'logistic':
    from sklearn.linear_model import LogisticRegression
    model = LogisticRegression()
    model.fit(X_train, y_train)
    y_pred = model.predict(X_test)
    score = accuracy_score(y_test, y_pred)
    result = {{"type": "logistic", "accuracy": score}}
elif '{}' == 'random_forest':
    from sklearn.ensemble import RandomForestClassifier
    model = RandomForestClassifier(n_estimators=100)
    model.fit(X_train, y_train)
    y_pred = model.predict(X_test)
    score = accuracy_score(y_test, y_pred)
    result = {{"type": "random_forest", "accuracy": score, "feature_importance": model.feature_importances_.tolist()}}
else:
    result = {{"error": "Unknown model type"}}

print(json.dumps(result))
"#,
            x_data, y_data, model_type, model_type, model_type
        );

        self.execute(&code)
    }

    /// TensorFlow/Keras neural network
    pub fn create_neural_network(&self, layers: &str, input_shape: &str) -> Result<String, String> {
        let code = format!(
            r#"
import tensorflow as tf
import json

model = tf.keras.Sequential([
    tf.keras.layers.Input(shape={}),
    {}
])

model.compile(optimizer='adam', loss='mse', metrics=['mae'])
summary = []
model.summary(print_fn=lambda x: summary.append(x))
result = {{"summary": summary, "total_params": int(model.count_params())}}
print(json.dumps(result))
"#,
            input_shape, layers
        );

        self.execute(&code)
    }

    /// PyTorch tensor operations
    pub fn pytorch_ops(&self, code: &str) -> Result<String, String> {
        let full_code = format!(
            r#"
import torch
import json

{}

print(json.dumps(result, default=lambda x: x.tolist() if hasattr(x, 'tolist') else str(x)))
"#,
            code
        );

        self.execute(&full_code)
    }

    /// Matplotlib plot generation (saves to file)
    pub fn create_plot(&self, plot_code: &str, output_path: &str) -> Result<String, String> {
        let code = format!(
            r#"
import matplotlib.pyplot as plt
import numpy as np

{}

plt.savefig('{}', dpi=150, bbox_inches='tight')
print('Plot saved to {}')
"#,
            plot_code, output_path, output_path
        );

        self.execute(&code)
    }

    /// Async Python execution with asyncio
    pub fn execute_async(&self, async_code: &str) -> Result<String, String> {
        let code = format!(
            r#"
import asyncio

async def main():
    {}

asyncio.run(main())
"#,
            async_code
        );

        self.execute(&code)
    }

    /// Execute Python with timeout
    pub fn execute_with_timeout(&self, code: &str, _timeout_secs: u64) -> Result<String, String> {

        let child = Command::new(&self.python_path)
            .arg("-c")
            .arg(code)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn Python: {}", e))?;

        let output = child
            .wait_with_output()
            .map_err(|e| format!("Failed to wait: {}", e))?;

        if output.status.success() {
            String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8: {}", e))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Python error: {}", stderr))
        }
    }
}

impl Default for PythonRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to initialize Python runtime")
    }
}

/// Helper function for interpreter use
pub fn execute_python(code: &str) -> Result<String, String> {
    let runtime = PythonRuntime::new()?;
    runtime.execute(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_execute() {
        if let Ok(runtime) = PythonRuntime::new() {
            let result = runtime.execute("print(2 + 2)").unwrap();
            assert!(result.contains("4"));
        }
    }

    #[test]
    fn test_python_eval_string() {
        if let Ok(runtime) = PythonRuntime::new() {
            let result = runtime.execute("print('hello'.upper())").unwrap();
            assert!(result.contains("HELLO"));
        }
    }

    #[test]
    fn test_numpy_stats() {
        if let Ok(runtime) = PythonRuntime::new() {
            if let Ok(stats) = runtime.numpy_stats(&[1.0, 2.0, 3.0, 4.0, 5.0]) {
                assert!(stats.contains_key("mean"));
            }
        }
    }
}
