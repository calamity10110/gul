#![allow(dead_code)]
// FFI (Foreign Function Interface) module - bridges to other languages

use std::collections::HashMap;
use std::process::Command;

#[allow(clippy::type_complexity)]
pub struct FfiBridge {
    rust_functions: HashMap<String, Box<dyn Fn(Vec<String>) -> String>>,
}

impl FfiBridge {
    pub fn new() -> Self {
        FfiBridge {
            rust_functions: HashMap::new(),
        }
    }

    pub fn call_rust(&self, function: &str, args: Vec<String>) -> Result<String, String> {
        if let Some(func) = self.rust_functions.get(function) {
            Ok(func(args))
        } else {
            Err(format!("Rust function '{}' not found", function))
        }
    }

    pub fn call_c(&self, function: &str, args: Vec<String>) -> Result<String, String> {
        // For C functions, we use libloading to dynamically load shared libraries
        // This is a simplified implementation - in production, you'd load actual .so/.dll files
        Err(format!(
            "C function '{}' called with args {:?} - dynamic loading not yet configured",
            function, args
        ))
    }

    pub fn call_python(
        &self,
        code: &str,
        function: &str,
        args: Vec<String>,
    ) -> Result<String, String> {
        // Execute Python code using subprocess
        let args_str = args.join(",");
        let python_code = format!(
            "{}\nresult = {}({})\nprint(result)",
            code, function, args_str
        );

        let output = Command::new("python3")
            .arg("-c")
            .arg(&python_code)
            .output()
            .map_err(|e| format!("Failed to execute Python: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn call_javascript(
        &self,
        code: &str,
        function: &str,
        args: Vec<String>,
    ) -> Result<String, String> {
        // Execute JavaScript using Node.js
        let args_str = args
            .iter()
            .map(|a| format!("\"{}\"", a))
            .collect::<Vec<_>>()
            .join(",");

        let js_code = format!("{}\nconsole.log({}({}));", code, function, args_str);

        let output = Command::new("node")
            .arg("-e")
            .arg(&js_code)
            .output()
            .map_err(|e| format!("Failed to execute JavaScript: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn execute_sql(&self, query: &str) -> Result<String, String> {
        #[cfg(feature = "sqlite")]
        {
            use rusqlite::{Connection, OpenFlags};

            // Create or open in-memory database for simple queries
            let conn = Connection::open_with_flags(":memory:", OpenFlags::SQLITE_OPEN_READ_WRITE)
                .map_err(|e| format!("SQLite connection error: {}", e))?;

            // Determine query type
            let query_upper = query.trim().to_uppercase();
            if query_upper.starts_with("SELECT") {
                // Execute SELECT query
                let mut stmt = conn
                    .prepare(query)
                    .map_err(|e| format!("SQL prepare error: {}", e))?;

                let column_count = stmt.column_count();
                let column_names: Vec<String> = (0..column_count)
                    .map(|i| stmt.column_name(i).unwrap_or("").to_string())
                    .collect();

                let rows: Result<Vec<Vec<String>>, _> = stmt
                    .query_map([], |row| {
                        let values: Vec<String> = (0..column_count)
                            .map(|i| {
                                row.get::<_, String>(i)
                                    .unwrap_or_else(|_| "NULL".to_string())
                            })
                            .collect();
                        Ok(values)
                    })
                    .map_err(|e| format!("SQL query error: {}", e))?
                    .collect();

                let rows = rows.map_err(|e| format!("Row error: {}", e))?;

                // Format as JSON array
                let json_rows: Vec<String> = rows
                    .iter()
                    .map(|row| {
                        let pairs: Vec<String> = column_names
                            .iter()
                            .zip(row.iter())
                            .map(|(k, v)| format!("\"{}\": \"{}\"", k, v))
                            .collect();
                        format!("{{{}}}", pairs.join(", "))
                    })
                    .collect();

                Ok(format!("[{}]", json_rows.join(", ")))
            } else {
                // Execute non-SELECT query (CREATE, INSERT, UPDATE, DELETE)
                let affected = conn
                    .execute(query, [])
                    .map_err(|e| format!("SQL execute error: {}", e))?;

                Ok(format!(
                    "{{\"affected_rows\": {}, \"query\": \"{}\"}}",
                    affected,
                    query.replace('"', "\\\"")
                ))
            }
        }

        #[cfg(not(feature = "sqlite"))]
        {
            // Return structured response when sqlite feature is not enabled
            Ok(format!(
                "{{\"status\": \"simulated\", \"query\": \"{}\"}}",
                query.replace('"', "\\\"")
            ))
        }
    }

    pub fn register_rust_function<F>(&mut self, name: String, func: F)
    where
        F: Fn(Vec<String>) -> String + 'static,
    {
        self.rust_functions.insert(name, Box::new(func));
    }
}

impl Default for FfiBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_function_registration() {
        let mut bridge = FfiBridge::new();
        bridge.register_rust_function("add".to_string(), |args| {
            let a: i32 = args[0].parse().unwrap_or(0);
            let b: i32 = args[1].parse().unwrap_or(0);
            (a + b).to_string()
        });

        let result = bridge.call_rust("add", vec!["5".to_string(), "3".to_string()]);
        assert_eq!(result.unwrap(), "8");
    }
}
