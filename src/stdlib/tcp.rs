// TCP Module for GUL Standard Library
use crate::ast::Value;
use std::collections::HashMap;

pub fn load_tcp_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // TCP client functions
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

    // TCP server functions
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

    // TCP socket options
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

    // TCP utilities
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

    module
}
