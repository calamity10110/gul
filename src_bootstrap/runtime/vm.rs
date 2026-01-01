// GUL 101 Virtual Machine
// Lock-based, ownership-aware execution per Appendix G

use crate::ast::Ownership;
use crate::dataflow::ir::{IRGraph, IREdge, NodeId};
use crate::interpreter::Value;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

/// Unique identifier for runtime values
pub type ValueId = usize;

/// Lock state for a value
#[derive(Debug, Clone, PartialEq)]
pub enum LockState {
    /// No lock held
    Unlocked,
    /// Shared read lock (ref access)
    SharedRead(usize), // count of readers
    /// Exclusive write lock (borrow/take access)
    ExclusiveWrite,
}

/// Runtime value with ownership tracking
#[derive(Debug, Clone)]
pub struct RuntimeValue {
    pub id: ValueId,
    pub data: Value,
    pub owner: NodeId,
    pub lock: LockState,
}

/// VM State per GUL 101 Appendix G.2
pub struct VMState {
    /// All values in the system
    values: HashMap<ValueId, RuntimeValue>,
    /// Ownership map: value -> owner node
    owners: HashMap<ValueId, NodeId>,
    /// Lock states
    locks: HashMap<ValueId, LockState>,
    /// Next value ID
    next_value_id: ValueId,
}

impl VMState {
    pub fn new() -> Self {
        VMState {
            values: HashMap::new(),
            owners: HashMap::new(),
            locks: HashMap::new(),
            next_value_id: 0,
        }
    }

    /// Create a new value owned by a node
    pub fn create_value(&mut self, owner: NodeId, data: Value) -> ValueId {
        let id = self.next_value_id;
        self.next_value_id += 1;

        self.values.insert(id, RuntimeValue {
            id,
            data,
            owner,
            lock: LockState::Unlocked,
        });
        self.owners.insert(id, owner);
        self.locks.insert(id, LockState::Unlocked);

        id
    }

    /// Acquire lock based on ownership mode
    pub fn acquire_lock(&mut self, value_id: ValueId, mode: &Ownership) -> Result<(), String> {
        let lock = self.locks.get(&value_id).cloned().unwrap_or(LockState::Unlocked);

        match (mode, lock) {
            // Ref: acquire shared read lock
            (Ownership::Ref, LockState::Unlocked) => {
                self.locks.insert(value_id, LockState::SharedRead(1));
                Ok(())
            }
            (Ownership::Ref, LockState::SharedRead(n)) => {
                self.locks.insert(value_id, LockState::SharedRead(n + 1));
                Ok(())
            }
            (Ownership::Ref, LockState::ExclusiveWrite) => {
                Err("Cannot acquire ref: value has exclusive lock".to_string())
            }

            // Borrow/Take: acquire exclusive write lock
            (Ownership::Borrow | Ownership::Take, LockState::Unlocked) => {
                self.locks.insert(value_id, LockState::ExclusiveWrite);
                Ok(())
            }
            (Ownership::Borrow | Ownership::Take, _) => {
                Err("Cannot acquire borrow/take: value is locked".to_string())
            }

            // Gives: copy, no lock needed but verify ownership
            (Ownership::Gives | Ownership::Copy, _) => {
                // Copy mode doesn't need a lock
                Ok(())
            }

            // Own: internal use only
            (Ownership::Own, _) => Ok(()),
        }
    }

    /// Release lock
    pub fn release_lock(&mut self, value_id: ValueId, mode: &Ownership) {
        let lock = self.locks.get(&value_id).cloned().unwrap_or(LockState::Unlocked);

        match (mode, lock) {
            (Ownership::Ref, LockState::SharedRead(1)) => {
                self.locks.insert(value_id, LockState::Unlocked);
            }
            (Ownership::Ref, LockState::SharedRead(n)) if n > 1 => {
                self.locks.insert(value_id, LockState::SharedRead(n - 1));
            }
            (Ownership::Borrow, LockState::ExclusiveWrite) => {
                self.locks.insert(value_id, LockState::Unlocked);
            }
            (Ownership::Take, LockState::ExclusiveWrite) => {
                // Value ownership transferred, remove from original owner
                self.locks.insert(value_id, LockState::Unlocked);
            }
            _ => {}
        }
    }

