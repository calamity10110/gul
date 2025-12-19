// Data-flow graph validator
// Validates that all contracts are satisfied

use super::{DataFlowGraph, contracts::ContractValidator};
use crate::ast::PortRef;

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub source: Option<PortRef>,
    pub target: Option<PortRef>,
}

/// Graph validator
pub struct GraphValidator {
    contract_validator: ContractValidator,
}

impl GraphValidator {
    pub fn new() -> Self {
        GraphValidator {
            contract_validator: ContractValidator::new(),
        }
    }

    /// Validate entire data-flow graph
    pub fn validate(&self, graph: &DataFlowGraph) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Check all required inputs are connected
        for node in &graph.nodes {
            let required_ports = &node.declaration.required_inputs.params;
            for param in required_ports {
                let port_ref = PortRef::new(&node.name, &param.name);
                if !self.is_input_connected(graph, &port_ref) {
                    errors.push(ValidationError {
                        message: format!(
                            "Required input '{}:{}' is not connected",
                            node.name, param.name
                        ),
                        source: None,
                        target: Some(port_ref),
                    });
                }
            }
        }

        // Check all required outputs are connected
        for node in &graph.nodes {
            let required_ports = &node.declaration.required_outputs.params;
            for param in required_ports {
                let port_ref = PortRef::new(&node.name, &param.name);
                if !self.is_output_connected(graph, &port_ref) {
                    errors.push(ValidationError {
                        message: format!(
                            "Required output '{}:{}' is not connected",
                            node.name, param.name
                        ),
                        source: Some(port_ref),
                        target: None,
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Check if input port is connected
    fn is_input_connected(&self, graph: &DataFlowGraph, port: &PortRef) -> bool {
        // Check connections
        for conn in &graph.connections {
            if conn.target.node_name == port.node_name && conn.target.port_name == port.port_name {
                return true;
            }
        }

        // Check external inputs
        for (_, target) in &graph.external_inputs {
            if target.node_name == port.node_name && target.port_name == port.port_name {
                return true;
            }
        }

        false
    }

    /// Check if output port is connected
    fn is_output_connected(&self, graph: &DataFlowGraph, port: &PortRef) -> bool {
        // Check connections
        for conn in &graph.connections {
            if conn.source.node_name == port.node_name && conn.source.port_name == port.port_name {
                return true;
            }
        }

        // Check if connected to output
        for output in &graph.outputs {
            if output.node_name == port.node_name && output.port_name == port.port_name {
                return true;
            }
        }

        false
    }
}

impl Default for GraphValidator {
    fn default() -> Self {
        Self::new()
    }
}
