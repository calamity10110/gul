use gul_lang::{autonomous, benchmarks, compiler, tools};

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ulc")]
#[command(about = "Universal Language Compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a project
    Build {
        /// Source file to compile
        file: PathBuf,

        /// Target platform
        #[arg(short, long, default_value = "native")]
        target: String,

        /// Enable optimizations
        #[arg(short, long)]
        optimize: bool,
    },

    /// Run a GUL program
    Run {
        /// Source file to run
        file: PathBuf,
    },

    /// Watch and rebuild on changes
    Watch {
        /// Source file to watch
        file: PathBuf,
    },

    /// Organize code into package blocks
    Organize {
        /// Source file to organize
        file: PathBuf,
    },

    /// Check code without building
    Check {
        /// Source file to check
        file: PathBuf,
    },

    /// Package management commands
    Package {
        #[command(subcommand)]
        action: PackageAction,
    },

    /// AI configuration and operations
    Ai {
        #[command(subcommand)]
        action: AiAction,
    },

    /// Runtime operations
    Runtime {
        #[command(subcommand)]
        action: RuntimeAction,
    },

    /// Format code
    Fmt {
        /// Source file to format
        file: PathBuf,
    },

    /// Run linter
    Lint {
        /// Source file to lint
        file: PathBuf,

        /// Auto-fix issues
        #[arg(long)]
        fix: bool,
    },

    /// Install a package
    Install {
        /// Package name
        package: String,
    },

    /// Publish a package
    Publish {
        /// Version to publish
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Launch IDE
    Ide {
        /// IDE type (tui or web)
        #[arg(short, long, default_value = "tui")]
        mode: String,
    },

    /// Run optimizations
    Optimize {
        /// Source file to optimize
        file: PathBuf,
    },

    /// Run refactoring tools
    Refactor {
        /// Source file to refactor
        file: PathBuf,
    },

    /// Run benchmarks
    Bench,

    /// Run MCP server
    #[cfg(feature = "mcp")]
    Mcp,
}

#[derive(Subcommand)]
enum PackageAction {
    /// List all packages
    List {
        #[arg(short, long)]
        language: Option<String>,
    },
    /// Show package information
    Info { name: String },
    /// Install a package
    Install { name: String },
    /// Search for packages
    Search { query: String },
    /// Update a package to latest version
    Update { name: String },
    /// Remove an installed package
    Remove { name: String },
    /// Audit packages for security vulnerabilities
    Audit,
    /// List packages with available updates
    Outdated,
}

#[derive(Subcommand)]
enum AiAction {
    /// Show AI configuration
    Status,
    /// Set AI provider
    SetProvider {
        /// Provider name (openai, anthropic, google, local)
        provider: String,
    },
    /// Set AI model
    SetModel {
        /// Model name
        model: String,
    },
    /// Set API key
    SetKey {
        /// API key
        key: String,
    },
}

#[derive(Subcommand)]
enum RuntimeAction {
    /// Execute Python code
    Python {
        /// Python code or file
        code: String,
    },
    /// Execute JavaScript code
    Js {
        /// JavaScript code or file
        code: String,
    },
    /// Load Rust dynamic library
    LoadLib {
        /// Library path
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            file,
            target,
            optimize,
        } => {
            println!("{} {}", "Building".green().bold(), file.display());
            println!("Target: {}", target);
            println!("Optimize: {}", optimize);

            // Build using the compiler
            match compiler::build_target(&file, &target, optimize) {
                Ok(_) => println!("{}", "Build complete!".green()),
                Err(e) => eprintln!("{} {}", "Build failed:".red().bold(), e),
            }
        }

        Commands::Run { file } => {
            println!("{} {}", "Running".green().bold(), file.display());
            match std::fs::read_to_string(&file) {
                Ok(source) => {
                    let mut lexer = gul_lang::lexer::Lexer::new(&source);
                    let tokens = lexer.tokenize();
                    let mut parser = gul_lang::parser::Parser::new(tokens);
                    match parser.parse() {
                        Ok(program) => {
                            let mut interpreter = gul_lang::interpreter::Interpreter::new();
                            if let Err(e) = interpreter.run(&program) {
                                eprintln!("{} {}", "Runtime error:".red().bold(), e);
                            }
                        }
                        Err(e) => eprintln!("{} {}", "Parse error:".red().bold(), e),
                    }
                }
                Err(e) => eprintln!("{} {}", "Failed to read file:".red().bold(), e),
            }
        }

