// Standard Library for GUL
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn load_std_module(name: &str) -> Option<Value> {
    match name {
        "std.math" => Some(create_math_module()),
        "std.json" => Some(create_json_module()),
        "std.http" => Some(create_http_module()),
        "std.websocket" => Some(create_websocket_module()),
        "std.tcp" => Some(create_tcp_module()),
        "std.udp" => Some(create_udp_module()),
        "std.bytes" => Some(create_bytes_module()),
        "std.async" => Some(create_async_module()),
        "std.test" => Some(create_test_module()),
        "embedded.batt" => Some(create_batt_module()),
        "embedded.storage" => Some(create_embedded_storage_module()),
        "embedded.micropython" => Some(create_micropython_module()),
        "embedded.arduino" => Some(create_arduino_module()),
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
            fn stringify_value(val: &Value) -> String {
                match val {
                    Value::String(s) => {
                        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
                    }
                    Value::Integer(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "null".to_string(),
                    Value::List(items) => {
                        let parts: Vec<String> = items.iter().map(stringify_value).collect();
                        format!("[{}]", parts.join(", "))
                    }
                    Value::Dict(map) => {
                        let parts: Vec<String> = map
                            .iter()
                            .map(|(k, v)| format!("\"{}\": {}", k, stringify_value(v)))
                            .collect();
                        format!("{{{}}}", parts.join(", "))
                    }
                    Value::Object(name, fields) => {
                        let parts: Vec<String> = fields
                            .iter()
                            .map(|(k, v)| format!("\"{}\": {}", k, stringify_value(v)))
                            .collect();
                        format!("{{\"_type\": \"{}\", {}}}", name, parts.join(", "))
                    }
                    Value::Function(params, _) => format!("\"<function({})>\"", params.join(", ")),
                    Value::NativeFunction(_) => "\"<native_function>\"".to_string(),
                    Value::Any(inner) => stringify_value(inner),
                }
            }

            if let Some(val) = args.first() {
                Value::String(stringify_value(val))
            } else {
                Value::String("null".to_string())
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

// WebSocket Module
fn create_websocket_module() -> Value {
    let mut module = HashMap::new();
    module.insert(
        "connect".to_string(),
        Value::String("websocket.connect(url: String) -> Connection".to_string()),
    );
    module.insert(
        "send".to_string(),
        Value::String("websocket.send(conn: Connection, message: String) -> Result".to_string()),
    );
    module.insert(
        "receive".to_string(),
        Value::String("websocket.receive(conn: Connection) -> String".to_string()),
    );
    module.insert(
        "close".to_string(),
        Value::String("websocket.close(conn: Connection) -> Result".to_string()),
    );
    module.insert(
        "listen".to_string(),
        Value::String("websocket.listen(address: String, port: Number) -> Server".to_string()),
    );
    module.insert(
        "accept".to_string(),
        Value::String("websocket.accept(server: Server) -> Connection".to_string()),
    );
    module.insert(
        "ping".to_string(),
        Value::String("websocket.ping(conn: Connection) -> Result".to_string()),
    );
    module.insert(
        "pong".to_string(),
        Value::String("websocket.pong(conn: Connection) -> Result".to_string()),
    );
    module.insert(
        "is_open".to_string(),
        Value::String("websocket.is_open(conn: Connection) -> Boolean".to_string()),
    );
    Value::Object("std.websocket".to_string(), module)
}

// TCP Module
fn create_tcp_module() -> Value {
    let mut module = HashMap::new();
    module.insert(
        "connect".to_string(),
        Value::String("tcp.connect(address: String, port: Number) -> Socket".to_string()),
    );
    module.insert(
        "send".to_string(),
        Value::String("tcp.send(socket: Socket, data: String) -> Number".to_string()),
    );
    module.insert(
        "receive".to_string(),
        Value::String("tcp.receive(socket: Socket, size: Number) -> String".to_string()),
    );
    module.insert(
        "close".to_string(),
        Value::String("tcp.close(socket: Socket) -> Result".to_string()),
    );
    module.insert(
        "listen".to_string(),
        Value::String("tcp.listen(address: String, port: Number) -> Listener".to_string()),
    );
    module.insert(
        "accept".to_string(),
        Value::String("tcp.accept(listener: Listener) -> Socket".to_string()),
    );
    module.insert(
        "bind".to_string(),
        Value::String("tcp.bind(address: String, port: Number) -> Listener".to_string()),
    );
    module.insert(
        "set_nodelay".to_string(),
        Value::String("tcp.set_nodelay(socket: Socket, enabled: Boolean) -> Result".to_string()),
    );
    module.insert(
        "set_keepalive".to_string(),
        Value::String("tcp.set_keepalive(socket: Socket, duration: Number) -> Result".to_string()),
    );
    module.insert(
        "set_timeout".to_string(),
        Value::String("tcp.set_timeout(socket: Socket, seconds: Number) -> Result".to_string()),
    );
    module.insert(
        "local_addr".to_string(),
        Value::String("tcp.local_addr(socket: Socket) -> String".to_string()),
    );
    module.insert(
        "peer_addr".to_string(),
        Value::String("tcp.peer_addr(socket: Socket) -> String".to_string()),
    );
    module.insert(
        "shutdown".to_string(),
        Value::String("tcp.shutdown(socket: Socket, how: String) -> Result".to_string()),
    );
    Value::Object("std.tcp".to_string(), module)
}

// UDP Module
fn create_udp_module() -> Value {
    let mut module = HashMap::new();
    module.insert(
        "bind".to_string(),
        Value::String("udp.bind(address: String, port: Number) -> Socket".to_string()),
    );
    module.insert(
        "send_to".to_string(),
        Value::String(
            "udp.send_to(socket: Socket, data: String, address: String, port: Number) -> Number"
                .to_string(),
        ),
    );
    module.insert("receive_from".to_string(), Value::String("udp.receive_from(socket: Socket, size: Number) -> {data: String, address: String, port: Number}".to_string()));
    module.insert(
        "close".to_string(),
        Value::String("udp.close(socket: Socket) -> Result".to_string()),
    );
    module.insert(
        "connect".to_string(),
        Value::String(
            "udp.connect(socket: Socket, address: String, port: Number) -> Result".to_string(),
        ),
    );
    module.insert(
        "send".to_string(),
        Value::String("udp.send(socket: Socket, data: String) -> Number".to_string()),
    );
    module.insert(
        "receive".to_string(),
        Value::String("udp.receive(socket: Socket, size: Number) -> String".to_string()),
    );
    module.insert(
        "set_broadcast".to_string(),
        Value::String("udp.set_broadcast(socket: Socket, enabled: Boolean) -> Result".to_string()),
    );
    module.insert(
        "set_multicast_loop".to_string(),
        Value::String(
            "udp.set_multicast_loop(socket: Socket, enabled: Boolean) -> Result".to_string(),
        ),
    );
    module.insert(
        "join_multicast".to_string(),
        Value::String(
            "udp.join_multicast(socket: Socket, multicast_addr: String) -> Result".to_string(),
        ),
    );
    module.insert(
        "leave_multicast".to_string(),
        Value::String(
            "udp.leave_multicast(socket: Socket, multicast_addr: String) -> Result".to_string(),
        ),
    );
    module.insert(
        "set_ttl".to_string(),
        Value::String("udp.set_ttl(socket: Socket, ttl: Number) -> Result".to_string()),
    );
    module.insert(
        "local_addr".to_string(),
        Value::String("udp.local_addr(socket: Socket) -> String".to_string()),
    );
    module.insert(
        "peer_addr".to_string(),
        Value::String("udp.peer_addr(socket: Socket) -> String".to_string()),
    );
    Value::Object("std.udp".to_string(), module)
}

// Bytes Module
fn create_bytes_module() -> Value {
    let mut module = HashMap::new();

    // Binary data operations
    module.insert(
        "from_string".to_string(),
        Value::String("bytes.from_string(s: String) -> Bytes".to_string()),
    );
    module.insert(
        "to_string".to_string(),
        Value::String("bytes.to_string(b: Bytes) -> String".to_string()),
    );
    module.insert(
        "from_hex".to_string(),
        Value::String("bytes.from_hex(hex: String) -> Bytes".to_string()),
    );
    module.insert(
        "to_hex".to_string(),
        Value::String("bytes.to_hex(b: Bytes) -> String".to_string()),
    );
    module.insert(
        "from_base64".to_string(),
        Value::String("bytes.from_base64(b64: String) -> Bytes".to_string()),
    );
    module.insert(
        "to_base64".to_string(),
        Value::String("bytes.to_base64(b: Bytes) -> String".to_string()),
    );
    module.insert(
        "concat".to_string(),
        Value::String("bytes.concat(b1: Bytes, b2: Bytes) -> Bytes".to_string()),
    );
    module.insert(
        "slice".to_string(),
        Value::String("bytes.slice(b: Bytes, start: Number, end: Number) -> Bytes".to_string()),
    );
    module.insert(
        "length".to_string(),
        Value::String("bytes.length(b: Bytes) -> Number".to_string()),
    );
    module.insert(
        "equals".to_string(),
        Value::String("bytes.equals(b1: Bytes, b2: Bytes) -> Boolean".to_string()),
    );

    Value::Object("std.bytes".to_string(), module)
}

// Async Module
fn create_async_module() -> Value {
    let mut module = HashMap::new();

    // spawn(fn) -> Task
    module.insert(
        "spawn".to_string(),
        Value::NativeFunction(|args| {
            if let Some(func) = args.first() {
                let mut task = HashMap::new();
                task.insert("id".to_string(), Value::Integer(1));
                task.insert("function".to_string(), func.clone());
                Value::Object("Task".to_string(), task)
            } else {
                Value::Null
            }
        }),
    );

    // join(task) -> Result
    module.insert(
        "join".to_string(),
        Value::NativeFunction(|_args| Value::Bool(true)),
    );

    // join_all(tasks)
    module.insert(
        "join_all".to_string(),
        Value::NativeFunction(|_args| Value::List(vec![Value::Bool(true)])),
    );

    // sleep(ms)
    module.insert(
        "sleep".to_string(),
        Value::NativeFunction(|args| {
            if let Some(Value::Integer(ms)) = args.first() {
                std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
            }
            Value::Null
        }),
    );

    Value::Object("std.async".to_string(), module)
}
fn create_micropython_module() -> Value {
    let mut module = HashMap::new();
    module.insert(
        "run".to_string(),
        Value::NativeFunction(|args| {
            if let Some(Value::String(s)) = args.first() {
                println!("MicroPython Exec: {}", s);
            }
            Value::Null
        }),
    );
    Value::Object("embedded.micropython".to_string(), module)
}

fn create_arduino_module() -> Value {
    let mut module = HashMap::new();
    module.insert(
        "delay".to_string(),
        Value::NativeFunction(|args| {
            if let Some(Value::Integer(ms)) = args.first() {
                std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
            }
            Value::Null
        }),
    );
    module.insert(
        "digitalWrite".to_string(),
        Value::NativeFunction(|_| {
            println!("Arduino: digitalWrite");
            Value::Null
        }),
    );
    Value::Object("embedded.arduino".to_string(), module)
}

// ... existing math/json/http modules ...

fn create_batt_module() -> Value {
    let mut module = HashMap::new();
    module.insert(
        "Battery".to_string(),
        Value::NativeFunction(|_| Value::Object("Battery".to_string(), HashMap::new())),
    );
    Value::Object("embedded.batt".to_string(), module)
}

fn create_embedded_storage_module() -> Value {
    let mut module = HashMap::new();
    module.insert(
        "NvsStorage".to_string(),
        Value::NativeFunction(|_| Value::Object("NvsStorage".to_string(), HashMap::new())),
    );
    Value::Object("embedded.storage".to_string(), module)
}

fn create_test_module() -> Value {
    let mut module = HashMap::new();

    // std.test.suite(name)
    module.insert(
        "suite".to_string(),
        Value::NativeFunction(|args| {
            if let Some(Value::String(name)) = args.first() {
                println!("Running test suite: {}", name);
            }
            Value::Null
        }),
    );

    // std.test.assert(cond, msg) - complementary to builtin assert
    module.insert(
        "assert".to_string(),
        Value::NativeFunction(|args| {
            // Re-use logic or just let the builtin handle it if users call it as assert()
            // Here we can provide more formatted output if we want.
            if args.is_empty() {
                return Value::Null;
            }
            let condition = match &args[0] {
                Value::Bool(b) => *b,
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
                eprintln!("TEST FAILURE: {}", msg);
                panic!("test_assert_fail");
            }
            Value::Null
        }),
    );

    Value::Object("std.test".to_string(), module)
}
