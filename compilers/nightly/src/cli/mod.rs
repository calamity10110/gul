// CLI argument parsing

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "gulc")]
#[command(author = "GUL Team")]
#[command(version = "0.1.0")]
#[command(about = "GUL Programming Language Compiler", long_about = None)]
pub struct Args {
    /// Input GUL source file
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// Output path
    #[arg(short, long, value_name = "OUTPUT", default_value = "output")]
    pub output: String,

    /// Compile and assemble only, do not link
    #[arg(short = 'c', long)]
    pub compile_only: bool,

    /// Target platform (x86_64, aarch64, wasm32)
    #[arg(short, long, default_value = "x86_64")]
    pub target: String,

    /// Optimization level (0-3)
    #[arg(short = 'O', long, default_value = "2")]
    pub opt_level: u8,

    /// Disable optimizations
    #[arg(long)]
    pub no_optimize: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Quiet mode (suppress all output except errors)
    #[arg(short, long)]
    pub quiet: bool,

    /// Dump tokens after lexing
    #[arg(long)]
    pub dump_tokens: bool,

    /// Dump AST after parsing
    #[arg(long)]
    pub dump_ast: bool,

    /// Auto-generate syntax documentation
    #[arg(long)]
    pub generate_syntax_docs: bool,

    /// Custom syntax.md output path
    #[arg(long, value_name = "PATH", default_value = "syntax.md")]
    pub syntax_output: String,
}
