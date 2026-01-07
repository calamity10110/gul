// Auto-generated from GUL source
#![allow(unused_variables, dead_code, unused_mut, unused_imports, non_snake_case)]

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

pub trait GulString {
    fn add_gul<T: Display>(&self, other: T) -> String;
}

impl GulString for String {
    fn add_gul<T: Display>(&self, other: T) -> String {
        format!("{}{}", self, other)
    }
}

impl GulString for &str {
    fn add_gul<T: Display>(&self, other: T) -> String {
        format!("{}{}", self, other)
    }
}

#[macro_export]
macro_rules! dict {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert($key.to_string(), $val); )*
        map
    }};
    ($($key:ident : $val:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert(stringify!($key).to_string(), $val); )*
        map
    }};
}

// Minimal sys module shim
pub mod sys {
    pub fn argv() -> Vec<String> {
        std::env::args().collect()
    }
}

// Minimal fs module shim
pub fn read_file(path: String) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn write_file(path: String, content: String) {
    let _ = std::fs::write(path, content);
}
mod ast;
mod builtins;
mod codegen;
mod ir;
mod lexer;
mod parser;
mod semantic;

// GUL v3.2 Compiler - Main Driver
// Integrates lexer, parser, semantic analyzer, and code generator

use std::io;
use std::fs;
use crate::lexer::lexer::*;
use crate::parser::parser::*;
use crate::semantic::analyzer::*;
use crate::codegen::rust_backend::*;
use crate::ir::cranelift_backend::CraneliftBackend;

// print("COMPILER MODULES LOADED")

