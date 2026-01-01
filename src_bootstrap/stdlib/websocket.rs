// WebSocket Module for GUL Standard Library
use crate::ast::Value;
use std::collections::HashMap;

pub fn load_websocket_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // WebSocket connection functions
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

    // WebSocket server functions
    module.insert(
        "listen".to_string(),
        Value::String("websocket.listen(address: String, port: Number) -> Server".to_string()),
    );
    module.insert(
        "accept".to_string(),
        Value::String("websocket.accept(server: Server) -> Connection".to_string()),
    );

    // WebSocket utilities
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

    module
}
