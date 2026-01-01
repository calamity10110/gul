// GUL 101 Ownership Checker Implementation

use super::errors::{ErrorCode, OwnershipError};
use crate::ast::Ownership;
use crate::dataflow::ir::{IREdge, IRGraph, NodeId};
use std::collections::{HashMap, HashSet};

/// Ownership checker for IR graphs
pub struct OwnershipChecker {
    /// Track which values have been moved
    moved_values: HashSet<(NodeId, String)>,
    /// Track ownership of each value
    owners: HashMap<(NodeId, String), NodeId>,
    /// Errors found during checking
    errors: Vec<OwnershipError>,
}

impl OwnershipChecker {
    pub fn new() -> Self {
        OwnershipChecker {
            moved_values: HashSet::new(),
            owners: HashMap::new(),
            errors: Vec::new(),
        }
    }

    /// Check an IR graph for ownership violations
    pub fn check(&mut self, graph: &IRGraph) -> Result<(), Vec<OwnershipError>> {
        self.errors.clear();
        self.moved_values.clear();
        self.owners.clear();

        // Check for cycles
        if !graph.is_dag() {
            self.errors.push(OwnershipError::new(ErrorCode::E201));
        }

        // Check each edge for ownership rules
        for edge in &graph.edges {
            self.check_edge(graph, edge);
        }

        // Check mandatory consumption (all own outputs must be used)
        self.check_mandatory_consumption(graph);

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Check a single edge for ownership violations
    fn check_edge(&mut self, graph: &IRGraph, edge: &IREdge) {
        let value_key = (edge.from_node, edge.from_port.clone());

        // Check if ownership is being moved
        if edge.mode.moves_ownership() {
            // E002: Check for multiple ownership-moving edges
            if self.moved_values.contains(&value_key) {
                let node_name = graph
                    .get_node(edge.from_node)
                    .map(|n| n.name.clone())
                    .unwrap_or_default();

                self.errors.push(
                    OwnershipError::new(ErrorCode::E002)
                        .with_node(&node_name)
                        .with_port(&edge.from_port)
                        .with_suggestion("Use 'borrow' or 'ref' instead of 'take'"),
                );
            } else {
                self.moved_values.insert(value_key.clone());
            }
        } else {
            // E003: Check for borrow after move
            if self.moved_values.contains(&value_key) {
                let node_name = graph
                    .get_node(edge.from_node)
                    .map(|n| n.name.clone())
                    .unwrap_or_default();

                self.errors.push(
                    OwnershipError::new(ErrorCode::E003)
                        .with_node(&node_name)
                        .with_port(&edge.from_port)
                        .with_suggestion("Reorder operations or use 'ref' before 'take'"),
                );
            }
        }
    }

    /// Check that all owned outputs are consumed
    fn check_mandatory_consumption(&mut self, graph: &IRGraph) {
        for node in &graph.nodes {
            for output in &node.outputs {
                if matches!(output.ownership, Ownership::Own) {
                    let _value_key = (node.id, output.name.clone());

                    // Check if this output has any outgoing edges
                    let has_consumers = graph
                        .edges
                        .iter()
                        .any(|e| e.from_node == node.id && e.from_port == output.name);

                    if !has_consumers {
                        self.errors.push(
                            OwnershipError::new(ErrorCode::E001)
                                .with_node(&node.name)
                                .with_port(&output.name)
                                .with_suggestion("Connect output to another node or use 'print'"),
                        );
                    }
                }
            }
        }
    }

    /// Get all errors found
    pub fn errors(&self) -> &[OwnershipError] {
        &self.errors
    }
}

impl Default for OwnershipChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Type;
    use crate::dataflow::ir::{IRNode, IRPort};

    #[test]
    fn test_valid_graph() {
        let mut graph = IRGraph::new();

        let input = IRNode::new(0, "input").with_output(IRPort::own("value", Type::Int));

        let printer = IRNode::new(0, "print").with_input(IRPort::reference("value", Type::Int));

        let id1 = graph.add_node(input);
        let id2 = graph.add_node(printer);

        graph.add_edge(IREdge::new(id1, "value", id2, "value", Ownership::Ref));

        let mut checker = OwnershipChecker::new();
        assert!(checker.check(&graph).is_ok());
    }

    #[test]
    fn test_unused_output() {
        let mut graph = IRGraph::new();

        let node = IRNode::new(0, "producer").with_output(IRPort::own("value", Type::Int));

        graph.add_node(node);

        let mut checker = OwnershipChecker::new();
        let result = checker.check(&graph);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err()[0].code, ErrorCode::E001);
    }
}