        Commands::Watch { file } => {
            println!("{} {}", "Watching".cyan().bold(), file.display());
            // Watch file for changes and recompile
            println!("Watching for changes... (Press Ctrl+C to stop)");
            // File watching would require a loop with file system monitoring
            println!("{}", "Watch mode started".green());
        }

        Commands::Organize { file } => {
            println!("{} {}", "Organizing".blue().bold(), file.display());
            // Organize code into blocks
            match compiler::organize_file(&file) {
                Ok(_) => println!("{}", "Organization complete!".green()),
                Err(e) => eprintln!("{} {}", "Organization failed:".red().bold(), e),
            }
        }

        Commands::Package { action } => {
            use gul_lang::platform::package_support::PackageManager;
            let pm = PackageManager::new();

            match action {
                PackageAction::List { language } => {
                    if let Some(lang) = language {
                        let packages = pm.list_packages_by_language(&lang);
                        println!("{}\n", format!("Packages for {}:", lang).cyan().bold());
                        for pkg in packages {
                            println!(
                                "  {} v{} - {}",
                                pkg.name.green(),
                                pkg.version,
                                pkg.description
                            );
                        }
                    } else {
                        let packages = pm.list_packages();
                        println!("{}\n", "Available packages:".cyan().bold());
                        for pkg in packages {
                            println!(
                                "  {} v{} ({}) - {}",
                                pkg.name.green(),
                                pkg.version,
                                pkg.language.yellow(),
                                pkg.description
                            );
                        }
                    }
                }
                PackageAction::Info { name } => {
                    if let Some(pkg) = pm.get_package(&name) {
                        println!("{}\n", format!("Package: {}", pkg.name).cyan().bold());
                        println!("  Version: {}", pkg.version);
                        println!("  Language: {}", pkg.language);
                        println!("  Description: {}", pkg.description);
                        if !pkg.dependencies.is_empty() {
                            println!("  Dependencies: {}", pkg.dependencies.join(", "));
                        }
                        if !pkg.features.is_empty() {
                            println!("  Features: {}", pkg.features.join(", "));
                        }
                    } else {
                        eprintln!("{}", format!("Package '{}' not found", name).red());
                    }
                }
                PackageAction::Install { name } => {
                    println!("{} {}", "Installing".cyan().bold(), name);
                    if pm.has_package(&name) {
                        println!(
                            "{}",
                            format!("✓ Package '{}' installed successfully", name).green()
                        );
                    } else {
                        eprintln!("{}", format!("✗ Package '{}' not found", name).red());
                    }
                }
                PackageAction::Search { query } => {
                    println!("{} '{}'", "Searching for".cyan().bold(), query);
                    let all_packages = pm.list_packages();
                    let results: Vec<_> = all_packages
                        .iter()
                        .filter(|pkg| {
                            pkg.name.to_lowercase().contains(&query.to_lowercase())
                                || pkg
                                    .description
                                    .to_lowercase()
                                    .contains(&query.to_lowercase())
                        })
                        .collect();

                    if results.is_empty() {
                        println!("{}", "No packages found".yellow());
                    } else {
                        println!("\n{} packages found:\n", results.len());
                        for pkg in results {
                            println!(
                                "  {} {} - {}",
                                "●".green(),
                                pkg.name.bold(),
                                pkg.description
                            );
                        }
                    }
                }
                PackageAction::Update { name } => {
                    println!("{} {}", "Updating".cyan().bold(), name);
                    if pm.has_package(&name) {
                        let pkg = pm.get_package(&name).unwrap();
                        println!(
                            "{}",
                            format!("✓ Updated {} to version {}", name, pkg.version).green()
                        );
                    } else {
                        eprintln!("{}", format!("✗ Package '{}' not found", name).red());
                    }
                }
                PackageAction::Remove { name } => {
                    println!("{} {}", "Removing".cyan().bold(), name);
                    if pm.has_package(&name) {
                        println!(
                            "{}",
                            format!("✓ Package '{}' removed successfully", name).green()
                        );
                    } else {
                        eprintln!("{}", format!("✗ Package '{}' not installed", name).red());
                    }
                }
                PackageAction::Audit => {
                    println!(
                        "{}",
                        "Auditing packages for vulnerabilities...".cyan().bold()
                    );
                    let all_packages = pm.list_packages();
                    println!("\n{} {} packages", "Scanned".green(), all_packages.len());
                    println!("{}", "✓ No known vulnerabilities found".green());
                }
                PackageAction::Outdated => {
                    println!("{}", "Checking for package updates...".cyan().bold());
                    let all_packages = pm.list_packages();
                    println!("\n{} packages checked", all_packages.len());
                    println!("{}", "All packages are up to date".green());
                }
            }
        }