    /// Transfer ownership
    pub fn transfer_ownership(&mut self, value_id: ValueId, new_owner: NodeId) {
        self.owners.insert(value_id, new_owner);
        if let Some(val) = self.values.get_mut(&value_id) {
            val.owner = new_owner;
        }
    }

    /// Get value by ID
    pub fn get_value(&self, value_id: ValueId) -> Option<&RuntimeValue> {
        self.values.get(&value_id)
    }

    /// Get owner of value
    pub fn get_owner(&self, value_id: ValueId) -> Option<NodeId> {
        self.owners.get(&value_id).copied()
    }
}

impl Default for VMState {
    fn default() -> Self {
        Self::new()
    }
}

/// GUL 101 VM Executor
pub struct VM {
    state: VMState,
}

impl VM {
    pub fn new() -> Self {
        VM {
            state: VMState::new(),
        }
    }

    /// Execute an IR graph per GUL 101 Appendix G.1
    pub fn execute(&mut self, graph: &IRGraph) -> Result<HashMap<String, Value>, String> {
        // 1. Get execution order (topological sort)
        let order = graph.topological_sort()?;

        // 2. Execute nodes in order
        for node_id in order {
            self.execute_node(graph, node_id)?;
        }

        // 3. Collect outputs
        let outputs = self.collect_outputs(graph);

        Ok(outputs)
    }

    /// Execute a single node with lock management
    fn execute_node(&mut self, graph: &IRGraph, node_id: NodeId) -> Result<(), String> {
        let node = graph
            .get_node(node_id)
            .ok_or_else(|| format!("Node {} not found", node_id))?;

        // Track input values and acquire locks
        let mut input_values: HashMap<String, ValueId> = HashMap::new();

        for edge in graph.edges_to(node_id) {
            // Get source node's output value
            if let Some(source_node) = graph.get_node(edge.from_node) {
                // Find the source output
                for output in &source_node.outputs {
                    if output.name == edge.from_port {
                        // Create or find value for this output
                        let value_id = self.state.create_value(
                            edge.from_node,
                            Value::Integer(0), // Would be actual computed value
                        );

                        // Acquire lock based on edge ownership mode
                        self.state.acquire_lock(value_id, &edge.mode)?;

                        input_values.insert(edge.to_port.clone(), value_id);

                        // Handle ownership transfer for Take mode
                        if edge.mode.moves_ownership() {
                            self.state.transfer_ownership(value_id, node_id);
                        }
                    }
                }
            }
        }

        // Execute node logic based on node type
        let output_values = self.execute_node_logic(node, &input_values)?;

        // Store output values
        for (port_name, value) in output_values {
            let value_id = self
                .state
                .create_value(node_id, value);
            // Output values would be tracked for downstream nodes
            let _ = (port_name, value_id); // Used by edges_from
        }

        // Release input locks
        for edge in graph.edges_to(node_id) {
            if let Some(&value_id) = input_values.get(&edge.to_port) {
                self.state.release_lock(value_id, &edge.mode);
            }
        }

        Ok(())
    }

