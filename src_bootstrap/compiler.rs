#![allow(dead_code)]
// Compiler module - organizes code into blocks and generates output

pub mod blocks;
pub mod builder;
pub mod codegen;

use crate::ast::Program;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::path::Path;

pub struct Compiler {
    program: Program,
}

impl Compiler {
    pub fn new(program: Program) -> Self {
        Compiler { program }
    }

    pub fn compile(&self) -> Result<(), String> {
        // Implement compilation pipeline
        // 1. Parse and validate
        // 2. Organize into blocks
        // 3. Generate code
        self.organize_blocks()?;
        Ok(())
    }

    pub fn organize_blocks(&self) -> Result<(), String> {
        // Organize into package blocks using the blocks module
        blocks::organize_program(&self.program)?;
        Ok(())
    }
}

// Public API functions for CLI

pub fn build_target(file: &Path, target: &str, optimize: bool) -> Result<(), String> {
    // Read and compile the file
    let source =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;

    // Tokenize and parse
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    // Compile
    let compiler = Compiler::new(program);
    compiler.compile()?;

    // Build for target
    builder::build_for_target(target, optimize)?;

    Ok(())
}

pub fn organize_file(file: &Path) -> Result<(), String> {
    // Read and organize the file
    let source =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;

    // Tokenize and parse
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    // Organize into blocks
    let compiler = Compiler::new(program);
    compiler.organize_blocks()?;

    Ok(())
}

pub fn check_file(file: &Path) -> Result<(), String> {
    // Read and type-check the file
    let source =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;

    // Tokenize and parse
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    // Semantic analysis
    crate::semantic::analyze(&program)?;

    Ok(())
}