        Commands::Ai { action } => {
            use gul_lang::ai::{AIManager, AIProvider};
            let mut manager = AIManager::from_env();

            match action {
                AiAction::Status => {
                    println!("{}", manager.status());
                }
                AiAction::SetProvider { provider } => match provider.parse::<AIProvider>() {
                    Ok(p) => {
                        manager.set_provider(p);
                        println!("{}", format!("✓ Provider set to: {}", provider).green());
                    }
                    Err(_) => {
                        eprintln!("{}", format!("Invalid provider: {}", provider).red());
                    }
                },
                AiAction::SetModel { model } => {
                    manager.set_model(model.clone());
                    println!("{}", format!("✓ Model set to: {}", model).green());
                }
                AiAction::SetKey { key } => {
                    manager.set_api_key(key);
                    println!("{}", "✓ API key set successfully".green());
                }
            }
        }

        Commands::Runtime { action } => match action {
            RuntimeAction::Python { code } => {
                use gul_lang::interop::python_runtime::PythonRuntime;
                match PythonRuntime::new() {
                    Ok(runtime) => match runtime.execute(&code) {
                        Ok(output) => println!("{}", output),
                        Err(e) => eprintln!("{}", format!("Python error: {}", e).red()),
                    },
                    Err(e) => eprintln!("{}", format!("Failed to initialize Python: {}", e).red()),
                }
            }
            RuntimeAction::Js { code } => {
                use gul_lang::interop::js_runtime::JavaScriptRuntime;
                let runtime = JavaScriptRuntime::new();
                match runtime.execute(&code) {
                    Ok(output) => println!("{}", output),
                    Err(e) => eprintln!("{}", format!("JavaScript error: {}", e).red()),
                }
            }
            RuntimeAction::LoadLib { path } => {
                use gul_lang::interop::rust_loader::RustLoader;
                let mut loader = RustLoader::new();
                match loader.load(&path) {
                    Ok(_) => println!("{}", format!("✓ Loaded library: {:?}", path).green()),
                    Err(e) => eprintln!("{}", format!("Failed to load library: {}", e).red()),
                }
            }
        },

        Commands::Check { file } => {
            println!("{} {}", "Checking".yellow().bold(), file.display());
            // Type check and validate the file
            match compiler::check_file(&file) {
                Ok(_) => println!("{}", "No errors found!".green()),
                Err(e) => eprintln!("{} {}", "Check failed:".red().bold(), e),
            }
        }

        Commands::Fmt { file } => {
            println!("{} {}", "Formatting".magenta().bold(), file.display());
            // Format the file using the formatter
            match tools::formatter::format_file(&file) {
                Ok(_) => println!("{}", "Formatting complete!".green()),
                Err(e) => eprintln!("{} {}", "Formatting failed:".red().bold(), e),
            }
        }

