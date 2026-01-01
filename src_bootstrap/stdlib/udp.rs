// UDP Module for GUL Standard Library
use crate::ast::Value;
use std::collections::HashMap;

pub fn load_udp_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();

    // UDP socket functions
    module.insert(
        "bind".to_string(),
        Value::String("udp.bind(address: String, port: Number) -> Socket".to_string()),
    );
    module.insert(
        "send_to".to_string(),
        Value::String("udp.send_to(socket: Socket, data: String, address: String, port: Number) -> Number".to_string()),
    );
    module.insert(
        "receive_from".to_string(),
        Value::String("udp.receive_from(socket: Socket, size: Number) -> {data: String, address: String, port: Number}".to_string()),
    );
    module.insert(
        "close".to_string(),
        Value::String("udp.close(socket: Socket) -> Result".to_string()),
    );

    // UDP connection functions (for connected UDP)
    module.insert(
        "connect".to_string(),
        Value::String("udp.connect(socket: Socket, address: String, port: Number) -> Result".to_string()),
    );
    module.insert(
        "send".to_string(),
        Value::String("udp.send(socket: Socket, data: String) -> Number".to_string()),
    );
    module.insert(
        "receive".to_string(),
        Value::String("udp.receive(socket: Socket, size: Number) -> String".to_string()),
    );

    // UDP socket options
    module.insert(
        "set_broadcast".to_string(),
        Value::String("udp.set_broadcast(socket: Socket, enabled: Boolean) -> Result".to_string()),
    );
    module.insert(
        "set_multicast_loop".to_string(),
        Value::String("udp.set_multicast_loop(socket: Socket, enabled: Boolean) -> Result".to_string()),
    );
    module.insert(
        "join_multicast".to_string(),
        Value::String("udp.join_multicast(socket: Socket, multicast_addr: String) -> Result".to_string()),
    );
    module.insert(
        "leave_multicast".to_string(),
        Value::String("udp.leave_multicast(socket: Socket, multicast_addr: String) -> Result".to_string()),
    );
    module.insert(
        "set_ttl".to_string(),
        Value::String("udp.set_ttl(socket: Socket, ttl: Number) -> Result".to_string()),
    );

    // UDP utilities
    module.insert(
        "local_addr".to_string(),
        Value::String("udp.local_addr(socket: Socket) -> String".to_string()),
    );
    module.insert(
        "peer_addr".to_string(),
        Value::String("udp.peer_addr(socket: Socket) -> String".to_string()),
    );

    module
}
