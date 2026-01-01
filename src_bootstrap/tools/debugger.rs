// Debugger module - provides debugging capabilities
#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BreakpointType {
    Line(usize),
    Conditional { line: usize, condition: String },
    Function(String),
}

#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: usize,
    pub bp_type: BreakpointType,
    pub enabled: bool,
    pub hit_count: usize,
}

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub line: usize,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DebuggerState {
    Running,
    Paused,
    Stopped,
    SteppingOver,
    SteppingInto,
    SteppingOut,
}

pub struct Debugger {
    breakpoints: HashMap<usize, Breakpoint>,
    next_bp_id: usize,
    call_stack: Vec<StackFrame>,
    current_line: usize,
    state: DebuggerState,
    watch_expressions: HashMap<String, String>,
    variables: HashMap<String, String>,
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            breakpoints: HashMap::new(),
            next_bp_id: 1,
            call_stack: Vec::new(),
            current_line: 0,
            state: DebuggerState::Stopped,
            watch_expressions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    // Breakpoint management
    pub fn add_breakpoint(&mut self, bp_type: BreakpointType) -> usize {
        let id = self.next_bp_id;
        self.next_bp_id += 1;

        let breakpoint = Breakpoint {
            id,
            bp_type,
            enabled: true,
            hit_count: 0,
        };

        self.breakpoints.insert(id, breakpoint);
        id
    }

    pub fn remove_breakpoint(&mut self, id: usize) -> bool {
        self.breakpoints.remove(&id).is_some()
    }

