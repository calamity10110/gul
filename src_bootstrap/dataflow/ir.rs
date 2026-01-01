// GUL 101 Intermediate Representation
// Typed, ownership-checked data-flow graph IR

use crate::ast::{Ownership, Type};

/// Unique identifier for nodes
pub type NodeId = usize;

/// Unique identifier for values
pub type ValueId = usize;

/// IR Port - input or output of a node
#[derive(Debug, Clone, PartialEq)]
pub struct IRPort {
    pub name: String,
    pub port_type: Type,
    pub ownership: Ownership,
}

/// IR Node - a computation unit in the graph
#[derive(Debug, Clone)]
pub struct IRNode {
    pub id: NodeId,
    pub name: String,
    pub inputs: Vec<IRPort>,
    pub outputs: Vec<IRPort>,
    pub traits: Vec<String>,
}

/// IR Edge - connection between ports
#[derive(Debug, Clone)]
pub struct IREdge {
    pub from_node: NodeId,
    pub from_port: String,
    pub to_node: NodeId,
    pub to_port: String,
    pub mode: Ownership,
}

/// IR Graph - the complete data-flow graph
#[derive(Debug, Clone)]
pub struct IRGraph {
    pub nodes: Vec<IRNode>,
    pub edges: Vec<IREdge>,
    pub entry_node: Option<NodeId>,
    pub exit_nodes: Vec<NodeId>,
}

impl IRPort {
    pub fn new(name: &str, port_type: Type, ownership: Ownership) -> Self {
        IRPort {
            name: name.to_string(),
            port_type,
            ownership,
        }
    }

    /// Create borrow port (default input)
    pub fn borrow(name: &str, port_type: Type) -> Self {
        IRPort::new(name, port_type, Ownership::Borrow)
    }

    /// Create own port (default output)
    pub fn own(name: &str, port_type: Type) -> Self {
        IRPort::new(name, port_type, Ownership::Own)
    }

    /// Create ref port (shared immutable)
    pub fn reference(name: &str, port_type: Type) -> Self {
        IRPort::new(name, port_type, Ownership::Ref)
    }
}

impl IRNode {
    pub fn new(id: NodeId, name: &str) -> Self {
        IRNode {
            id,
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            traits: Vec::new(),
        }
    }

    pub fn with_input(mut self, port: IRPort) -> Self {
        self.inputs.push(port);
        self
    }

    pub fn with_output(mut self, port: IRPort) -> Self {
        self.outputs.push(port);
        self
    }

    pub fn with_trait(mut self, trait_name: &str) -> Self {
        self.traits.push(trait_name.to_string());
        self
    }

    /// Get input port by name
    pub fn get_input(&self, name: &str) -> Option<&IRPort> {
        self.inputs.iter().find(|p| p.name == name)
    }

    /// Get output port by name
    pub fn get_output(&self, name: &str) -> Option<&IRPort> {
        self.outputs.iter().find(|p| p.name == name)
    }
}

impl IREdge {
    pub fn new(
        from_node: NodeId,
        from_port: &str,
        to_node: NodeId,
        to_port: &str,
        mode: Ownership,
    ) -> Self {
        IREdge {
            from_node,
            from_port: from_port.to_string(),
            to_node,
            to_port: to_port.to_string(),
            mode,
        }
    }

    /// Create borrow edge
    pub fn borrow(from_node: NodeId, from_port: &str, to_node: NodeId, to_port: &str) -> Self {
        IREdge::new(from_node, from_port, to_node, to_port, Ownership::Borrow)
    }

    /// Create take edge (ownership transfer)
    pub fn take(from_node: NodeId, from_port: &str, to_node: NodeId, to_port: &str) -> Self {
        IREdge::new(from_node, from_port, to_node, to_port, Ownership::Take)
    }
}

impl IRGraph {
    pub fn new() -> Self {
        IRGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            entry_node: None,
            exit_nodes: Vec::new(),
        }
    }

    /// Add a node and return its ID
    pub fn add_node(&mut self, mut node: IRNode) -> NodeId {
        let id = self.nodes.len();
        node.id = id;
        self.nodes.push(node);
        id
    }

    /// Add an edge
    pub fn add_edge(&mut self, edge: IREdge) {
        self.edges.push(edge);
    }

    /// Get node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&IRNode> {
        self.nodes.get(id)
    }

    /// Get mutable node by ID
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut IRNode> {
        self.nodes.get_mut(id)
    }

    /// Get all edges from a node
    pub fn edges_from(&self, node_id: NodeId) -> Vec<&IREdge> {
        self.edges
            .iter()
            .filter(|e| e.from_node == node_id)
            .collect()
    }

    /// Get all edges to a node
    pub fn edges_to(&self, node_id: NodeId) -> Vec<&IREdge> {
        self.edges.iter().filter(|e| e.to_node == node_id).collect()
    }

    /// Check if graph is a DAG (no cycles)
    pub fn is_dag(&self) -> bool {
        // Simple cycle detection using DFS
        let mut visited = vec![false; self.nodes.len()];
        let mut in_stack = vec![false; self.nodes.len()];

        for i in 0..self.nodes.len() {
            if !visited[i] && self.has_cycle(i, &mut visited, &mut in_stack) {
                return false;
            }
        }
        true
    }

    fn has_cycle(&self, node: NodeId, visited: &mut Vec<bool>, in_stack: &mut Vec<bool>) -> bool {
        visited[node] = true;
        in_stack[node] = true;

        for edge in self.edges_from(node) {
            if !visited[edge.to_node] {
                if self.has_cycle(edge.to_node, visited, in_stack) {
                    return true;
                }
            } else if in_stack[edge.to_node] {
                return true;
            }
        }

        in_stack[node] = false;
        false
    }

    /// Topological sort of nodes
    pub fn topological_sort(&self) -> Result<Vec<NodeId>, &'static str> {
        if !self.is_dag() {
            return Err("Graph contains cycles");
        }

        let mut result = Vec::new();
        let mut visited = vec![false; self.nodes.len()];

        for i in 0..self.nodes.len() {
            if !visited[i] {
                self.topo_dfs(i, &mut visited, &mut result);
            }
        }

        result.reverse();
        Ok(result)
    }

    fn topo_dfs(&self, node: NodeId, visited: &mut Vec<bool>, result: &mut Vec<NodeId>) {
        visited[node] = true;

        for edge in self.edges_from(node) {
            if !visited[edge.to_node] {
                self.topo_dfs(edge.to_node, visited, result);
            }
        }

        result.push(node);
    }
}

impl Default for IRGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_graph_creation() {
        let mut graph = IRGraph::new();

        let input_node = IRNode::new(0, "input").with_output(IRPort::own("value", Type::Int));

        let add_node = IRNode::new(0, "add")
            .with_input(IRPort::borrow("a", Type::Int))
            .with_input(IRPort::borrow("b", Type::Int))
            .with_output(IRPort::own("result", Type::Int));

        let id1 = graph.add_node(input_node);
        let id2 = graph.add_node(add_node);

        graph.add_edge(IREdge::borrow(id1, "value", id2, "a"));

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
        assert!(graph.is_dag());
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = IRGraph::new();

        let n1 = graph.add_node(IRNode::new(0, "A"));
        let n2 = graph.add_node(IRNode::new(0, "B"));
        let n3 = graph.add_node(IRNode::new(0, "C"));

        graph.add_edge(IREdge::borrow(n1, "out", n2, "in"));
        graph.add_edge(IREdge::borrow(n2, "out", n3, "in"));

        let order = graph.topological_sort().unwrap();
        assert_eq!(order, vec![0, 1, 2]);
    }
}
