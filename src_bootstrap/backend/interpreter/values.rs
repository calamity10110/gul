use crate::frontend::ast::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub enum ControlFlow {
    Next,
    Return(Value),
    Break,
    Continue,
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
    Function(
        Vec<(String, Option<Type>)>,
        Vec<(String, Option<Type>)>,
        Vec<Statement>,
    ),
    NativeFunction(fn(Vec<Value>) -> Value),
    Lambda(Vec<(String, Option<Type>)>, Box<Expression>), // Updated Lambda for v3.2
    Dual(f64, f64),                                       // Auto-diff: (value, gradient)
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
            (Value::Function(p1, o1, b1), Value::Function(p2, o2, b2)) => {
                p1 == p2 && o1 == o2 && b1 == b2
            }
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
