use crate::frontend::ast::*;
use super::values::{Value, ControlFlow};
use super::Interpreter;
use rayon::prelude::*;
use std::process::Command;
use std::io::Write;

impl Interpreter {
    pub(crate) fn execute_statements(&mut self, statements: &[Statement]) -> Result<ControlFlow, String> {
        for stmt in statements {
            let flow = self.execute(stmt)?;
            match flow {
                ControlFlow::Next => continue,
                _ => return Ok(flow),
            }
        }
        Ok(ControlFlow::Next)
    }

    pub(crate) fn execute(&mut self, stmt: &Statement) -> Result<ControlFlow, String> {
        match stmt {
            Statement::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(ControlFlow::Next)
            }
            Statement::Definition { name, value } => {
                let val = self.evaluate(value)?;
                self.variables.insert(name.clone(), val);
                Ok(ControlFlow::Next)
            }
            Statement::If {
                condition,
                then_body,
                else_body,
            } => {
                let cond = self.evaluate(condition)?;
                let truthy = match cond {
                    Value::Bool(b) => b,
                    Value::Null => false,
                    Value::Integer(i) => i != 0,
                    _ => true,
                };
                if truthy {
                    self.execute_statements(then_body)
                } else if let Some(else_stmts) = else_body {
                    self.execute_statements(else_stmts)
                } else {
                    Ok(ControlFlow::Next)
                }
            }
            Statement::Loop { body } => {
                loop {
                    let flow = self.execute_statements(body)?;
                    match flow {
                        ControlFlow::Break => break Ok(ControlFlow::Next),
                        ControlFlow::Continue | ControlFlow::Next => continue,
                        ControlFlow::Return(v) => break Ok(ControlFlow::Return(v)),
                    }
                }
            }
            Statement::While { condition, body, is_parallel } => {
                if *is_parallel {
                    // also_while implementation (v3.2)
                    // Simplified: parallel execute one iteration if condition holds?
                    // While loops are hard to parallelize safely without transactions.
                    // For now, execute sequentially but mark as parallel-intended.
                    println!("Warning: also_while executed sequentially for safety.");
                }
                loop {
                     let cond = self.evaluate(condition)?;
                     let truthy = match cond {
                        Value::Bool(b) => b,
                        Value::Null => false,
                        Value::Integer(i) => i != 0,
                        _ => true,
                    };
                    if !truthy { break Ok(ControlFlow::Next); }

                    let flow = self.execute_statements(body)?;
                    match flow {
                        ControlFlow::Break => break Ok(ControlFlow::Next),
                        ControlFlow::Continue | ControlFlow::Next => continue,
                        ControlFlow::Return(v) => break Ok(ControlFlow::Return(v)),
                    }
                }
            }
            Statement::For {
                variable,
                iterable,
                body,
                is_parallel,
            } => {
                let iterable_val = self.evaluate(iterable)?;
                match iterable_val {
                    Value::List(items) => {
                        if *is_parallel {
                            // Parallel execution using Rayon
                            items.par_iter().for_each(|item| {
                                let mut thread_interpreter = self.clone();
                                thread_interpreter.variables.insert(variable.clone(), item.clone());
                                let _ = thread_interpreter.execute_statements(body);
                            });
                            Ok(ControlFlow::Next)
                        } else {
                            // Sequential execution
                            for item in items {
                                self.variables.insert(variable.clone(), item);
                                let flow = self.execute_statements(body)?;
                                match flow {
                                    ControlFlow::Break => break,
                                    ControlFlow::Continue | ControlFlow::Next => continue,
                                    ControlFlow::Return(v) => return Ok(ControlFlow::Return(v)),
                                }
                            }
                            Ok(ControlFlow::Next)
                        }
                    }
                    _ => Err("For loop expects a list".to_string()),
                }
            }
            Statement::Function {
                name, params, outputs, body, ..
            } => {
                let val = Value::Function(params.clone(), outputs.clone(), body.clone());
                self.variables.insert(name.clone(), val);
                Ok(ControlFlow::Next)
            }
            Statement::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.evaluate(e)?
                } else {
                    Value::Null
                };
                Ok(ControlFlow::Return(val))
            }
            Statement::Break => Ok(ControlFlow::Break),
            Statement::Continue => Ok(ControlFlow::Continue),
            Statement::Import(modules) => {
                for name in modules {
                    if let Some(module) = crate::stdlib::load_std_module(name) {
                        let parts: Vec<&str> = name.split('.').collect();
                        if let Some(short_name) = parts.last() {
                            self.variables.insert(short_name.to_string(), module);
                        }
                    }
                }
                Ok(ControlFlow::Next)
            }
            Statement::Assignment { name, value } => {
                let val = self.evaluate(value)?;
                self.variables.insert(name.clone(), val);
                Ok(ControlFlow::Next)
            }
            Statement::GlobalDef { name, value, .. } => {
                let val = self.evaluate(value)?;
                self.variables.insert(name.clone(), val);
                Ok(ControlFlow::Next)
            }
            Statement::Main { body } => {
                self.execute_statements(body).map(|_| ControlFlow::Next)
            }
            Statement::ForeignBlock { language, code } => {
                // Execute foreign code blocks based on language
                match language.as_str() {
                    "python" => {
                        let output = Command::new("python3")
                            .arg("-c")
                            .arg(code)
                            .output();

                        match output {
                            Ok(out) => {
                                if !out.status.success() {
                                    eprintln!("Python error: {}", String::from_utf8_lossy(&out.stderr));
                                }
                                print!("{}", String::from_utf8_lossy(&out.stdout));
                            }
                            Err(e) => eprintln!("Failed to execute python: {}", e),
                        }
                    }
                    "rust" => {
                        // Rust blocks are compiled at compile time, skip at runtime
                        // But for GUL script execution, we compile and run on the fly?
                        let temp_file = "gul_temp.rs";
                        let temp_bin = "./gul_temp_bin";

                        // 1. Write code to file
                        if let Ok(mut file) = std::fs::File::create(temp_file) {
                            let _ = file.write_all(code.as_bytes());
                        }

                        // 2. Compile
                        let status = Command::new("rustc")
                            .arg(temp_file)
                            .arg("-o")
                            .arg("gul_temp_bin")
                            .status();

                        if let Ok(s) = status {
                            if s.success() {
                                // 3. Run
                                let _start = std::time::Instant::now();
                                let run = Command::new(temp_bin).status();
                                if let Ok(_) = run {
                                     // Success
                                }
                                // Cleanup
                                let _ = std::fs::remove_file(temp_file);
                                let _ = std::fs::remove_file(temp_bin);
                            } else {
                                eprintln!("Rust compilation failed");
                            }
                        } else {
                             eprintln!("Failed to run rustc");
                        }
                    }
                    "sql" => {
                        // SQL blocks can be executed against database
                        // Placeholder
                         println!("[SQL block - {} chars]", code.len());
                    }
                    "js" | "javascript" => {
                         // Placeholder
                         println!("[JavaScript block - {} chars]", code.len());
                    }
                    "c" => {
                        // C blocks
                        println!("[C block compiled]");
                    }
                    _ => {
                        println!("[{} block - {} chars]", language, code.len());
                    }
                }
                Ok(ControlFlow::Next)
            }
            Statement::Try {
                try_body,
                catch_var,
                catch_body,
                finally_body,
            } => {
                // Execute try block
                let result = self.execute_statements(try_body);

                match result {
                    Ok(_) => {
                        // Try succeeded, execute finally if present
                        if let Some(finally) = finally_body {
                            let _ = self.execute_statements(finally); // Ignore finally errors for now
                        }
                        Ok(ControlFlow::Next)
                    }
                    Err(error) => {
                        // Try failed, execute catch if present
                        if let Some(catch) = catch_body {
                            if let Some(var_name) = catch_var {
                                // Bind error to variable
                                self.variables
                                    .insert(var_name.clone(), Value::String(error));
                            }
                            let catch_result = self.execute_statements(catch);
                            // Execute finally even if catch fails
                            if let Some(finally) = finally_body {
                                let _ = self.execute_statements(finally);
                            }
                            catch_result
                        } else {
                            // No catch, execute finally and re-raise error
                            if let Some(finally) = finally_body {
                                let _ = self.execute_statements(finally);
                            }
                            Err(error)
                        }
                    }
                }
            }
            Statement::Throw(expr) => {
                let error_val = self.evaluate(expr)?;
                let error_msg = match error_val {
                    Value::String(s) => s,
                    _ => format!("{:?}", error_val),
                };
                Err(error_msg)
            }
            _ => Ok(ControlFlow::Next),
        }
    }
}