    pub fn enable_breakpoint(&mut self, id: usize) -> bool {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_breakpoint(&mut self, id: usize) -> bool {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn list_breakpoints(&self) -> Vec<&Breakpoint> {
        self.breakpoints.values().collect()
    }

    pub fn should_break_at_line(&mut self, line: usize) -> bool {
        let mut should_break = false;
        let mut bp_ids_to_update = Vec::new();

        for (id, bp) in &self.breakpoints {
            if !bp.enabled {
                continue;
            }

            match &bp.bp_type {
                BreakpointType::Line(bp_line) => {
                    if *bp_line == line {
                        should_break = true;
                        bp_ids_to_update.push(*id);
                    }
                }
                BreakpointType::Conditional {
                    line: bp_line,
                    condition,
                } => {
                    if *bp_line == line && self.evaluate_condition(condition) {
                        should_break = true;
                        bp_ids_to_update.push(*id);
                    }
                }
                _ => {}
            }
        }

        // Update hit counts
        for id in bp_ids_to_update {
            if let Some(bp) = self.breakpoints.get_mut(&id) {
                bp.hit_count += 1;
            }
        }

        should_break
    }

    // Execution control
    pub fn start(&mut self) {
        self.state = DebuggerState::Running;
        self.current_line = 0;
        self.call_stack.clear();
    }

    pub fn pause(&mut self) {
        self.state = DebuggerState::Paused;
    }

    pub fn resume(&mut self) {
        self.state = DebuggerState::Running;
    }

    pub fn stop(&mut self) {
        self.state = DebuggerState::Stopped;
        self.call_stack.clear();
        self.current_line = 0;
    }

    pub fn step_over(&mut self) {
        self.state = DebuggerState::SteppingOver;
    }

    pub fn step_into(&mut self) {
        self.state = DebuggerState::SteppingInto;
    }

    pub fn step_out(&mut self) {
        self.state = DebuggerState::SteppingOut;
    }

    pub fn get_state(&self) -> &DebuggerState {
        &self.state
    }

    // Call stack management
    pub fn push_frame(&mut self, function_name: String, line: usize) {
        let frame = StackFrame {
            function_name,
            line,
            variables: HashMap::new(),
        };
        self.call_stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<StackFrame> {
        self.call_stack.pop()
    }

    pub fn get_call_stack(&self) -> &[StackFrame] {
        &self.call_stack
    }

    pub fn get_current_frame(&self) -> Option<&StackFrame> {
        self.call_stack.last()
    }

    pub fn get_current_frame_mut(&mut self) -> Option<&mut StackFrame> {
        self.call_stack.last_mut()
    }

    // Variable inspection
    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    pub fn list_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }

    pub fn inspect_variable(&self, name: &str) -> Option<String> {
        self.get_variable(name).map(|v| format!("{} = {}", name, v))
    }

    // Watch expressions
    pub fn add_watch(&mut self, expression: String) {
        let value = self.evaluate_watch(&expression);
        self.watch_expressions.insert(expression, value);
    }

    pub fn remove_watch(&mut self, expression: &str) -> bool {
        self.watch_expressions.remove(expression).is_some()
    }

    pub fn list_watches(&self) -> &HashMap<String, String> {
        &self.watch_expressions
    }

    pub fn update_watches(&mut self) {
        let expressions: Vec<String> = self.watch_expressions.keys().cloned().collect();
        for expr in expressions {
            let value = self.evaluate_watch(&expr);
            self.watch_expressions.insert(expr, value);
        }
    }

    // Helper methods
    fn evaluate_condition(&self, _condition: &str) -> bool {
        // Simplified condition evaluation
        // In a real implementation, this would parse and evaluate the condition
        true
    }

    fn evaluate_watch(&self, expression: &str) -> String {
        // Simplified watch evaluation
        // In a real implementation, this would evaluate the expression
        self.get_variable(expression)
            .cloned()
            .unwrap_or_else(|| "<not available>".to_string())
    }

    pub fn set_current_line(&mut self, line: usize) {
        self.current_line = line;
    }

    pub fn get_current_line(&self) -> usize {
        self.current_line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_creation() {
        let debugger = Debugger::new();
        assert_eq!(debugger.state, DebuggerState::Stopped);
        assert_eq!(debugger.current_line, 0);
    }

    #[test]
    fn test_add_breakpoint() {
        let mut debugger = Debugger::new();
        let id = debugger.add_breakpoint(BreakpointType::Line(10));
        assert_eq!(id, 1);
        assert_eq!(debugger.breakpoints.len(), 1);
    }

    #[test]
    fn test_remove_breakpoint() {
        let mut debugger = Debugger::new();
        let id = debugger.add_breakpoint(BreakpointType::Line(10));
        assert!(debugger.remove_breakpoint(id));
        assert_eq!(debugger.breakpoints.len(), 0);
    }

    #[test]
    fn test_enable_disable_breakpoint() {
        let mut debugger = Debugger::new();
        let id = debugger.add_breakpoint(BreakpointType::Line(10));

        assert!(debugger.disable_breakpoint(id));
        assert!(!debugger.breakpoints.get(&id).unwrap().enabled);

        assert!(debugger.enable_breakpoint(id));
        assert!(debugger.breakpoints.get(&id).unwrap().enabled);
    }

    #[test]
    fn test_should_break_at_line() {
        let mut debugger = Debugger::new();
        debugger.add_breakpoint(BreakpointType::Line(10));

        assert!(debugger.should_break_at_line(10));
        assert!(!debugger.should_break_at_line(11));
    }

    #[test]
    fn test_execution_control() {
        let mut debugger = Debugger::new();

        debugger.start();
        assert_eq!(debugger.state, DebuggerState::Running);

        debugger.pause();
        assert_eq!(debugger.state, DebuggerState::Paused);

        debugger.resume();
        assert_eq!(debugger.state, DebuggerState::Running);

        debugger.stop();
        assert_eq!(debugger.state, DebuggerState::Stopped);
    }

    #[test]
    fn test_call_stack() {
        let mut debugger = Debugger::new();

        debugger.push_frame("main".to_string(), 1);
        debugger.push_frame("foo".to_string(), 10);

        assert_eq!(debugger.get_call_stack().len(), 2);
        assert_eq!(debugger.get_current_frame().unwrap().function_name, "foo");

        debugger.pop_frame();
        assert_eq!(debugger.get_call_stack().len(), 1);
    }

    #[test]
    fn test_variable_inspection() {
        let mut debugger = Debugger::new();

        debugger.set_variable("x".to_string(), "42".to_string());
        assert_eq!(debugger.get_variable("x"), Some(&"42".to_string()));
        assert_eq!(debugger.inspect_variable("x"), Some("x = 42".to_string()));
    }

    #[test]
    fn test_watch_expressions() {
        let mut debugger = Debugger::new();
        debugger.set_variable("x".to_string(), "42".to_string());

        debugger.add_watch("x".to_string());
        assert_eq!(debugger.list_watches().len(), 1);

        debugger.remove_watch("x");
        assert_eq!(debugger.list_watches().len(), 0);
    }
}
