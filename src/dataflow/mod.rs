// GUL Data-Flow Engine
// Implements data-flow graph construction, validation, and execution

pub mod contracts;
pub mod executor;
pub mod graph;
pub mod validator;

use crate::ast::{DataFlowConnection, ExternalInput, NodeDeclaration, PortRef};

/// Data-flow graph representing a program
#[derive(Debug, Clone)]
pub struct DataFlowGraph {
    /// All nodes in the graph
    pub nodes: Vec<NodeInstance>,
    /// All connections between nodes
    pub connections: Vec<DataFlowConnection>,
    /// External inputs
    pub external_inputs: Vec<(ExternalInput, PortRef)>,
    /// Output ports (connected to print/output)
    pub outputs: Vec<PortRef>,
}

/// Instance of a node in the graph
#[derive(Debug, Clone)]
pub struct NodeInstance {
    pub name: String,
    pub declaration: NodeDeclaration,
    pub instance_id: usize,
}

impl DataFlowGraph {
    /// Create a new empty data-flow graph
    pub fn new() -> Self {
        DataFlowGraph {
            nodes: Vec::new(),
            connections: Vec::new(),
            external_inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, name: &str, declaration: NodeDeclaration) -> usize {
        let id = self.nodes.len();
        self.nodes.push(NodeInstance {
            name: name.to_string(),
            declaration,
            instance_id: id,
        });
        id
    }

    /// Add a connection between nodes
    pub fn add_connection(&mut self, source: PortRef, target: PortRef) {
        self.connections.push(DataFlowConnection { source, target });
    }

    /// Add external input
    pub fn add_external_input(&mut self, input: ExternalInput, target: PortRef) {
        self.external_inputs.push((input, target));
    }

    /// Add output
    pub fn add_output(&mut self, port: PortRef) {
        self.outputs.push(port);
    }

    /// Get node by name
    pub fn get_node(&self, name: &str) -> Option<&NodeInstance> {
        self.nodes.iter().find(|n| n.name == name)
    }
}

impl Default for DataFlowGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{PortContract, PortParam};

    #[test]
    fn test_dataflow_graph_creation() {
        let graph = DataFlowGraph::new();
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.connections.len(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = DataFlowGraph::new();
        let decl = NodeDeclaration {
            name: "add".to_string(),
            required_inputs: PortContract {
                port_type: TypeAnnotation::new("int"),
                params: vec![
                    PortParam {
                        name: "a".to_string(),
                        label: "A".to_string(),
                    },
                    PortParam {
                        name: "b".to_string(),
                        label: "B".to_string(),
                    },
                ],
            },
            required_outputs: PortContract {
                port_type: TypeAnnotation::new("int"),
                params: vec![PortParam {
                    name: "result".to_string(),
                    label: "Sum".to_string(),
                }],
            },
            optional_inputs: None,
            optional_outputs: None,
        };

        let id = graph.add_node("add1", decl);
        assert_eq!(id, 0);
        assert_eq!(graph.nodes.len(), 1);
    }
}
