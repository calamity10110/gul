// Data-flow executor
// Executes data-flow graphs

use super::DataFlowGraph;
use crate::interpreter::Value;
use std::collections::HashMap;

/// Execution context
pub struct ExecutionContext {
    /// Values at each port
    port_values: HashMap<String, Value>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        ExecutionContext {
            port_values: HashMap::new(),
        }
    }

    /// Set value at a port
    pub fn set_port_value(&mut self, node: &str, port: &str, value: Value) {
        let key = format!("{}:{}", node, port);
        self.port_values.insert(key, value);
    }

    /// Get value at a port
    pub fn get_port_value(&self, node: &str, port: &str) -> Option<&Value> {
        let key = format!("{}:{}", node, port);
        self.port_values.get(&key)
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Data-flow executor
pub struct Executor {
    context: ExecutionContext,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            context: ExecutionContext::new(),
        }
    }

    /// Execute a data-flow graph
    pub fn execute(&mut self, graph: &DataFlowGraph) -> Result<Vec<Value>, String> {
        // 1. Set external inputs
        for (input, target) in &graph.external_inputs {
            // Convert ExternalInput value to Value
            // This would use the interpreter to evaluate the expression
            let value = Value::Integer(0); // Placeholder
            self.context.set_port_value(&target.node_name, &target.port_name, value);
        }

        // 2. Execute nodes in topological order
        let execution_order = self.topological_sort(graph)?;
        
        for node_name in &execution_order {
            self.execute_node(graph, node_name)?;
        }

        // 3. Collect outputs
        let mut outputs = Vec::new();
        for output in &graph.outputs {
            if let Some(value) = self.context.get_port_value(&output.node_name, &output.port_name) {
                outputs.push(value.clone());
            }
        }

        Ok(outputs)
    }

    /// Topological sort of nodes
    fn topological_sort(&self, graph: &DataFlowGraph) -> Result<Vec<String>, String> {
        // Simple implementation: return nodes in order
        Ok(graph.nodes.iter().map(|n| n.name.clone()).collect())
    }

    /// Execute a single node
    fn execute_node(&mut self, _graph: &DataFlowGraph, _node_name: &str) -> Result<(), String> {
        // This would execute the node's function with inputs and set outputs
        // For now, just a placeholder
        Ok(())
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_context() {
        let mut ctx = ExecutionContext::new();
        ctx.set_port_value("node1", "a", Value::Integer(42));
        
        let value = ctx.get_port_value("node1", "a");
        assert!(value.is_some());
        assert_eq!(*value.unwrap(), Value::Integer(42));
    }
}
