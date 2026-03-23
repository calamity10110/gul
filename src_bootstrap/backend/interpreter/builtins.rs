use super::values::Value;
use super::Interpreter;
use std::collections::HashMap;

impl Interpreter {
    pub(crate) fn register_builtins(&mut self) {
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
}
