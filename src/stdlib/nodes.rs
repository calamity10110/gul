// GUL 101 Standard Library Node Contracts
// Per Appendix F

use crate::ast::{Ownership, Type};
use crate::dataflow::ir::{IRNode, IRPort};

/// Create print node contract (F.1)
pub fn print_node() -> IRNode {
    IRNode::new(0, "print")
        .with_input(IRPort::new("value", Type::Any, Ownership::Ref))
}

/// Create input node contract (F.1)
pub fn input_node() -> IRNode {
    IRNode::new(0, "input")
        .with_output(IRPort::new("value", Type::Any, Ownership::Gives))
}

/// Create add node contract (F.2)
pub fn add_node() -> IRNode {
    IRNode::new(0, "add")
        .with_input(IRPort::new("a", Type::Int, Ownership::Borrow))
        .with_input(IRPort::new("b", Type::Int, Ownership::Borrow))
        .with_output(IRPort::new("result", Type::Int, Ownership::Own))
}

/// Create multiply node contract (F.2)
pub fn multiply_node() -> IRNode {
    IRNode::new(0, "multiply")
        .with_input(IRPort::new("a", Type::Int, Ownership::Borrow))
        .with_input(IRPort::new("b", Type::Int, Ownership::Borrow))
        .with_output(IRPort::new("result", Type::Int, Ownership::Own))
}

/// Create subtract node contract
pub fn subtract_node() -> IRNode {
    IRNode::new(0, "subtract")
        .with_input(IRPort::new("a", Type::Int, Ownership::Borrow))
        .with_input(IRPort::new("b", Type::Int, Ownership::Borrow))
        .with_output(IRPort::new("result", Type::Int, Ownership::Own))
}

/// Create divide node contract
pub fn divide_node() -> IRNode {
    IRNode::new(0, "divide")
        .with_input(IRPort::new("a", Type::Int, Ownership::Borrow))
        .with_input(IRPort::new("b", Type::Int, Ownership::Borrow))
        .with_output(IRPort::new("result", Type::Float, Ownership::Own))
}

/// Create list_push node contract (F.3)
pub fn list_push_node() -> IRNode {
    IRNode::new(0, "list_push")
        .with_input(IRPort::new("list", Type::List(Box::new(Type::Any)), Ownership::Borrow))
        .with_input(IRPort::new("value", Type::Any, Ownership::Borrow))
}

/// Create http_get node contract (F.4)
pub fn http_get_node() -> IRNode {
    IRNode::new(0, "http_get")
        .with_input(IRPort::new("url", Type::String, Ownership::Ref))
        .with_output(IRPort::new("response", Type::String, Ownership::Own))
        .with_trait("async")
}

/// Registry of all standard library nodes
pub struct StdlibNodes {
    nodes: Vec<IRNode>,
}

impl StdlibNodes {
    pub fn new() -> Self {
        StdlibNodes {
            nodes: vec![
                print_node(),
                input_node(),
                add_node(),
                multiply_node(),
                subtract_node(),
                divide_node(),
                list_push_node(),
                http_get_node(),
            ],
        }
    }

    /// Get node by name
    pub fn get(&self, name: &str) -> Option<&IRNode> {
        self.nodes.iter().find(|n| n.name == name)
    }

    /// Get all nodes
    pub fn all(&self) -> &[IRNode] {
        &self.nodes
    }
}

impl Default for StdlibNodes {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_node_contract() {
        let node = print_node();
        assert_eq!(node.name, "print");
        assert_eq!(node.inputs.len(), 1);
        assert_eq!(node.inputs[0].name, "value");
        assert_eq!(node.inputs[0].ownership, Ownership::Ref);
    }

    #[test]
    fn test_add_node_contract() {
        let node = add_node();
        assert_eq!(node.name, "add");
        assert_eq!(node.inputs.len(), 2);
        assert_eq!(node.outputs.len(), 1);
        assert_eq!(node.outputs[0].ownership, Ownership::Own);
    }

    #[test]
    fn test_stdlib_registry() {
        let stdlib = StdlibNodes::new();
        assert!(stdlib.get("print").is_some());
        assert!(stdlib.get("add").is_some());
        assert!(stdlib.get("unknown").is_none());
    }
}
