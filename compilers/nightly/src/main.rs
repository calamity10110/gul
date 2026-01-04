// Main entry point for GUL compiler

mod lexer;
mod parser;
mod ast;
mod semantic;
mod ir;
mod optimizer;
mod codegen;
mod stdlib;
mod ffi;
mod cli;
mod docs;

use anyhow::{anyhow, Result};
use clap::Parser as ClapParser;
use cli::Args;
use colored::*;
use std::fs;
use std::process::Command;

fn main() -> Result<()> {
    let args = Args::parse();
    
    if args.verbose {
        println!("{}", "GUL Compiler v0.1.0".bright_green().bold());
    }
    
    // Read source file
    let source = fs::read_to_string(&args.input)
        .map_err(|e| anyhow!("Failed to read {}: {}", args.input, e))?;
    
    // Compile
    match compile(&source, &args) {
        Ok(output_path) => {
            if !args.quiet {
                println!("{} Compiled successfully: {}", "✓".green(), output_path);
            }
            
            // Auto-generate syntax documentation if requested
            if args.generate_syntax_docs {
                docs::generate_syntax_md(&args.input, &args.output)?;
                if !args.quiet {
                    println!("{} Generated syntax.md", "✓".green());
                }
            }
            
            Ok(())
        }
        Err(e) => {
            eprintln!("{} Compilation failed:", "✗".red());
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

const STDLIB_C: &str = include_str!("stdlib/stdlib.c");

fn compile(source: &str, args: &Args) -> Result<String> {
    // Phase 1: Lexical Analysis
    if args.verbose {
        println!("{}", "→ Lexing...".cyan());
    }
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    if args.dump_tokens {
        println!("\n{}", "Tokens:".yellow());
        for token in &tokens {
            println!("  {:?}", token);
        }
    }
    
    // Phase 2: Parsing
    if args.verbose {
        println!("{}", "→ Parsing...".cyan());
    }
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;
    
    if args.dump_ast {
        println!("\n{}", "AST:".yellow());
        println!("{:#?}", ast);
    }
    
    // Phase 3: Semantic Analysis
    if args.verbose {
        println!("{}", "→ Semantic analysis...".cyan());
    }
    let mut analyzer = semantic::Analyzer::new();
    let checked_ast = analyzer.analyze(&ast)?;
    
    // Phase 4: IR Generation
    if args.verbose {
        println!("{}", "→ Generating IR...".cyan());
    }
    let mut ir_gen = ir::IrGenerator::new();
    let ir = ir_gen.generate(&checked_ast)?;
    
    // Phase 5: Optimization
    if !args.no_optimize {
        if args.verbose {
            println!("{}", "→ Optimizing...".cyan());
        }
        // let mut optimizer = optimizer::Optimizer::new();
        // optimizer.optimize(&ir)?;
    }
    
    // Phase 6: Code Generation
    if args.verbose {
        println!("{}", "→ Generating code...".cyan());
    }
    // For Cranelift, ir_gen.generate() currently returns object code bytes directly (IR phase combined with CodeGen for MVP)
    // Adjusting logic: 'ir' var above is actually the object code.
    let object_code = ir; 
    
    // Output handling
    if args.compile_only {
        // Just write the object file
        fs::write(&args.output, object_code)?;
        Ok(args.output.clone())
    } else {
        // Link logic
        let obj_path = format!("{}.o", args.output);
        fs::write(&obj_path, object_code)?;
        
        // Write stdlib to temp file
        let stdlib_path = format!("{}_stdlib.c", args.output);
        fs::write(&stdlib_path, STDLIB_C)?;
        
        if args.verbose {
            println!("{}", "→ Linking...".cyan());
        }
        
        let status = Command::new("cc")
            .arg(&obj_path) // User program object
            .arg(&stdlib_path) // Standard library C source
            .arg("-o")
            .arg(&args.output) // Output executable
            .status()
            .map_err(|e| anyhow!("Failed to invoke linker (cc): {}", e))?;
            
        // Cleanup temp files
        if !args.verbose {
             let _ = fs::remove_file(&obj_path);
             let _ = fs::remove_file(&stdlib_path);
        }
            
        if !status.success() {
             return Err(anyhow!("Linker failed with status: {}", status));
        }
        
        Ok(args.output.clone())
    }
}
