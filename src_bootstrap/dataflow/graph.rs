// Data-flow graph builder
// Constructs a data-flow graph from parsed AST

use super::DataFlowGraph;
use crate::ast::{DataFlowBlock, NodeDeclaration};

/// Graph builder for constructing data-flow graphs
pub struct GraphBuilder {
    /// Node declarations from .def files
    declarations: Vec<NodeDeclaration>,
    /// Current graph being built
    graph: DataFlowGraph,
}

impl GraphBuilder {
    /// Create a new graph builder
    pub fn new() -> Self {
        GraphBuilder {
            declarations: Vec::new(),
            graph: DataFlowGraph::new(),
        }
    }

    /// Register a node declaration
    pub fn register_node(&mut self, decl: NodeDeclaration) {
        self.declarations.push(decl);
    }

    /// Get node declaration by name
    pub fn get_declaration(&self, name: &str) -> Option<&NodeDeclaration> {
        self.declarations.iter().find(|d| d.name == name)
    }

    /// Build graph from data-flow block
    pub fn build(&mut self, block: &DataFlowBlock) -> Result<DataFlowGraph, String> {
        // Add external inputs
        for (input, target) in &block.external_inputs {
            self.graph.add_external_input(input.clone(), target.clone());
        }

        // Add connections
        for conn in &block.connections {
            self.graph
                .add_connection(conn.source.clone(), conn.target.clone());
        }

        // Add outputs
        for output in &block.outputs {
            self.graph.add_output(output.clone());
        }

        Ok(self.graph.clone())
    }

    /// Instantiate a node from declaration
    pub fn instantiate_node(&mut self, name: &str, instance_name: &str) -> Result<usize, String> {
        let decl = self
            .get_declaration(name)
            .ok_or_else(|| format!("Node '{}' not found", name))?
            .clone();

        Ok(self.graph.add_node(instance_name, decl))
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}
