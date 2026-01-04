// Code Generator

use anyhow::Result;

#[allow(dead_code)]
pub struct CodeGen {
    target: String,
}

#[allow(dead_code)]
impl CodeGen {
    pub fn new(target: &str) -> Self {
        CodeGen {
            target: target.to_string(),
        }
    }

    pub fn generate(&mut self, ir: &[u8]) -> Result<Vec<u8>> {
        // IR is already machine code from Cranelift
        Ok(ir.to_vec())
    }
}
