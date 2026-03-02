use crate::frontend::ast::*;
use std::collections::HashMap;

// Module declarations
pub mod values;
mod builtins;
mod exec;
mod eval;

// Re-export public types
pub use values::{ControlFlow, Value};

#[derive(Clone)]
pub struct Interpreter {
    pub variables: HashMap<String, Value>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            variables: HashMap::new(),
        };
        interpreter.register_builtins();
        interpreter
    }

    pub fn run(&mut self, program: &crate::frontend::ast::Program) -> Result<(), String> {
        for stmt in &program.statements {
            if let ControlFlow::Return(_) = self.execute(stmt)? {
                break;
            }
        }
        Ok(())
    }
}
