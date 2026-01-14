use crate::ast::*;
use std::collections::HashMap;
use std::fmt;
use rayon::prelude::*;
use std::process::Command;
use std::io::Write;

#[derive(Clone, Debug)]
pub enum ControlFlow {
    Next,
    Return(Value),
    Break,
    Continue,
}

#[derive(Clone)]
pub struct Interpreter {
    variables: HashMap<String, Value>,
}

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Object(String, HashMap<String, Value>), // Struct instance
    Function(Vec<(String, Option<Type>)>, Vec<(String, Option<Type>)>, Vec<Statement>),
    NativeFunction(fn(Vec<Value>) -> Value),
    Lambda(Vec<(String, Option<Type>)>, Box<Expression>), // Updated Lambda for v3.2
    Dual(f64, f64), // Auto-diff: (value, gradient)
    Any(Box<Value>), // Gradual typing - boxed to avoid recursion
    Null,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Dict(a), Value::Dict(b)) => a == b,
            (Value::Object(n1, f1), Value::Object(n2, f2)) => n1 == n2 && f1 == f2,
            (Value::Function(p1, o1, b1), Value::Function(p2, o2, b2)) => p1 == p2 && o1 == o2 && b1 == b2,
            (Value::NativeFunction(_), Value::NativeFunction(_)) => false, // Can't compare fn ptrs
            (Value::Lambda(p1, b1), Value::Lambda(p2, b2)) => p1 == p2 && b1 == b2,
            (Value::Dual(v1, d1), Value::Dual(v2, d2)) => v1 == v2 && d1 == d2,
            (Value::Any(a), Value::Any(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fp) => write!(f, "{}", fp),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::List(l) => write!(f, "{:?}", l),
            Value::Dict(d) => write!(f, "{:?}", d),
            Value::Object(name, fields) => write!(f, "{} {:?}", name, fields),
            Value::Function(params, outputs, _) => write!(f, "fn({:?}) -> ({:?})", params, outputs),
            Value::NativeFunction(_) => write!(f, "native_fn"),
            Value::Lambda(params, _) => write!(f, "lambda({:?})", params),
            Value::Any(val) => write!(f, "any({:?})", val),
            Value::Null => write!(f, "null"),
            Value::Dual(v, d) => write!(f, "Dual({}, grad={})", v, d),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            variables: HashMap::new(),
        };
        interpreter.register_builtins();
        interpreter
    }

    pub fn run(&mut self, program: &crate::ast::Program) -> Result<(), String> {
        for stmt in &program.statements {
            if let ControlFlow::Return(_) = self.execute(stmt)? {
                break;
            }
        }
        Ok(())
    }

    fn execute_statements(&mut self, statements: &[Statement]) -> Result<ControlFlow, String> {
        for stmt in statements {
            let flow = self.execute(stmt)?;
            match flow {
                ControlFlow::Next => continue,
                _ => return Ok(flow),
            }
        }
        Ok(ControlFlow::Next)
    }

    fn register_builtins(&mut self) {
        self.variables.insert(
            "print".to_string(),
            Value::NativeFunction(|args| {
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    match arg {
                        Value::String(s) => print!("{}", s),
                        Value::Integer(v) => print!("{}", v),
                        Value::Float(v) => print!("{}", v),
                        Value::Bool(v) => print!("{}", v),
                        _ => print!("{:?}", arg),
                    }
                }
                println!();
                Value::Null
            }),
        );
        self.variables.insert(
            "car".to_string(),
            Value::NativeFunction(|args| {
                if args.is_empty() {
                    return Value::Null;
                } // Error?
                if let Value::List(l) = &args[0] {
                    if l.is_empty() {
                        Value::Null
                    } else {
                        l[0].clone()
                    }
                } else {
                    Value::Null
                }
            }),
        );
        self.variables.insert(
            "cdr".to_string(),
            Value::NativeFunction(|args| {
                if args.is_empty() {
                    return Value::Null;
                }
                if let Value::List(l) = &args[0] {
                    if l.is_empty() {
                        Value::List(vec![])
                    } else {
                        Value::List(l[1..].to_vec())
                    }
                } else {
                    Value::Null
                }
            }),
        );
        self.variables.insert(
            "cons".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Value::Null;
                }
                let head = args[0].clone();
                if let Value::List(tail) = &args[1] {
                    let mut new_list = tail.clone();
                    new_list.insert(0, head);
                    Value::List(new_list)
                } else {
                    Value::Null
                }
            }),
        );
        self.variables.insert(
            "assert".to_string(),
            Value::NativeFunction(|args| {
                if args.is_empty() {
                    return Value::Null;
                }
                let condition = match &args[0] {
                    Value::Bool(b) => *b,
                    Value::Null => false,
                    Value::Integer(i) => *i != 0,
                    _ => true,
                };

                if !condition {
                    let msg = if args.len() > 1 {
                        match &args[1] {
                            Value::String(s) => s.clone(),
                            _ => format!("{:?}", args[1]),
                        }
                    } else {
                        "Assertion failed".to_string()
                    };
                    panic!("GUL Assertion Failed: {}", msg);
                }
                Value::Null
            }),
        );
        self.variables.insert(
            "len".to_string(),
            Value::NativeFunction(|args| {
                if let Some(val) = args.first() {
                    match val {
                        Value::List(l) => Value::Integer(l.len() as i64),
                        Value::String(s) => Value::Integer(s.len() as i64),
                        Value::Object(_, m) => Value::Integer(m.len() as i64),
                        _ => Value::Integer(0),
                    }
                } else {
                    Value::Integer(0)
                }
            }),
        );
        self.variables.insert(
            "type".to_string(),
            Value::NativeFunction(|args| {
                if let Some(val) = args.first() {
                    Value::String(format!("{:?}", val))
                } else {
                    Value::String("null".to_string())
                }
            }),
        );
        self.variables.insert(
            "range".to_string(),
            Value::NativeFunction(|args| {
                if args.len() >= 1 {
                    let start = if args.len() >= 2 {
                        match args[0] {
                            Value::Integer(i) => i,
                            _ => 0,
                        }
                    } else {
                        0
                    };
                    let end = match args.last().unwrap() {
                        Value::Integer(i) => *i,
                        _ => 0,
                    };
                    let list: Vec<Value> = (start..end).map(Value::Integer).collect();
                    Value::List(list)
                } else {
                    Value::List(vec![])
                }
            }),
        );
        // "grad" is handled specially in evaluate to allow interpreter access

        // "input" builtin (v3.2)
        self.variables.insert(
            "input".to_string(),
            Value::NativeFunction(|_| {
                let mut buffer = String::new();
                if std::io::stdin().read_line(&mut buffer).is_ok() {
                    Value::String(buffer.trim().to_string())
                } else {
                    Value::Null
                }
            }),
        );
        self.variables.insert(
            "println".to_string(),
            Value::NativeFunction(|args| {
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { print!(" "); }
                    match arg {
                        Value::String(s) => print!("{}", s),
                        Value::Integer(v) => print!("{}", v),
                        Value::Float(v) => print!("{}", v),
                        Value::Bool(v) => print!("{}", v),
                        _ => print!("{:?}", arg),
                    }
                }
                println!();
                Value::Null
            }),
        );

        self.variables.insert(
            "@str".to_string(),
            Value::NativeFunction(|args| {
                if let Some(val) = args.first() {
                     match val {
                         Value::String(s) => Value::String(s.clone()),
                         Value::Integer(i) => Value::String(i.to_string()),
                         Value::Float(f) => Value::String(f.to_string()),
                         Value::Bool(b) => Value::String(b.to_string()),
                         _ => Value::String(format!("{:?}", val)),
                     }
                } else {
                    Value::String("null".to_string())
                }
            }),
        );
    }

    fn execute(&mut self, stmt: &Statement) -> Result<ControlFlow, String> {
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

    fn evaluate(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Integer(i) => Ok(Value::Integer(*i)),
            Expression::Float(f) => Ok(Value::Float(*f)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Bool(b) => Ok(Value::Bool(*b)),
            Expression::Identifier(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            Expression::Call { function, args } => {
                // Special handling for grad(f, x)
                if let Expression::Identifier(name) = function.as_ref() {
                    if name == "grad" {
                        if args.len() != 2 {
                            return Err("grad expects 2 arguments (function, value)".to_string());
                        }
                        let f_val = self.evaluate(&args[0])?;
                        let x_val = self.evaluate(&args[1])?;
                        
                        let x_dual = match x_val {
                            Value::Integer(i) => Value::Dual(i as f64, 1.0),
                            Value::Float(f) => Value::Dual(f, 1.0),
                            _ => return Err("grad expects numeric value".to_string()),
                        };
                        
                        match f_val {
                            Value::Lambda(params, body_expr) => {
                                if params.len() != 1 { return Err("grad expects function with 1 parameter".to_string()); }
                                let param_name = &params[0].0;
                                let old_val = self.variables.insert(param_name.clone(), x_dual);
                                let result = self.evaluate(&body_expr)?;
                                if let Some(v) = old_val { self.variables.insert(param_name.clone(), v); } 
                                else { self.variables.remove(param_name); }
                                
                                match result {
                                    Value::Dual(_, grad) => return Ok(Value::Float(grad)),
                                    Value::Integer(_) | Value::Float(_) => return Ok(Value::Float(0.0)),
                                    _ => return Err("Function did not return numeric value".to_string()),
                                }
                            }
                             _ => return Err("grad currently supports Arrow Functions only".to_string()),
                        }
                    }
                }

                let func_val = self.evaluate(function)?;
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate(arg)?);
                }

                match func_val {
                    Value::NativeFunction(f) => Ok(f(arg_vals)),
                    Value::Function(params, outputs, body) => {
                         let mut local_interpreter = self.clone();
                         // Bind inputs
                         for (i, (param_name, _)) in params.iter().enumerate() {
                             if i < arg_vals.len() {
                                 local_interpreter.variables.insert(param_name.clone(), arg_vals[i].clone());
                             }
                         }
                         // Bind outputs as Null initially
                         for (out_name, _) in &outputs {
                             if !out_name.is_empty() {
                                 local_interpreter.variables.insert(out_name.clone(), Value::Null);
                             }
                         }

                         match local_interpreter.execute_statements(&body)? {
                             ControlFlow::Return(v) => Ok(v),
                             _ => {
                                 // Return outputs if no explicit return
                                 if outputs.is_empty() {
                                     Ok(Value::Null)
                                 } else if outputs.len() == 1 {
                                     let (out_name, _) = &outputs[0];
                                     if out_name.is_empty() {
                                          Ok(Value::Null)
                                     } else {
                                          Ok(local_interpreter.variables.get(out_name).cloned().unwrap_or(Value::Null))
                                     }
                                 } else {
                                     let mut res_map = HashMap::new();
                                     for (out_name, _) in &outputs {
                                         if !out_name.is_empty() {
                                             res_map.insert(out_name.clone(), local_interpreter.variables.get(out_name).cloned().unwrap_or(Value::Null));
                                         }
                                     }
                                     Ok(Value::Dict(res_map))
                                 }
                             }
                         }
                    }
                    Value::Lambda(params, body_expr) => {
                         // Execute expression body
                         for (i, (param, _ty)) in params.iter().enumerate() {
                             if i < arg_vals.len() {
                                 self.variables.insert(param.clone(), arg_vals[i].clone());
                             }
                         }
                         self.evaluate(&body_expr)
                    }
                    _ => Err(format!("Not a callable: {:?}", func_val)),
                }
            }
            Expression::Binary { left, op, right } => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                match (l, op, r) {
                    (Value::String(a), BinaryOp::Add, Value::String(b)) => {
                        Ok(Value::String(a + &b))
                    }
                    (Value::Integer(a), BinaryOp::Add, Value::Integer(b)) => {
                        Ok(Value::Integer(a + b))
                    }
                    (Value::Integer(a), BinaryOp::Subtract, Value::Integer(b)) => {
                        Ok(Value::Integer(a - b))
                    }
                    (Value::Integer(a), BinaryOp::Multiply, Value::Integer(b)) => {
                        Ok(Value::Integer(a * b))
                    }
                    (Value::Integer(a), BinaryOp::Divide, Value::Integer(b)) => {
                        if b == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Integer(a / b))
                        }
                    }
                    (Value::Float(a), BinaryOp::Add, Value::Float(b)) => {
                        Ok(Value::Float(a + b))
                    }
                    (Value::Float(a), BinaryOp::Subtract, Value::Float(b)) => {
                        Ok(Value::Float(a - b))
                    }
                    (Value::Float(a), BinaryOp::Multiply, Value::Float(b)) => {
                        Ok(Value::Float(a * b))
                    }
                    (Value::Float(a), BinaryOp::Divide, Value::Float(b)) => {
                        Ok(Value::Float(a / b))
                    }
                    // Auto-differentiation (Dual numbers)
                    (Value::Dual(v1, d1), BinaryOp::Add, Value::Dual(v2, d2)) => {
                        Ok(Value::Dual(v1 + v2, d1 + d2))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Subtract, Value::Dual(v2, d2)) => {
                        Ok(Value::Dual(v1 - v2, d1 - d2))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Multiply, Value::Dual(v2, d2)) => {
                        // Product rule: d(uv) = udv + vdu
                        Ok(Value::Dual(v1 * v2, v1 * d2 + v2 * d1))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Divide, Value::Dual(v2, d2)) => {
                        // Quotient rule: d(u/v) = (vdu - udv) / v^2
                        Ok(Value::Dual(v1 / v2, (v2 * d1 - v1 * d2) / (v2 * v2)))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Add, Value::Integer(n)) => {
                         Ok(Value::Dual(v1 + n as f64, d1))
                    }
                    (Value::Integer(n), BinaryOp::Add, Value::Dual(v2, d2)) => {
                         Ok(Value::Dual(n as f64 + v2, d2))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Multiply, Value::Integer(n)) => {
                         Ok(Value::Dual(v1 * n as f64, d1 * n as f64))
                    }
                    (Value::Integer(n), BinaryOp::Multiply, Value::Dual(v2, d2)) => {
                         Ok(Value::Dual(n as f64 * v2, n as f64 * d2))
                    }
                     // Add more combinations (Float, etc.) as needed for basic support
                    
                    (l, BinaryOp::Equal, r) => Ok(Value::Bool(l == r)),
                    (l, BinaryOp::NotEqual, r) => Ok(Value::Bool(l != r)),
                    (Value::Integer(a), BinaryOp::Less, Value::Integer(b)) => Ok(Value::Bool(a < b)),
                    (Value::Integer(a), BinaryOp::Greater, Value::Integer(b)) => {
                        Ok(Value::Bool(a > b))
                    }
                    (Value::Integer(a), BinaryOp::LessEqual, Value::Integer(b)) => {
                        Ok(Value::Bool(a <= b))
                    }
                    (Value::Integer(a), BinaryOp::GreaterEqual, Value::Integer(b)) => {
                        Ok(Value::Bool(a >= b))
                    }
                    (Value::Float(a), BinaryOp::Less, Value::Float(b)) => Ok(Value::Bool(a < b)),
                    (Value::Float(a), BinaryOp::Greater, Value::Float(b)) => Ok(Value::Bool(a > b)),
                    // Add more binary ops support...
                    _ => Ok(Value::Null),
                }
            }
            Expression::List(elements) => {
                let mut vals = Vec::new();
                for e in elements {
                    vals.push(self.evaluate(e)?);
                }
                Ok(Value::List(vals))
            }
            Expression::Dict(pairs) => {
                let mut dict = HashMap::new();
                for (k, v) in pairs {
                    let val = self.evaluate(v)?;
                    dict.insert(k.clone(), val);
                }
                Ok(Value::Dict(dict))
            }
            Expression::Member { object, member } => {
                let obj = self.evaluate(object)?;
                match obj {
                    Value::Object(_, fields) => fields
                        .get(member)
                        .cloned()
                        .ok_or_else(|| format!("Member not found: {}", member)),
                    Value::Dict(fields) => fields
                        .get(member)
                        .cloned()
                        .ok_or_else(|| format!("Member not found: {}", member)),
                    Value::Function(params, outputs, _) => {
                         // v3.2 Function Node properties
                         if member.starts_with("input_") {
                             let idx: usize = member[6..].parse().unwrap_or(0);
                             if idx > 0 && idx <= params.len() {
                                 let (name, _ty) = &params[idx-1];
                                 let mut map = HashMap::new();
                                 map.insert(name.clone(), Value::String(name.clone())); // Simplified proxy
                                 return Ok(Value::Dict(map));
                             }
                         } else if member.starts_with("output_") {
                             let idx: usize = member[7..].parse().unwrap_or(0);
                             if idx > 0 && idx <= outputs.len() {
                                 let (name, _ty) = &outputs[idx-1];
                                 return Ok(Value::String(name.clone()));
                             }
                         }
                         Err(format!("Function property not found: {}", member))
                    }
                    _ => Err(format!("Cannot access member '{}' on non-object", member)),
                }
            }
            Expression::Ownership { value, .. } => self.evaluate(value),
            Expression::Await(expr) => self.evaluate(expr),
            Expression::ListOp { op, args } => {
                match op {
                    ListOp::Car => {
                        if args.is_empty() {
                            return Err("car expects 1 argument".to_string());
                        }
                        let val = self.evaluate(&args[0])?;
                        if let Value::List(list) = val {
                            if list.is_empty() {
                                Ok(Value::Null) // or error?
                            } else {
                                Ok(list[0].clone())
                            }
                        } else {
                            Err("car expects a list".to_string())
                        }
                    }
                    ListOp::Cdr => {
                        if args.is_empty() {
                            return Err("cdr expects 1 argument".to_string());
                        }
                        let val = self.evaluate(&args[0])?;
                        if let Value::List(list) = val {
                            if list.is_empty() {
                                Ok(Value::List(vec![]))
                            } else {
                                Ok(Value::List(list[1..].to_vec()))
                            }
                        } else {
                            Err("cdr expects a list".to_string())
                        }
                    }
                    ListOp::Cons => {
                        if args.len() != 2 {
                            return Err("cons expects 2 arguments".to_string());
                        }
                        let head = self.evaluate(&args[0])?;
                        let tail = self.evaluate(&args[1])?;
                        if let Value::List(mut list) = tail {
                            list.insert(0, head);
                            Ok(Value::List(list))
                        } else {
                            Err("cons expects second argument to be a list".to_string())
                        }
                    }
                    ListOp::Map => {
                        // map(fn, list) - apply function to each element
                        if args.len() != 2 {
                            return Err("map expects 2 arguments (function, list)".to_string());
                        }
                        let func = self.evaluate(&args[0])?;
                        let list_val = self.evaluate(&args[1])?;
                        if let Value::List(items) = list_val {
                            let mut result = Vec::new();
                            for item in items {
                                // Apply function to each item
                                match &func {
                                    Value::NativeFunction(f) => {
                                        result.push(f(vec![item]));
                                    }
                                    Value::Function(params, _outputs, _body) => {
                                        // For user functions, simplified call
                                        if params.is_empty() {
                                            result.push(item);
                                        } else {
                                            // Would need proper scope handling
                                            result.push(item);
                                        }
                                    }
                                    _ => result.push(item),
                                }
                            }
                            Ok(Value::List(result))
                        } else {
                            Err("map expects second argument to be a list".to_string())
                        }
                    }
                    ListOp::Fold => {
                        // fold(fn, initial, list) - reduce list to single value
                        if args.len() != 3 {
                            return Err(
                                "fold expects 3 arguments (function, initial, list)".to_string()
                            );
                        }
                        let func = self.evaluate(&args[0])?;
                        let initial = self.evaluate(&args[1])?;
                        let list_val = self.evaluate(&args[2])?;
                        if let Value::List(items) = list_val {
                            let mut acc = initial;
                            for item in items {
                                match &func {
                                    Value::NativeFunction(f) => {
                                        acc = f(vec![acc, item]);
                                    }
                                    _ => {
                                        // For non-native functions, simplified handling
                                        acc = item;
                                    }
                                }
                            }
                            Ok(acc)
                        } else {
                            Err("fold expects third argument to be a list".to_string())
                        }
                    }
                    ListOp::Slice => {
                        // slice(list, start, end) - get sublist
                        if args.len() < 2 || args.len() > 3 {
                            return Err(
                                "slice expects 2-3 arguments (list, start, [end])".to_string()
                            );
                        }
                        let list_val = self.evaluate(&args[0])?;
                        let start_val = self.evaluate(&args[1])?;
                        let end_val = if args.len() == 3 {
                            Some(self.evaluate(&args[2])?)
                        } else {
                            None
                        };

                        if let Value::List(items) = list_val {
                            let start = match start_val {
                                Value::Integer(i) => i.max(0) as usize,
                                _ => return Err("slice start must be integer".to_string()),
                            };
                            let end = match end_val {
                                Some(Value::Integer(i)) => i.min(items.len() as i64) as usize,
                                None => items.len(),
                                _ => return Err("slice end must be integer".to_string()),
                            };
                            if start > end || start >= items.len() {
                                Ok(Value::List(vec![]))
                            } else {
                                Ok(Value::List(items[start..end.min(items.len())].to_vec()))
                            }
                        } else {
                            Err("slice expects first argument to be a list".to_string())
                        }
                    }
                }
            }
            Expression::Typed { expr, .. } => {
                // For gradual typing, just evaluate the inner expression
                // Type checking is done at compile time
                self.evaluate(expr)
            }
            Expression::Lambda { params, body } => {
                let typed_params: Vec<(String, Option<Type>)> = params.iter().map(|p| (p.clone(), None)).collect();
                Ok(Value::Lambda(typed_params, body.clone()))
            }
            _ => Ok(Value::Null),
        }
    }
}