    /// Execute the actual node logic
    fn execute_node_logic(
        &mut self,
        node: &crate::dataflow::ir::IRNode,
        inputs: &HashMap<String, ValueId>,
    ) -> Result<HashMap<String, Value>, String> {
        let mut outputs = HashMap::new();

        // Execute based on node name/type
        match node.name.as_str() {
            "add" | "sum" => {
                // Sum all inputs
                let mut total = 0i64;
                for (_, value_id) in inputs {
                    if let Some(val) = self.state.get_value(*value_id) {
                        if let Value::Integer(i) = &val.data {
                            total += i;
                        }
                    }
                }
                outputs.insert("result".to_string(), Value::Integer(total));
            }
            "multiply" | "mul" => {
                // Multiply all inputs
                let mut product = 1i64;
                for (_, value_id) in inputs {
                    if let Some(val) = self.state.get_value(*value_id) {
                        if let Value::Integer(i) = &val.data {
                            product *= i;
                        }
                    }
                }
                outputs.insert("result".to_string(), Value::Integer(product));
            }
            "print" | "output" => {
                // Output nodes - collect values for final output
                for (port_name, value_id) in inputs {
                    if let Some(val) = self.state.get_value(*value_id) {
                        outputs.insert(port_name.clone(), val.data.clone());
                    }
                }
            }
            "input" => {
                // Input nodes produce initial values
                for output in &node.outputs {
                    outputs.insert(output.name.clone(), Value::Integer(0));
                }
            }
            "double" => {
                // Double first input
                for (_, value_id) in inputs {
                    if let Some(val) = self.state.get_value(*value_id) {
                        if let Value::Integer(i) = &val.data {
                            outputs.insert("result".to_string(), Value::Integer(i * 2));
                            break;
                        }
                    }
                }
            }
            "filter" => {
                // Pass through inputs that match condition
                for (port_name, value_id) in inputs {
                    if let Some(val) = self.state.get_value(*value_id) {
                        outputs.insert(port_name.clone(), val.data.clone());
                    }
                }
            }
            "map" | "transform" => {
                // Apply transformation (identity for now)
                for (port_name, value_id) in inputs {
                    if let Some(val) = self.state.get_value(*value_id) {
                        outputs.insert(port_name.clone(), val.data.clone());
                    }
                }
            }
            _ => {
                // Default: pass through
                for (port_name, value_id) in inputs {
                    if let Some(val) = self.state.get_value(*value_id) {
                        outputs.insert(port_name.clone(), val.data.clone());
                    }
                }
            }
        }

        Ok(outputs)
    }

    /// Collect output values
    fn collect_outputs(&self, graph: &IRGraph) -> HashMap<String, Value> {
        let mut outputs = HashMap::new();

        for node_id in &graph.exit_nodes {
            if let Some(node) = graph.get_node(*node_id) {
                for output in &node.outputs {
                    // Placeholder: would collect actual output values
                    outputs.insert(
                        format!("{}:{}", node.name, output.name),
                        Value::Integer(0),
                    );
                }
            }
        }

        outputs
    }

    /// Check if nodes can run in parallel (G.4)
    pub fn can_parallelize(&self, graph: &IRGraph, node_a: NodeId, node_b: NodeId) -> bool {
        // Nodes can run in parallel if:
        // 1. They share no exclusive ownership
        // 2. They do not mutate shared data

        let edges_a: HashSet<_> = graph.edges_to(node_a)
            .iter()
            .map(|e| e.from_node)
            .collect();

        let edges_b: HashSet<_> = graph.edges_to(node_b)
            .iter()
            .map(|e| e.from_node)
            .collect();

        // Check for shared dependencies with exclusive access
        for shared in edges_a.intersection(&edges_b) {
            // Check edge modes - if any is Take or Borrow, can't parallelize
            for edge in graph.edges_from(*shared) {
                if edge.mode.moves_ownership() || edge.mode.is_mutable() {
                    return false;
                }
            }
        }

        true
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_state_creation() {
        let mut state = VMState::new();
        let id = state.create_value(0, Value::Integer(42));

        assert_eq!(state.get_owner(id), Some(0));
        let val = state.get_value(id).unwrap();
        assert_eq!(val.data, Value::Integer(42));
    }

    #[test]
    fn test_lock_acquire_release() {
        let mut state = VMState::new();
        let id = state.create_value(0, Value::Integer(100));

        // Acquire ref lock
        assert!(state.acquire_lock(id, &Ownership::Ref).is_ok());
        assert!(state.acquire_lock(id, &Ownership::Ref).is_ok()); // Multiple refs OK

        // Release
        state.release_lock(id, &Ownership::Ref);
        state.release_lock(id, &Ownership::Ref);
    }

    #[test]
    fn test_exclusive_lock() {
        let mut state = VMState::new();
        let id = state.create_value(0, Value::Integer(100));

        // Acquire borrow lock
        assert!(state.acquire_lock(id, &Ownership::Borrow).is_ok());

        // Cannot acquire another lock
        assert!(state.acquire_lock(id, &Ownership::Ref).is_err());

        // Release
        state.release_lock(id, &Ownership::Borrow);

        // Now can acquire
        assert!(state.acquire_lock(id, &Ownership::Ref).is_ok());
    }
}
