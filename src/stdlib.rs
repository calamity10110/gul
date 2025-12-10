use crate::interpreter::Value;
use std::collections::HashMap;

pub fn load_std_module(name: &str) -> Option<Value> {
    match name {
        "std.math" => Some(create_math_module()),
        "std.json" => Some(create_json_module()),
        "std.http" => Some(create_http_module()),
        _ => None,
    }
}

fn create_math_module() -> Value {
    let mut module = HashMap::new();

    module.insert(
        "sqrt".to_string(),
        Value::NativeFunction(|args| {
            if let Some(Value::Float(f)) = args.first() {
                Value::Float(f.sqrt())
            } else if let Some(Value::Integer(i)) = args.first() {
                Value::Float((*i as f64).sqrt())
            } else {
                Value::Null
            }
        }),
    );

    module.insert(
        "abs".to_string(),
        Value::NativeFunction(|args| match args.first() {
            Some(Value::Float(f)) => Value::Float(f.abs()),
            Some(Value::Integer(i)) => Value::Integer(i.abs()),
            _ => Value::Null,
        }),
    );

    module.insert("PI".to_string(), Value::Float(std::f64::consts::PI));

    Value::Object("std.math".to_string(), module)
}

fn create_json_module() -> Value {
    let mut module = HashMap::new();

    module.insert(
        "stringify".to_string(),
        Value::NativeFunction(|args| {
            // Simplified stringify
            if let Some(val) = args.first() {
                match val {
                    Value::String(s) => Value::String(format!("\"{}\"", s)),
                    Value::Integer(i) => Value::String(i.to_string()),
                    Value::Float(f) => Value::String(f.to_string()),
                    Value::Bool(b) => Value::String(b.to_string()),
                    // TODO: Recursive for complex types
                    _ => Value::String("{}".to_string()),
                }
            } else {
                Value::String("".to_string())
            }
        }),
    );

    // Parse is more complex, returning basic implementation for now
    module.insert(
        "parse".to_string(),
        Value::NativeFunction(|_args| Value::Dict(HashMap::new())),
    );

    Value::Object("std.json".to_string(), module)
}

fn create_http_module() -> Value {
    let mut module = HashMap::new();

    // HTTP Client struct placeholder factory
    // In GUL: client = http::Client::new(url)
    // We need to support: http.Client.new(url) -> Returns an Object representing the client

    // Because our basic interpreter doesn't fully support Classes/Static methods well yet,
    // we will mock it by returning a "Client" object that has "get" and "post" methods.

    let mut client_class = HashMap::new();
    client_class.insert(
        "new".to_string(),
        Value::NativeFunction(|_args| {
            // Return a Client instance
            let mut client_instance = HashMap::new();

            client_instance.insert(
                "get".to_string(),
                Value::NativeFunction(|_args| {
                    // Mock request
                    // returns { "status": 200, "json": fn()... }
                    let mut response = HashMap::new();
                    response.insert("status".to_string(), Value::Integer(200));
                    response.insert(
                        "json".to_string(),
                        Value::NativeFunction(|_| {
                            // Mock data
                            let mut data = HashMap::new();
                            data.insert("mock".to_string(), Value::Bool(true));
                            Value::Dict(data)
                        }),
                    );

                    Value::Object("Response".to_string(), response)
                }),
            );

            client_instance.insert(
                "post".to_string(),
                Value::NativeFunction(|_args| {
                    let mut response = HashMap::new();
                    response.insert("status".to_string(), Value::Integer(201));
                    Value::Object("Response".to_string(), response)
                }),
            );

            Value::Object("Client".to_string(), client_instance)
        }),
    );

    module.insert(
        "Client".to_string(),
        Value::Object("ClientClass".to_string(), client_class),
    );

    Value::Object("std.http".to_string(), module)
}
