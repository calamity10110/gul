#![allow(dead_code)]
#![allow(unused_variables)]
// Distributed runtime for Universal Language
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    Active,
    Inactive,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub address: String,
    pub status: NodeStatus,
    pub load: f32, // 0.0 to 1.0
}

pub struct DistributedState {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl DistributedState {
    pub fn new() -> Self {
        DistributedState {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub fn delete(&self, key: &str) -> bool {
        let mut data = self.data.lock().unwrap();
        data.remove(key).is_some()
    }
}

impl Default for DistributedState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DistributedRuntime {
    nodes: Vec<Node>,
    state: DistributedState,
    replication_factor: usize,
}

impl Default for DistributedRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributedRuntime {
    pub fn new() -> Self {
        DistributedRuntime {
            nodes: Vec::new(),
            state: DistributedState::new(),
            replication_factor: 3,
        }
    }

    pub fn add_node(&mut self, id: String, address: String) {
        let node = Node {
            id,
            address,
            status: NodeStatus::Active,
            load: 0.0,
        };
        self.nodes.push(node);
    }

    pub fn remove_node(&mut self, id: &str) -> bool {
        if let Some(pos) = self.nodes.iter().position(|n| n.id == id) {
            self.nodes.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn list_nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn get_active_nodes(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|n| n.status == NodeStatus::Active)
            .collect()
    }

    pub fn set_node_status(&mut self, id: &str, status: NodeStatus) -> Result<(), String> {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == id) {
            node.status = status;
            Ok(())
        } else {
            Err(format!("Node '{}' not found", id))
        }
    }

    pub fn remote_call(
        &self,
        node_id: &str,
        function: &str,
        _args: Vec<String>,
    ) -> Result<String, String> {
        let node = self
            .nodes
            .iter()
            .find(|n| n.id == node_id)
            .ok_or_else(|| format!("Node '{}' not found", node_id))?;

        if node.status != NodeStatus::Active {
            return Err(format!("Node '{}' is not active", node_id));
        }

        // Simplified RPC - just return a success message
        Ok(format!("Called {} on {}", function, node.address))
    }

    pub fn balance_load(&mut self) -> Result<(), String> {
        let active_nodes = self.get_active_nodes().len();

        if active_nodes == 0 {
            return Err("No active nodes available".to_string());
        }

        let target_load = 1.0 / active_nodes as f32;

        for node in self.nodes.iter_mut() {
            if node.status == NodeStatus::Active {
                node.load = target_load;
            }
        }

        Ok(())
    }

    pub fn get_least_loaded_node(&self) -> Option<&Node> {
        self.get_active_nodes()
            .into_iter()
            .min_by(|a, b| a.load.partial_cmp(&b.load).unwrap())
    }

    pub fn replicate_state(&self, key: &str) -> Result<Vec<String>, String> {
        let active_nodes = self.get_active_nodes();

        if active_nodes.len() < self.replication_factor {
            return Err(format!(
                "Not enough active nodes ({}) for replication factor ({})",
                active_nodes.len(),
                self.replication_factor
            ));
        }

        // Return the IDs of nodes that would store this key
        Ok(active_nodes
            .iter()
            .take(self.replication_factor)
            .map(|n| n.id.clone())
            .collect())
    }

    pub fn handle_node_failure(&mut self, node_id: &str) -> Result<(), String> {
        self.set_node_status(node_id, NodeStatus::Failed)?;

        // Rebalance load among remaining nodes
        self.balance_load()?;

        Ok(())
    }

    pub fn get_state(&self) -> &DistributedState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distributed_runtime_creation() {
        let runtime = DistributedRuntime::new();
        assert_eq!(runtime.nodes.len(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut runtime = DistributedRuntime::new();
        runtime.add_node("node1".to_string(), "192.168.1.1:8080".to_string());

        assert_eq!(runtime.nodes.len(), 1);
        assert_eq!(runtime.nodes[0].id, "node1");
    }

    #[test]
    fn test_remove_node() {
        let mut runtime = DistributedRuntime::new();
        runtime.add_node("node1".to_string(), "192.168.1.1:8080".to_string());

        assert!(runtime.remove_node("node1"));
        assert_eq!(runtime.nodes.len(), 0);
    }

    #[test]
    fn test_get_active_nodes() {
        let mut runtime = DistributedRuntime::new();
        runtime.add_node("node1".to_string(), "192.168.1.1:8080".to_string());
        runtime.add_node("node2".to_string(), "192.168.1.2:8080".to_string());

        let active = runtime.get_active_nodes();
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn test_set_node_status() {
        let mut runtime = DistributedRuntime::new();
        runtime.add_node("node1".to_string(), "192.168.1.1:8080".to_string());

        assert!(runtime
            .set_node_status("node1", NodeStatus::Inactive)
            .is_ok());
        assert_eq!(runtime.nodes[0].status, NodeStatus::Inactive);
    }

    #[test]
    fn test_remote_call() {
        let mut runtime = DistributedRuntime::new();
        runtime.add_node("node1".to_string(), "192.168.1.1:8080".to_string());

        let result = runtime.remote_call("node1", "test_function", vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_balance_load() {
        let mut runtime = DistributedRuntime::new();
        runtime.add_node("node1".to_string(), "192.168.1.1:8080".to_string());
        runtime.add_node("node2".to_string(), "192.168.1.2:8080".to_string());

        assert!(runtime.balance_load().is_ok());
        assert_eq!(runtime.nodes[0].load, 0.5);
        assert_eq!(runtime.nodes[1].load, 0.5);
    }

    #[test]
    fn test_get_least_loaded_node() {
        let mut runtime = DistributedRuntime::new();
        runtime.add_node("node1".to_string(), "192.168.1.1:8080".to_string());
        runtime.add_node("node2".to_string(), "192.168.1.2:8080".to_string());

        runtime.nodes[0].load = 0.8;
        runtime.nodes[1].load = 0.3;

        let least_loaded = runtime.get_least_loaded_node().unwrap();
        assert_eq!(least_loaded.id, "node2");
    }

    #[test]
    fn test_replicate_state() {
        let mut runtime = DistributedRuntime::new();
        runtime.add_node("node1".to_string(), "192.168.1.1:8080".to_string());
        runtime.add_node("node2".to_string(), "192.168.1.2:8080".to_string());
        runtime.add_node("node3".to_string(), "192.168.1.3:8080".to_string());

        let replicas = runtime.replicate_state("test_key").unwrap();
        assert_eq!(replicas.len(), 3);
    }

    #[test]
    fn test_handle_node_failure() {
        let mut runtime = DistributedRuntime::new();
        runtime.add_node("node1".to_string(), "192.168.1.1:8080".to_string());
        runtime.add_node("node2".to_string(), "192.168.1.2:8080".to_string());

        assert!(runtime.handle_node_failure("node1").is_ok());
        assert_eq!(runtime.nodes[0].status, NodeStatus::Failed);
    }

    #[test]
    fn test_distributed_state() {
        let state = DistributedState::new();

        state.set("key1".to_string(), "value1".to_string());
        assert_eq!(state.get("key1").unwrap(), "value1");

        assert!(state.delete("key1"));
        assert!(state.get("key1").is_none());
    }
}