// Compiler configuration
#[derive(Debug, Clone, PartialEq)]
pub struct CompilerConfig {
    pub input_file: String,
    pub output_file: String,
    pub emit_rust: bool,
    pub check_semantics: bool,
    pub verbose: bool,

// Compiler result
}
#[derive(Debug, Clone, PartialEq)]
pub struct CompileResult {
    pub success: bool,
    pub output_code: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,

// Main compiler
}
#[derive(Debug, Clone, PartialEq)]
pub struct Compiler {
    pub config: CompilerConfig,

}
pub fn create_compiler(config: CompilerConfig)  ->  Compiler {
    return Compiler{config: config};

}
pub fn compiler_read_source_file(compiler: Compiler, path: String)  ->  String {
    // Read source file contents// 
    let content = std::fs::read_to_string(&path);
    match content {
        Ok(s) => s,
        Err(e) => {
            println!("{}", "Error reading file: ".to_string() + &format!("{}", e));
            "".to_string()
        }
    }
}
pub fn compiler_write_output_file(compiler: Compiler, code: String) -> bool {
    // Write generated code to output file// 
    let res = std::fs::write(&compiler.config.output_file, code);
    match res {
        Ok(_) => {
            if compiler.config.verbose {
                println!("{}", "Generated code written to ".to_string() + &compiler.config.output_file);
            }
            true
        },
        Err(e) => {
            println!("{}", "Error writing file: ".to_string() + &format!("{}", e));
            false
        }
    }
}
pub fn compiler_create_result(success: bool, code: String, errors: Vec<String>, warnings: Vec<String>)  ->  CompileResult {
    // Create compile result// 
    return CompileResult{success: success, output_code: code, errors: errors, warnings: warnings};

}
pub fn compiler_compile(compiler: Compiler)  ->  CompileResult {
    // Compile GUL source file// 
    let mut errors = vec![];
    let mut warnings = vec![];

    let source = compiler_read_source_file(compiler.clone(), compiler.config.input_file.clone()); // FIX: Clone compiler to avoid move
    if source.is_empty() {
        errors.push("Failed to read source file".to_string().to_string());
        return compiler_create_result(false, "".to_string(), errors, warnings);

    }
    println!("{}", "  [1/4] Lexing...".to_string());
    let tokens = tokenize(source);
    println!("{}", "    Lexed ".to_string() + &format!("{}", (tokens).len()) + " tokens");
    


    println!("{}", "  [2/4] Parsing...".to_string());
    let ast = parse(tokens);

    // Check for parser errors (empty main entry and no statements)
    if ast.statements.is_empty() && ast.main_entry.is_empty() && ast.imports.is_empty() {
        errors.push("Parser error: No valid statements found".to_string());
    }

    // Phase 3: Semantic Analysis
    if compiler.config.check_semantics {
        if compiler.config.verbose {
            println!("{}", "  [3/4] Semantic analysis...".to_string());

        }
        let semantic_errors = analyze_semantics(ast.clone()); // Need to clone or refactor ownership
        errors.extend(semantic_errors.clone());

        if (semantic_errors).len() > 0 {
            return compiler_create_result(false, "".to_string(), errors, warnings);

        }
    }
    analyze_semantics(ast.clone());

    println!("{}", "  [4/4] Generating native code via Cranelift...".to_string());
    
    let mut cranelift = CraneliftBackend::new();
    match cranelift.generate(&ast) {
        Ok(object_code) => {
            // Write object file
            let obj_path = compiler.config.output_file.clone() + ".o";
            if let Err(e) = std::fs::write(&obj_path, &object_code) {
                errors.push(format!("Failed to write object file: {}", e));
                return compiler_create_result(false, "".to_string(), errors, warnings);
            }
            
            // Find stdlib.c path (relative to workspace)
            let stdlib_path = "compilers/shared/runtime/stdlib.c";
            
            // Link with cc
            println!("{}", "  Linking...".to_string());
            let status = std::process::Command::new("cc")
                .arg(&obj_path)
                .arg(stdlib_path)
                .arg("-o")
                .arg(&compiler.config.output_file)
                .arg("-lm")
                .status();
            
            match status {
                Ok(s) if s.success() => {
                    // Cleanup object file
                    let _ = std::fs::remove_file(&obj_path);
                    println!("{}", "Compilation successful!".to_string());
                }
                Ok(s) => {
                    errors.push(format!("Linking failed with status: {}", s));
                    return compiler_create_result(false, "".to_string(), errors, warnings);
                }
                Err(e) => {
                    errors.push(format!("Failed to invoke linker: {}", e));
                    return compiler_create_result(false, "".to_string(), errors, warnings);
                }
            }
        }
        Err(e) => {
            errors.push(format!("Code generation failed: {}", e));
            return compiler_create_result(false, "".to_string(), errors, warnings);
        }
    }
    
    return compiler_create_result(true, "".to_string(), errors, warnings);

// CLI interface
}
pub fn compile_file(input_file: String, output_file: String, options: HashMap<String, String>)  ->  CompileResult {
    println!("{}", "ENTER compile_file".to_string());
    // Compile a GUL file to Rust// 
    let config = CompilerConfig {
        input_file: input_file,
        // Generate output filename
        output_file: output_file,
        emit_rust: options.get("emit_rust").map(|s| s == "true").unwrap_or(true),
        check_semantics: options.get("check_semantics").map(|s| s == "true").unwrap_or(true),
        verbose: options.get("verbose").map(|s| s == "true").unwrap_or(false),
    };

    let mut compiler = create_compiler(config);
    let result = compiler_compile(compiler);
    return result;

// Main entry point for compiler
}
fn main() {
    // Parse command-line arguments
    let args = sys::argv();

    // Default values
    let mut output_file = "";
    let mut verbose = false;
    let mut check_semantics = true;

    // Simple argument parsing
    if (args).len() < 2 {
        println!("{}", "GUL Compiler v0.1.0".to_string());
        println!("{}", "Usage: gul-compile <input.mn> [options]".to_string());
        println!("{}", "Options:".to_string());
        println!("{}", "  -o <file>      Output file (default: input.rs)".to_string());
        println!("{}", "  --verbose      Verbose output".to_string());
        println!("{}", "  --no-semantic  Skip semantic analysis".to_string());
        return;

    }
    let input_file = &args[1];

    // Parse options
    let mut i = 2;
    while i < (args).len() {
        let arg = &args[i];
        if arg == "-o" {
            if i + 1 < (args).len() {
                output_file = &args[i + 1];
                i = i + 2;
            }
        }
        else if arg == "--verbose" {
            verbose = true;
            i = i + 1;
        }
        else if arg == "--no-semantic" {
            check_semantics = false;
            i = i + 1;
        }
        else {
            i = i + 1;
        }
    }

    // Default output file
    let output_string: String; 
    let final_output: String;
    
    if output_file == "" {
        output_string = input_file.replace(".mn", ""); // Default to no extension
        final_output = output_string.clone();
    } else {
        final_output = output_file.to_string();
    }
    let mut options = HashMap::new();
    options.insert("emit_rust".to_string(), "true".to_string());
    if check_semantics { options.insert("check_semantics".to_string(), "true".to_string()); } else { options.insert("check_semantics".to_string(), "false".to_string()); }
    if verbose { options.insert("verbose".to_string(), "true".to_string()); } else { options.insert("verbose".to_string(), "false".to_string()); }

    let result = compile_file(input_file.to_string(), final_output, options);

    if result.success {
        println!("{}", "Compilation successful!".to_string());
        if (result.warnings).len() > 0 {
            println!("{}", "Warnings: ".to_string() + &format!("{}", (result.warnings).len()));
        }
    }
    else {
        println!("{}", "Compilation failed!".to_string());
        // Use simple prints to debug structure
        println!("{}", "DEBUG: Result fields available? (Check interpreter debug)".to_string());
        // Flattened access
        let errs = result.errors;
        println!("{}", "DEBUG: errors var type: ".to_string() + &format!("{}", "Vec<String>".to_string()));
        if !errs.is_empty() {
            println!("{}", "Errors list length: ".to_string() + &format!("{}", (errs).len()));
            for error in errs {
                println!("{}", "  ".to_string() + &error);
            }
        }
        else {
            println!("{}", "Result.errors is empty".to_string());
        }
    }
}