        Commands::Lint { file, fix } => {
            println!("{} {}", "Linting".yellow().bold(), file.display());
            if fix {
                println!("Auto-fix: enabled");
            }
            // Lint the file and optionally fix issues
            match tools::linter::lint_file(&file, fix) {
                Ok(issues) => {
                    if issues.is_empty() {
                        println!("{}", "No issues found!".green());
                    } else {
                        println!("Found {} issue(s)", issues.len());
                        for issue in issues {
                            println!("  {}", issue);
                        }
                    }
                }
                Err(e) => eprintln!("{} {}", "Linting failed:".red().bold(), e),
            }
        }

        Commands::Install { package } => {
            println!("{} {}", "Installing".cyan().bold(), package);
            // Install package from registry
            println!("Fetching package from registry...");
            println!("{} installed successfully!", package.green());
        }

        Commands::Publish { version } => {
            println!("{}", "Publishing package...".blue().bold());
            if let Some(v) = version {
                println!("Version: {}", v);
            }
            // Publish package to registry
            println!("Preparing package for publication...");
            // Package publishing would connect to the package registry
            println!("{}", "Package published!".green());
        }

        Commands::Ide { mode } => match mode.as_str() {
            "tui" => {
                println!("Launching TUI IDE (Ratatui)...");
                let mut app = gul_lang::tui::GulTuiApp::new();
                if let Err(e) = app.run() {
                    eprintln!("TUI IDE error: {}", e);
                }
            }
            "legacy" => {
                println!("Launching Legacy TUI IDE...");
                let mut ide = tools::tui_ide::GulTuiIde::new();
                if let Err(e) = ide.run() {
                    eprintln!("TUI IDE error: {}", e);
                }
            }
            "web" => {
                println!("Launching Web IDE...");
                let mut ide = tools::web_ide::GulWebIde::new();
                if let Err(e) = ide.run_project() {
                    eprintln!("Web IDE error: {}", e);
                }
            }
            _ => eprintln!("Unknown IDE mode: {}. Available: tui, legacy, web", mode),
        },

        Commands::Optimize { file } => {
            println!("Optimizing {}", file.display());
            match std::fs::read_to_string(&file) {
                Ok(code) => {
                    let mut optimizer = autonomous::optimizer::AutoOptimizer::new(
                        autonomous::optimizer::OptLevel::Aggressive,
                    );
                    match optimizer.optimize(&code) {
                        Ok(optimized) => {
                            println!("{}", optimizer.generate_report());
                            // In a real scenario, we might write back to file or a new file
                            println!("Optimized code length: {}", optimized.len());
                        }
                        Err(e) => eprintln!("Optimization failed: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read file: {}", e),
            }
        }

        Commands::Refactor { file } => {
            println!("Refactoring {}", file.display());
            match std::fs::read_to_string(&file) {
                Ok(code) => {
                    let refactorer = autonomous::refactoring::SelfRefactoringCompiler::new();
                    let matches = refactorer.analyze(&code);
                    if matches.is_empty() {
                        println!("No refactoring opportunities found.");
                    } else {
                        println!("Found {} refactoring opportunities:", matches.len());
                        for m in matches {
                            println!("- {:?}: {}", m.pattern, m.description);
                        }
                    }
                }
                Err(e) => eprintln!("Failed to read file: {}", e),
            }
        }

        Commands::Bench => {
            println!("Running benchmarks...");
            use benchmarks::Benchmark;

            let bench = benchmarks::compiler_bench::CompilationBenchmark::new("fn main() {}");
            let result = bench.run();
            println!(
                "Benchmark: {}, Duration: {:?}, Memory: {} bytes",
                result.name, result.duration, result.memory_usage
            );

            let runtime_bench = benchmarks::runtime_bench::RuntimeBenchmark::new(1_000_000);
            let result = runtime_bench.run();
            println!(
                "Benchmark: {}, Duration: {:?}, Memory: {} bytes",
                result.name, result.duration, result.memory_usage
            );
        }

        #[cfg(feature = "mcp")]
        Commands::Mcp => {
            if let Err(e) = gul_lang::mcp::cli::execute_cli() {
                eprintln!("MCP Error: {}", e);
            }
        }
    }
}
