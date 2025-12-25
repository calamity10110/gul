use crate::ast::*;
use std::collections::HashMap;
use std::fmt;

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
    Function(Vec<String>, Vec<Statement>),
    NativeFunction(fn(Vec<Value>) -> Value),
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
            (Value::Function(p1, b1), Value::Function(p2, b2)) => p1 == p2 && b1 == b2,
            (Value::NativeFunction(_), Value::NativeFunction(_)) => false, // Can't compare fn ptrs
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
            Value::Function(params, _) => write!(f, "fn({:?})", params),
            Value::NativeFunction(_) => write!(f, "native_fn"),
            Value::Any(val) => write!(f, "any({:?})", val),
            Value::Null => write!(f, "null"),
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
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute_statements(&mut self, statements: &[Statement]) -> Result<(), String> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
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
    }

    fn execute(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Statement::Definition { name, value } => {
                let val = self.evaluate(value)?;
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Statement::Function {
                name, params, body, ..
            } => {
                let val = Value::Function(params.clone(), body.clone());
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Statement::StructDef { name, .. } => {
                // Just register name as placeholder construct for now,
                // or handle struct instantiation logic later.
                self.variables.insert(name.clone(), Value::Null); // Minimal
                Ok(())
            }
            Statement::Import(name) => {
                if let Some(module) = crate::stdlib::load_std_module(name) {
                    // Register module. For "std.math", we might want to register as "math" or "std"
                    // Ideally parser handles "use std.math as m".
                    // For now, simpler: Import "std.math" -> registers "math" variable?
                    // Or just registers the full name if we support dotted access lookup (which we don't fully yet).
                    // Let's split by dot and take list part: "std.math" -> "math"

                    let parts: Vec<&str> = name.split('.').collect();
                    if let Some(short_name) = parts.last() {
                        self.variables.insert(short_name.to_string(), module);
                    }
                }
                Ok(())
            }
            Statement::Main { body } => {
                for s in body {
                    self.execute(s)?;
                }
                Ok(())
            }
            Statement::Assignment { name, value } => {
                let val = self.evaluate(value)?;
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Statement::GlobalDef { name, value, .. } => {
                let val = self.evaluate(value)?;
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Statement::ForeignBlock { language, .. } => {
                println!("Executing foreign {} block (mock)", language);
                Ok(())
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
                        Ok(())
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
            _ => Ok(()),
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
                let func_val = self.evaluate(function)?;
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate(arg)?);
                }

                match func_val {
                    Value::NativeFunction(f) => Ok(f(arg_vals)),
                    Value::Function(_params, _body) => {
                        // Create scope
                        // This naive implementation just overwrites variables, no scope stack yet!
                        // Needs scope stack for real functions, but good enough for flat scripts.
                        Ok(Value::Null)
                    }
                    _ => Err("Not a callable".to_string()),
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
                    _ => Err(format!("Cannot access member '{}' on non-object", member)),
                }
            }
            Expression::Ownership { value, .. } => self.evaluate(value),
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
                    _ => Err("List operation not implemented".to_string()),
                }
            }
            Expression::Typed { expr, .. } => {
                // For gradual typing, just evaluate the inner expression
                // Type checking is done at compile time
                self.evaluate(expr)
            }
            _ => Ok(Value::Null),
        }
    }
}
