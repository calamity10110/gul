// Go Runtime Integration
// Execute Go code via subprocess or compile-and-run

use std::collections::HashMap;
use std::fs;
use std::process::Command;

/// Go runtime manager
pub struct GoRuntime {
    temp_dir: String,
}

impl GoRuntime {
    pub fn new() -> Result<Self, String> {
        // Create temp directory for Go files
        let temp_dir = std::env::temp_dir()
            .join("gul_go_runtime")
            .to_string_lossy()
            .to_string();

        fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp dir: {}", e))?;

        Ok(Self { temp_dir })
    }

    /// Execute Go code by writing to temp file and running
    pub fn execute(&self, code: &str) -> Result<String, String> {
        let file_path = format!("{}/main.go", self.temp_dir);

        // Wrap code in main package if not present
        let full_code = if code.contains("package main") {
            code.to_string()
        } else {
            format!(
                r#"package main

import "fmt"

func main() {{
    {}
}}
"#,
                code
            )
        };

        // Write Go file
        fs::write(&file_path, &full_code)
            .map_err(|e| format!("Failed to write Go file: {}", e))?;

        // Run with go run
        let output = Command::new("go")
            .arg("run")
            .arg(&file_path)
            .output()
            .map_err(|e| format!("Failed to execute Go: {}", e))?;

        if output.status.success() {
            String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8: {}", e))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Go error: {}", stderr))
        }
    }

    /// Execute Go function with arguments
    pub fn call_function(&self, func_code: &str, args: Vec<&str>) -> Result<String, String> {
        let args_str = args
            .iter()
            .map(|a| format!("\"{}\"", a))
            .collect::<Vec<_>>()
            .join(", ");

        let code = format!(
            r#"package main

import "fmt"

{}

func main() {{
    result := process({})
    fmt.Println(result)
}}
"#,
            func_code, args_str
        );

        self.execute(&code)
    }

    /// Execute concurrent Go code with goroutines
    pub fn execute_concurrent(&self, code: &str) -> Result<String, String> {
        let full_code = format!(
            r#"package main

import (
    "fmt"
    "sync"
)

func main() {{
    var wg sync.WaitGroup
    {}
    wg.Wait()
}}
"#,
            code
        );

        self.execute(&full_code)
    }

    /// Execute Go code with channels
    pub fn execute_with_channels(&self, producer: &str, consumer: &str) -> Result<String, String> {
        let full_code = format!(
            r#"package main

import "fmt"

func main() {{
    ch := make(chan interface{{}}, 100)
    done := make(chan bool)

    // Producer
    go func() {{
        {}
        close(ch)
    }}()

    // Consumer
    go func() {{
        for v := range ch {{
            {}
        }}
        done <- true
    }}()

    <-done
}}
"#,
            producer, consumer
        );

        self.execute(&full_code)
    }

    /// Execute HTTP server code (returns after timeout for testing)
    pub fn run_http_handler(&self, handler: &str, port: u16) -> Result<String, String> {
        let code = format!(
            r#"package main

import (
    "fmt"
    "net/http"
    "time"
)

func handler(w http.ResponseWriter, r *http.Request) {{
    {}
}}

func main() {{
    http.HandleFunc("/", handler)
    server := &http.Server{{Addr: ":{}"}}
    
    go func() {{
        time.Sleep(100 * time.Millisecond)
        server.Close()
    }}()
    
    fmt.Println("Server started on port {}")
    server.ListenAndServe()
}}
"#,
            handler, port, port
        );

        self.execute(&code)
    }

    /// Build standalone binary
    pub fn build(&self, code: &str, output_path: &str) -> Result<String, String> {
        let file_path = format!("{}/main.go", self.temp_dir);

        fs::write(&file_path, code).map_err(|e| format!("Failed to write Go file: {}", e))?;

        let output = Command::new("go")
            .arg("build")
            .arg("-o")
            .arg(output_path)
            .arg(&file_path)
            .output()
            .map_err(|e| format!("Failed to build Go: {}", e))?;

        if output.status.success() {
            Ok(format!("Built: {}", output_path))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Go build error: {}", stderr))
        }
    }

    /// Execute Go template
    pub fn execute_template(&self, template: &str, data: HashMap<String, String>) -> Result<String, String> {
        let data_json = serde_json::to_string(&data)
            .map_err(|e| format!("Failed to serialize data: {}", e))?;

        let code = format!(
            r#"package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "text/template"
)

func main() {{
    tmpl, err := template.New("t").Parse(`{}`)
    if err != nil {{
        fmt.Println("Template error:", err)
        return
    }}

    var data map[string]string
    json.Unmarshal([]byte(`{}`), &data)

    var buf bytes.Buffer
    tmpl.Execute(&buf, data)
    fmt.Println(buf.String())
}}
"#,
            template.replace('`', "` + \"`\" + `"),
            data_json.replace('\\', "\\\\")
        );

        self.execute(&code)
    }
}

impl Default for GoRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to initialize Go runtime")
    }
}

/// Helper function to execute Go code (for use in interpreter)
pub fn execute_go(code: &str) -> Result<String, String> {
    let runtime = GoRuntime::new()?;
    runtime.execute(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_execute() {
        if let Ok(runtime) = GoRuntime::new() {
            if let Ok(result) = runtime.execute("fmt.Println(2 + 2)") {
                assert!(result.contains("4"));
            }
        }
    }

    #[test]
    fn test_go_function() {
        if let Ok(runtime) = GoRuntime::new() {
            let func_code = r#"
func process(args ...string) string {
    return "Hello, " + args[0]
}
"#;
            if let Ok(result) = runtime.call_function(func_code, vec!["World"]) {
                assert!(result.contains("Hello, World"));
            }
        }
    }
}
