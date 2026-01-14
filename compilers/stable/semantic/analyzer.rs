// GUL v3.2 Compiler - Semantic Analyzer
// Type checking, scope management, and semantic validation

use std::collections::{HashMap, HashSet};
use crate::ast::nodes::*;
use crate::lexer::token::*;

// Method Signature for type checking
#[derive(Debug, Clone, PartialEq)]
pub struct MethodSignature {
    pub params: Vec<String>, // Parameter types
    pub return_type: String,
}

// Symbol table for scope management
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: String,
    pub is_mutable: bool,
    pub is_function: bool,
    pub signature: Option<MethodSignature>, // Added signature
    pub ownership_mode: String, // "owned", "borrow", "ref", "move", "kept"
    pub is_borrowed: bool, // Currently borrowed
    pub borrow_count: usize, // Number of active borrows
    pub line: usize,
    pub column: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>, // name -> Symbol
    pub parent: Option<Box<Scope>>, // Can be None

}
pub fn create_scope()  ->  Scope {
    return Scope{symbols: HashMap::new(), parent: None};

}
pub fn create_child_scope(parent: Scope)  ->  Scope {
    return Scope{symbols: HashMap::new(), parent: Some(Box::new(parent))};
}
impl Scope {
    pub fn define(&mut self, name: String, symbol: Symbol) {
        // Define a symbol in this scope// 
        self.symbols.insert(name, symbol);

    }
    pub fn resolve(&self, name: String)  ->  Option<Symbol> {
        // Resolve a symbol, checking parent scopes// 
        if self.symbols.contains_key(&name) {
            return Some(self.symbols.get(&name).unwrap().clone());
        }
        
        if let Some(parent_scope) = &self.parent {
             return parent_scope.resolve(name);
        }
        
        return None; // Not found

    }
    pub fn exists_in_current(&self, name: String)  ->  bool {
        // Check if symbol exists in current scope only// 
        return self.symbols.contains_key(&name);
    }
    
    // Borrow checker methods
    pub fn borrow(&mut self, name: String) -> Result<(), String> {
        // Create an immutable borrow of a symbol
        if let Some(symbol) = self.symbols.get_mut(&name) {
            if symbol.ownership_mode == "move" {
                return Err(format!("Cannot borrow '{}': value has been moved", name));
            }
            symbol.borrow_count += 1;
            symbol.is_borrowed = true;
            Ok(())
        } else if let Some(ref mut parent) = self.parent {
            parent.borrow(name)
        } else {
            Err(format!("Cannot borrow '{}': not found", name))
        }
    }
    
    pub fn borrow_mut(&mut self, name: String) -> Result<(), String> {
        // Create a mutable borrow of a symbol
        if let Some(symbol) = self.symbols.get_mut(&name) {
            if symbol.ownership_mode == "move" {
                return Err(format!("Cannot mutably borrow '{}': value has been moved", name));
            }
            if symbol.is_borrowed || symbol.borrow_count > 0 {
                return Err(format!("Cannot mutably borrow '{}': already borrowed", name));
            }
            if !symbol.is_mutable {
                return Err(format!("Cannot mutably borrow '{}': not declared as 'var'", name));
            }
            symbol.is_borrowed = true;
            symbol.ownership_mode = "ref".to_string();
            Ok(())
        } else if let Some(ref mut parent) = self.parent {
            parent.borrow_mut(name)
        } else {
            Err(format!("Cannot mutably borrow '{}': not found", name))
        }
    }
    
    pub fn release_borrow(&mut self, name: String) {
        // Release a borrow
        if let Some(symbol) = self.symbols.get_mut(&name) {
            if symbol.borrow_count > 0 {
                symbol.borrow_count -= 1;
            }
            if symbol.borrow_count == 0 {
                symbol.is_borrowed = false;
                if symbol.ownership_mode == "ref" {
                    symbol.ownership_mode = "owned".to_string();
                }
            }
        } else if let Some(ref mut parent) = self.parent {
            parent.release_borrow(name);
        }
    }
    
    pub fn mark_moved(&mut self, name: String) -> Result<(), String> {
        // Mark a symbol as moved (ownership transferred)
        if let Some(symbol) = self.symbols.get_mut(&name) {
            if symbol.is_borrowed || symbol.borrow_count > 0 {
                return Err(format!("Cannot move '{}': currently borrowed", name));
            }
            symbol.ownership_mode = "move".to_string();
            Ok(())
        } else if let Some(ref mut parent) = self.parent {
            parent.mark_moved(name)
        } else {
            Err(format!("Cannot move '{}': not found", name))
        }
    }
    
    pub fn check_use_after_move(&self, name: &String) -> Result<(), String> {
        // Check if a symbol has been moved
        if let Some(symbol) = self.symbols.get(name) {
            if symbol.ownership_mode == "move" {
                return Err(format!("Use of moved value: '{}'", name));
            }
            Ok(())
        } else if let Some(ref parent) = self.parent {
            parent.check_use_after_move(name)
        } else {
            Ok(()) // Not found in scope, let other checks handle it
        }
    }
// Semantic analyzer
}
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticAnalyzer {
    pub current_scope: Scope,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,

}
pub fn create_analyzer()  ->  SemanticAnalyzer {
    let mut analyzer = SemanticAnalyzer{current_scope: create_scope(), errors: vec![], warnings: vec![]};
    
    // Define builtins
    analyzer.current_scope.symbols.insert("print".to_string(), Symbol{
        name: "print".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "void".to_string() }),
        ownership_mode: "owned".to_string(),
        is_borrowed: false,
        borrow_count: 0,
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("println".to_string(), Symbol{
        name: "println".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "void".to_string() }),
        ownership_mode: "owned".to_string(),
        is_borrowed: false,
        borrow_count: 0,
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("str".to_string(), Symbol{
        name: "str".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "str".to_string() }),
        ownership_mode: "owned".to_string(),
        is_borrowed: false,
        borrow_count: 0,
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("int".to_string(), Symbol{
        name: "int".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "int".to_string() }),
        ownership_mode: "owned".to_string(),
        is_borrowed: false,
        borrow_count: 0,
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("len".to_string(), Symbol{
        name: "len".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "int".to_string() }),
        ownership_mode: "owned".to_string(),
        is_borrowed: false,
        borrow_count: 0,
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("input".to_string(), Symbol{
        name: "input".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        signature: Some(MethodSignature{ params: vec![], return_type: "str".to_string() }),
        ownership_mode: "owned".to_string(),
        is_borrowed: false,
        borrow_count: 0,
        line: 0,
        column: 0,
    });

    // Math Builtins
    let math_funcs = vec![
        "_native_sin", "_native_cos", "_native_tan", "_native_asin", "_native_acos", "_native_atan", "_native_atan2",
        "_native_exp", "_native_ln", "_native_log10", "_native_log2", "_native_pow", "_native_sqrt", "_native_cbrt",
        "gul_math_sin", "gul_math_cos", "gul_math_tan", "gul_math_exp", "gul_math_log", "gul_math_sqrt", "gul_math_pow",
        "gul_math_floor", "gul_math_ceil", "gul_math_round", "gul_math_trunc", "gul_math_abs", "gul_math_min", "gul_math_max",
        "gul_print_float",
    ];
    for f in math_funcs {
        analyzer.current_scope.symbols.insert(f.to_string(), Symbol{
            name: f.to_string(), 
            symbol_type: "fn".to_string(), 
            is_mutable: false, 
            is_function: true, 
            signature: Some(MethodSignature{ params: vec!["float".to_string()], return_type: "float".to_string() }),
            ownership_mode: "owned".to_string(),
            is_borrowed: false,
            borrow_count: 0,
            line: 0, 
            column: 0
        });
    }

    // Tensor & ML Builtins
    let tensor_funcs = vec![
        "gul_tensor_alloc", "gul_tensor_free", "gul_tensor_fill", 
        "gul_tensor_add", "gul_tensor_mul", "gul_tensor_matmul", 
        "gul_tensor_sum", "gul_tensor_mean",
        "gul_ml_sigmoid", "gul_ml_tanh", "gul_ml_relu",
        "gul_file_open", "gul_file_close",        "gul_file_read_line",
        // String
        "gul_string_len",
        "gul_string_substr",
        "gul_string_get",
        "gul_malloc"
    ];
    for f in tensor_funcs {
        analyzer.current_scope.symbols.insert(f.to_string(), Symbol{
            name: f.to_string(), 
            symbol_type: "fn".to_string(), 
            is_mutable: false, 
            is_function: true, 
            signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "any".to_string() }), // Generic signature for now
            ownership_mode: "owned".to_string(),
            is_borrowed: false,
            borrow_count: 0,
            line: 0, 
            column: 0
        });
    }

    // Autograd Builtins (Specific Signatures)
    analyzer.current_scope.symbols.insert("gul_autograd_begin".to_string(), Symbol{
        name: "gul_autograd_begin".to_string(), symbol_type: "fn".to_string(), is_mutable: false, is_function: true,
        signature: Some(MethodSignature{ params: vec![], return_type: "void".to_string() }), 
        ownership_mode: "owned".to_string(), is_borrowed: false, borrow_count: 0, line: 0, column: 0
    });
    analyzer.current_scope.symbols.insert("gul_autograd_end".to_string(), Symbol{
        name: "gul_autograd_end".to_string(), symbol_type: "fn".to_string(), is_mutable: false, is_function: true,
        signature: Some(MethodSignature{ params: vec![], return_type: "void".to_string() }), 
        ownership_mode: "owned".to_string(), is_borrowed: false, borrow_count: 0, line: 0, column: 0
    });
    analyzer.current_scope.symbols.insert("gul_make_var".to_string(), Symbol{
        name: "gul_make_var".to_string(), symbol_type: "fn".to_string(), is_mutable: false, is_function: true,
        signature: Some(MethodSignature{ params: vec!["float".to_string()], return_type: "any".to_string() }), 
        ownership_mode: "owned".to_string(), is_borrowed: false, borrow_count: 0, line: 0, column: 0
    });
    analyzer.current_scope.symbols.insert("gul_var_val".to_string(), Symbol{
        name: "gul_var_val".to_string(), symbol_type: "fn".to_string(), is_mutable: false, is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "float".to_string() }), 
        ownership_mode: "owned".to_string(), is_borrowed: false, borrow_count: 0, line: 0, column: 0
    });
    analyzer.current_scope.symbols.insert("gul_var_grad".to_string(), Symbol{
        name: "gul_var_grad".to_string(), symbol_type: "fn".to_string(), is_mutable: false, is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "float".to_string() }), 
        ownership_mode: "owned".to_string(), is_borrowed: false, borrow_count: 0, line: 0, column: 0
    });
    analyzer.current_scope.symbols.insert("gul_var_add".to_string(), Symbol{
        name: "gul_var_add".to_string(), symbol_type: "fn".to_string(), is_mutable: false, is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string(), "any".to_string()], return_type: "any".to_string() }), 
        ownership_mode: "owned".to_string(), is_borrowed: false, borrow_count: 0, line: 0, column: 0
    });
    analyzer.current_scope.symbols.insert("gul_var_mul".to_string(), Symbol{
        name: "gul_var_mul".to_string(), symbol_type: "fn".to_string(), is_mutable: false, is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string(), "any".to_string()], return_type: "any".to_string() }), 
        ownership_mode: "owned".to_string(), is_borrowed: false, borrow_count: 0, line: 0, column: 0
    });
    analyzer.current_scope.symbols.insert("gul_var_sin".to_string(), Symbol{
        name: "gul_var_sin".to_string(), symbol_type: "fn".to_string(), is_mutable: false, is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "any".to_string() }), 
        ownership_mode: "owned".to_string(), is_borrowed: false, borrow_count: 0, line: 0, column: 0
    });
    analyzer.current_scope.symbols.insert("gul_backward".to_string(), Symbol{
        name: "gul_backward".to_string(), symbol_type: "fn".to_string(), is_mutable: false, is_function: true,
        signature: Some(MethodSignature{ params: vec!["any".to_string()], return_type: "void".to_string() }), 
        ownership_mode: "owned".to_string(), is_borrowed: false, borrow_count: 0, line: 0, column: 0
    });
    
    return analyzer;

}
impl SemanticAnalyzer {
    pub fn analyze_program(&mut self, program: Program) {
        // Analyze entire program// 
        // Analyze imports first
        for import_stmt in program.imports {
            self.analyze_import(&import_stmt);

        // Analyze statements
        }
        // Analyze functions
        for func in &program.functions {
            self.analyze_function_decl(func);
        }
        // Analyze statements
        for stmt in program.statements {
            self.analyze_statement(&stmt);

        // Analyze main entry point
        }
        for stmt in program.main_entry {
            self.analyze_statement(&stmt);

        }
    }
    pub fn enter_scope(&mut self) {
        // Enter a new scope// 
        // Note: we move self.current_scope into the new child, so we need to clone usage if needed, 
        // but create_child_scope takes ownership of parent.
        // We need to carefully swap scopes to avoid move issues, or just replace.
        // Rust move semantics: self.current_scope is moved into function call.
        let old_scope = std::mem::replace(&mut self.current_scope, create_scope()); // Temporary dummy
        self.current_scope = create_child_scope(old_scope);

    }
    pub fn exit_scope(&mut self) {
        // Exit current scope// 
        if let Some(parent) = self.current_scope.parent.take() {
             self.current_scope = *parent;
        }
    }
    pub fn error(&mut self, message: String, line: usize, column: usize) {
        // Record an error// 
        let error_msg = format!("Semantic error at {}:{}: {}", line, column, message);
        self.errors.push(error_msg.clone());
        println!("{}", error_msg);

    }
    pub fn warn(&mut self, message: String, line: usize, column: usize) {
        // Record a warning// 
        let warn_msg = format!("Warning at {}:{}: {}", line, column, message);
        self.warnings.push(warn_msg);

    // ========================================
    // STATEMENT ANALYSIS
    // ========================================

    }
    pub fn analyze_statement(&mut self, stmt: &Statement) {
        // Analyze a statement// 
        match stmt {
            Statement::LetDecl(s) => self.analyze_let_stmt(s),
            Statement::VarDecl(s) => self.analyze_var_stmt(s),
            Statement::FunctionDecl(s) => self.analyze_function_decl(s),
            Statement::IfStmt(s) => self.analyze_if_stmt(s),
            Statement::WhileStmt(s) => self.analyze_while_stmt(s),
            Statement::ForStmt(s) => self.analyze_for_stmt(s),
            Statement::ReturnStmt(s) => self.analyze_return_stmt(s),
            Statement::AssignmentStmt(s) => self.analyze_assignment(s),
            Statement::ExpressionStmt(s) => self.analyze_expression_stmt(s),
            Statement::ImportStmt(s) => self.analyze_import(s),
            Statement::StructDecl(s) => self.analyze_struct_decl(s),
            Statement::ForeignCodeBlock(s) => self.analyze_foreign_block(s),
            _ => {}

        }
    }
    pub fn analyze_let_stmt(&mut self, stmt: &LetStmt) {
        // Analyze let declaration// 
        // Check if already defined in current scope
        if self.current_scope.exists_in_current(stmt.name.clone()) {
            self.error(format!("Variable '{}' already defined", stmt.name), stmt.node.line, stmt.node.column);
        }

        // Analyze the value expression
        let value_type = self.analyze_expression(&stmt.value);

        // Check type annotation if provided
        if !stmt.type_annotation.is_empty() && !value_type.is_empty() {
            if ! self.types_compatible(stmt.type_annotation.clone(), value_type.clone()) {
                self.error(format!("Type mismatch: expected {}, got {}", stmt.type_annotation, value_type),
                        stmt.node.line, stmt.node.column);
            }
        }
        // Define the symbol
        self.current_scope.define(stmt.name.clone(), Symbol{
            name: stmt.name.clone(),
            symbol_type: if !stmt.type_annotation.is_empty() { stmt.type_annotation.clone() } else { value_type },
            is_mutable: false,
            is_function: false,
            signature: None,
            ownership_mode: "owned".to_string(),
            is_borrowed: false,
            borrow_count: 0,
            line: stmt.node.line,
            column: stmt.node.column,
        },
        );

    }
    pub fn analyze_var_stmt(&mut self, stmt: &VarStmt) {
        // Analyze var declaration (similar to let but mutable)// 
        if self.current_scope.exists_in_current(stmt.name.clone()) {
            self.error(format!("Variable '{}' already defined", stmt.name), stmt.node.line, stmt.node.column);

        }
        let value_type = self.analyze_expression(&stmt.value);

        if !stmt.type_annotation.is_empty() && !value_type.is_empty() {
            if ! self.types_compatible(stmt.type_annotation.clone(), value_type.clone()) {
                self.error(format!("Type mismatch: expected {}, got {}", stmt.type_annotation, value_type),
                        stmt.node.line, stmt.node.column);

            }
        }
        self.current_scope.define(stmt.name.clone(), Symbol{
            name: stmt.name.clone(),
            symbol_type: if !stmt.type_annotation.is_empty() { stmt.type_annotation.clone() } else { value_type },
            is_mutable: true,
            is_function: false,
            signature: None,
            ownership_mode: "owned".to_string(),
            is_borrowed: false,
            borrow_count: 0,
            line: stmt.node.line,
            column: stmt.node.column,
        },
        );

    }
    pub fn analyze_function_decl(&mut self, stmt: &FunctionDecl) {
        // Analyze function declaration// 
        if self.current_scope.exists_in_current(stmt.name.clone()) {
            self.error(format!("Function '{}' already defined", stmt.name), stmt.node.line, stmt.node.column);

        // Define function symbol in current scope
        }
        // Create signature
        let mut param_types = vec![];
        for param in &stmt.parameters {
            param_types.push(if param.type_annotation.is_empty() { "any".to_string() } else { param.type_annotation.clone() });
        }
        let return_type = if stmt.return_type.is_empty() { "void".to_string() } else { stmt.return_type.clone() };

        self.current_scope.define(stmt.name.clone(), Symbol{
            name: stmt.name.clone(),
            symbol_type: "function".to_string(),
            is_mutable: false,
            is_function: true,
            signature: Some(MethodSignature{ params: param_types, return_type: return_type }),
            ownership_mode: "owned".to_string(),
            is_borrowed: false,
            borrow_count: 0,
            line: stmt.node.line,
            column: stmt.node.column,
        },
        );

        // Enter function scope
        self.enter_scope();

        // Define parameters
        for param in &stmt.parameters {
            self.current_scope.define(param.name.clone(), Symbol{
                name: param.name.clone(),
                symbol_type: if param.type_annotation.is_empty() { "any".to_string() } else { param.type_annotation.clone() },
                is_mutable: false,
                is_function: false,
                signature: None,
                ownership_mode: "owned".to_string(),
                is_borrowed: false,
                borrow_count: 0,
                line: stmt.node.line,
                column: stmt.node.column,
            },
            );

        // Analyze body
        }
        for s in &stmt.body {
            self.analyze_statement(s);

        }
        self.exit_scope();

    }
    pub fn analyze_if_stmt(&mut self, stmt: &IfStmt) {
        // Analyze if statement// 
        let cond_type = self.analyze_expression(&stmt.condition);

        // Check condition type (should be boolean or convertible)
        if !cond_type.is_empty() && cond_type != "bool".to_string() && cond_type != "any".to_string() {
             // Warning only for now
            // pass

        }
        self.enter_scope();
        for s in &stmt.then_body {
            self.analyze_statement(s);
        }
        self.exit_scope();

        // Analyze elif branches
        for elif_clause in &stmt.elif_clauses {
            let elif_cond_type = self.analyze_expression(&elif_clause.condition);
            self.enter_scope();
            for s in &elif_clause.body {
                self.analyze_statement(s);
            }
            self.exit_scope();

        // Analyze else branch
        }
        if (stmt.else_body).len() > 0 {
            self.enter_scope();
            for s in &stmt.else_body {
                self.analyze_statement(s);
            }
            self.exit_scope();

        }
    }
    pub fn analyze_while_stmt(&mut self, stmt: &WhileStmt) {
        // Analyze while loop// 
        let cond_type = self.analyze_expression(&stmt.condition);

        self.enter_scope();
        for s in &stmt.body {
            self.analyze_statement(s);
        }
        self.exit_scope();

    }
    pub fn analyze_for_stmt(&mut self, stmt: &ForStmt) {
        // Analyze for loop// 
        let iterable_type = self.analyze_expression(&stmt.iterable);

        self.enter_scope();

        // Define loop variable with inferred element type
        let element_type = if iterable_type.starts_with("list<") {
            // Extract element type from list<T>
            iterable_type.trim_start_matches("list<").trim_end_matches(">").to_string()
        } else if iterable_type.starts_with("set<") {
            iterable_type.trim_start_matches("set<").trim_end_matches(">").to_string()
        } else if iterable_type == "str" || iterable_type == "string" {
            "str".to_string() // Iterating over string yields characters as strings
        } else {
            "any".to_string() // Fallback for unknown iterables
        };
        
        self.current_scope.define(stmt.variable.clone(), Symbol{
            name: stmt.variable.clone(),
            symbol_type: element_type,
            is_mutable: false,
            is_function: false,
            signature: None,
            ownership_mode: "owned".to_string(),
            is_borrowed: false,
            borrow_count: 0,
            line: stmt.node.line,
            column: stmt.node.column,
        },
        );

        for s in &stmt.body {
            self.analyze_statement(s);

        }
        self.exit_scope();

    }
    pub fn analyze_return_stmt(&mut self, stmt: &ReturnStmt) {
        // Analyze return statement// 
        if let Some(val) = &stmt.value {
            self.analyze_expression(val);

        }
    }
    pub fn analyze_assignment(&mut self, stmt: &AssignmentStmt) {
        // Analyze assignment// 
        let target_type = self.analyze_expression(&stmt.target);
        let value_type = self.analyze_expression(&stmt.value);

        // Check type compatibility between target and value
        if !target_type.is_empty() && !value_type.is_empty() && target_type != "any" && value_type != "any" {
            if !self.types_compatible(target_type.clone(), value_type.clone()) {
                self.error(format!("Type mismatch in assignment: cannot assign {} to {}", value_type, target_type),
                    stmt.node.line, stmt.node.column);
            }
        }

        // If target is an identifier, check mutability
        if let Expression::Identifier(ident) = &stmt.target {
            let symbol_opt = self.current_scope.resolve(ident.name.clone());

            if symbol_opt.is_none() {
                self.error(format!("Undefined variable '{}'", ident.name), stmt.node.line, stmt.node.column);
            }
            else {
                let symbol = symbol_opt.unwrap();
                if ! symbol.is_mutable {
                    self.error(format!("Cannot assign to immutable variable '{}'", ident.name),
                            stmt.node.line, stmt.node.column);

                }
            }
        }
    }
    pub fn analyze_expression_stmt(&mut self, stmt: &ExpressionStmt) {
        // Analyze expression statement// 
        self.analyze_expression(&stmt.expression);

    }
    pub fn analyze_import(&mut self, stmt: &ImportStmt) {
        // Analyze import statement
        // Validate against known standard library modules
        let std_libs = vec!["std", "math", "io", "tensor", "simd", "flow", "alloc", "data", "ml"]; // simplified list
        
        if stmt.module_path.is_empty() {
             self.error("Empty module path".to_string(), stmt.node.line, stmt.node.column);
             return;
        }

        let root = &stmt.module_path[0];
        // Allow 'std' or specific known roots
        if root == "std" {
             if stmt.module_path.len() > 1 {
                 let sub = &stmt.module_path[1];
                 if !std_libs.contains(&sub.as_str()) && sub != "box" && sub != "rc" && sub != "arc" && sub != "refcell" && sub != "sync" && sub != "mem" && sub != "thread" {
                      // Warn only for now as we might add more std libs
                      // self.warn(format!("Unknown standard library module 'std.{}'", sub), stmt.node.line, stmt.node.column);
                 }
             }
        } else if root.starts_with('@') {
             // Foreign imports or decorators, ignore
        } else {
             // Local import check (stub)
        }
    }
    // ========================================
    // EXPRESSION ANALYSIS
    // ========================================

    pub fn analyze_expression(&mut self, expr: &Expression)  ->  String {
        // Analyze an expression && return its type// 
        match expr {
            Expression::Literal(l) => {
                if l.value_type == TokenType::Integer { return "int".to_string() }
                else if l.value_type == TokenType::Float { return "float".to_string() }
                else if l.value_type == TokenType::String { return "str".to_string() }
                else if l.value_type == TokenType::TrueKeyword || l.value_type == TokenType::FalseKeyword { return "bool".to_string() }
                else { return "any".to_string() }
            },
            Expression::Identifier(e) => self.analyze_identifier(e),
            Expression::BinaryOp(e) => self.analyze_binary_op(e),
            Expression::UnaryOp(e) => self.analyze_unary_op(e),
            Expression::Call(e) => self.analyze_call(e),
            Expression::Index(e) => self.analyze_index(e),
            Expression::Attribute(e) => self.analyze_attribute(e),
            Expression::List(e) => self.analyze_list_literal(e),
            Expression::Tuple(e) => self.analyze_tuple_literal(e),
            Expression::Set(e) => self.analyze_set_literal(e),
            Expression::Dict(e) => self.analyze_dict_literal(e),
            Expression::TypeConstructor(e) => self.analyze_type_constructor(e),
            Expression::Table(e) => self.analyze_table_literal(e),
            Expression::Await(e) => self.analyze_await_expression(e),
            _ => "any".to_string(),

        }
    }
    pub fn analyze_identifier(&mut self, expr: &IdentifierExpr)  ->  String {
        // Analyze identifier && return type// 
        let symbol_opt = self.current_scope.resolve(expr.name.clone());

        if symbol_opt.is_none() {
            // Allow @-prefixed type constructors as valid
            if expr.name.starts_with('@') {
                match expr.name.as_str() {
                    "@int" => return "int".to_string(),
                    "@flt" | "@float" => return "flt".to_string(),
                    "@str" | "@string" => return "str".to_string(),
                    "@bool" => return "bool".to_string(),
                    _ => {}
                }
            }
            self.error(format!("Undefined variable '{}'", expr.name), expr.node.line, expr.node.column);
            return "any".to_string();
        }
        
        // Borrow checking: check for use after move
        if let Err(e) = self.current_scope.check_use_after_move(&expr.name) {
            self.error(e, expr.node.line, expr.node.column);
        }
        
        return symbol_opt.unwrap().symbol_type;
    }
    pub fn analyze_binary_op(&mut self, expr: &BinaryOpExpr)  ->  String {
        // Analyze binary operation// 
        let left_type = self.analyze_expression(&expr.left);
        let right_type = self.analyze_expression(&expr.right);

        // Type checking for operators
        if expr.operator == TokenType::Plus ||
        expr.operator == TokenType::Minus ||
        expr.operator == TokenType::Star ||
        expr.operator == TokenType::Slash {
            // Arithmetic operators
            if left_type == "int".to_string() && right_type == "int".to_string() {
                return "int".to_string();
            }
            else if left_type == "float".to_string() && right_type == "float".to_string() {
                return "float".to_string();
            }
            else if (left_type == "int".to_string() && right_type == "float".to_string()) ||
                    (left_type == "float".to_string() && right_type == "int".to_string()) {
                self.error(format!("Type mismatch: binary op {:?} requires same types (got {} and {}). Use explicit cast.", 
                    expr.operator, left_type, right_type), expr.node.line, expr.node.column);
                return "any".to_string(); // Poison
            }
            else {
                return "any".to_string();

            }
        }
        else if expr.operator == TokenType::EqualEqual ||
            expr.operator == TokenType::NotEqual ||
            expr.operator == TokenType::Less ||
            expr.operator == TokenType::Greater ||
            expr.operator == TokenType::LessEq ||
            expr.operator == TokenType::GreaterEq {
            // Comparison operators
            return "bool".to_string();
        }
        else if expr.operator == TokenType::And || expr.operator == TokenType::Or {
            // Logical operators
            return "bool".to_string();

        }
        else {
            return "any".to_string();

        }
    }
    pub fn analyze_unary_op(&mut self, expr: &UnaryOpExpr)  ->  String {
        // Analyze unary operation// 
        let operand_type = self.analyze_expression(&expr.operand);

        if expr.operator == TokenType::Minus {
            return operand_type; // Negation preserves type
        }
        else if expr.operator == TokenType::Not {
            return "bool".to_string();
        }
        else {
            return "any".to_string();

        }
    }
    pub fn analyze_call(&mut self, expr: &CallExpr)  ->  String {
        // Analyze function call
        // Analyze callee
        let callee_type = self.analyze_expression(&expr.callee);
        
        let mut return_type = "any".to_string();
        
        // Helper to check signature if available
        if let Expression::Identifier(ident) = &*expr.callee {
             if let Some(symbol) = self.current_scope.resolve(ident.name.clone()) {
                 if let Some(sig) = symbol.signature {
                     return_type = sig.return_type.clone();
                     
                     // Check arg count
                     if sig.params.len() != expr.arguments.len() {
                         self.error(format!("Function '{}' expects {} arguments, got {}", 
                             ident.name, sig.params.len(), expr.arguments.len()), 
                             expr.node.line, expr.node.column);
                     } else {
                         // Check arg types
                         for (i, arg) in expr.arguments.iter().enumerate() {
                             let arg_type = self.analyze_expression(arg);
                             let expected_type = &sig.params[i];
                             
                             if !self.types_compatible(expected_type.clone(), arg_type.clone()) {
                                 let node = arg.get_node();
                                 self.error(format!("Argument {} of '{}' mismatch: expected {}, got {}", 
                                     i+1, ident.name, expected_type, arg_type), 
                                     node.line, node.column);
                             }
                         }
                         return return_type; // Use helper return
                     }
                 }
             }
        }

        // Fallback if no signature check happened (still analyze args)
        for arg in &expr.arguments {
            self.analyze_expression(arg);
        }
        return return_type; 

    }
    pub fn analyze_index(&mut self, expr: &IndexExpr)  ->  String {
        // Analyze index operation// 
        let object_type = self.analyze_expression(&expr.object);
        let index_type = self.analyze_expression(&expr.index);

        // Infer element type from collection type
        if object_type.starts_with("list<") {
            return object_type.trim_start_matches("list<").trim_end_matches(">").to_string();
        } else if object_type.starts_with("dict<") {
            // dict<K,V> - indexing returns V
            if let Some(comma_pos) = object_type.find(',') {
                return object_type[comma_pos+1..].trim_end_matches(">").trim().to_string();
            }
        } else if object_type == "str" || object_type == "string" {
            return "str".to_string(); // String indexing returns character
        } else if object_type == "table" {
            return "float".to_string(); // Table cells are floats
        }
        return "any".to_string();

    }
    pub fn analyze_attribute(&mut self, expr: &AttributeExpr)  ->  String {
        // Analyze attribute access// 
        let object_type = self.analyze_expression(&expr.object);

        // Check if attribute exists - for now, return any for unknown types
        // Known struct types would be looked up in a type registry
        if object_type == "table" {
            // Table has known attributes
            if expr.attribute == "col_count" || expr.attribute == "row_count" {
                return "int".to_string();
            }
        }
        return "any".to_string();

    }
    pub fn analyze_list_literal(&mut self, expr: &ListExpr)  ->  String {
        // Analyze list literal// 
        for elem in &expr.elements {
            self.analyze_expression(elem);
        }
        return "list".to_string();

    }
    pub fn analyze_tuple_literal(&mut self, expr: &TupleExpr)  ->  String {
        // Analyze tuple literal// 
        for elem in &expr.elements {
            self.analyze_expression(elem);
        }
        return "tuple".to_string();

    }
    pub fn analyze_set_literal(&mut self, expr: &SetExpr)  ->  String {
        // Analyze set literal// 
        for elem in &expr.elements {
            self.analyze_expression(elem);
        }
        return "set".to_string();

    }
    pub fn analyze_dict_literal(&mut self, expr: &DictExpr)  ->  String {
        // Analyze dict literal// 
        for pair in &expr.pairs {
            self.analyze_expression(&pair.0);
            self.analyze_expression(&pair.1);
        }
        return "dict".to_string();

    }
    pub fn analyze_type_constructor(&mut self, expr: &TypeConstructorExpr)  ->  String {
        // Analyze type constructor// 
        self.analyze_expression(&expr.argument);
        return expr.type_name.clone();
    }

    pub fn analyze_table_literal(&mut self, expr: &TableExpr) -> String {
        // Analyze table literal
        // Validate columns
        let col_count = expr.columns.len();
        
        // Validate rows
        for row in &expr.rows {
            if row.values.len() != col_count {
                 self.error(format!("Row '{}' has {} values, expected {} (columns)", 
                     row.name, row.values.len(), col_count), 
                     expr.node.line, expr.node.column);
            }
            for val in &row.values {
                self.analyze_expression(val);
            }
        }
        return "table".to_string();
    }

    pub fn analyze_await_expression(&mut self, expr: &AwaitExpr) -> String {
        // Analyze await <expr>
        let val_type = self.analyze_expression(&expr.value);
        // Check if val_type is awaitable (Future, Promise, async return)
        if val_type.starts_with("Future<") {
            // Unwrap Future<T> to T
            return val_type.trim_start_matches("Future<").trim_end_matches(">").to_string();
        } else if val_type.starts_with("Promise<") {
            return val_type.trim_start_matches("Promise<").trim_end_matches(">").to_string();
        }
        // For other types, await just returns the value (simplified)
        return val_type;
    }

    // ========================================
    // HELPER METHODS
    // ========================================

    pub fn types_compatible(&self, expected: String, actual: String)  ->  bool {
        // Check if two types are compatible
        if expected == actual {
            return true;
        }
        if expected == "any".to_string() || actual == "any".to_string() {
            return true;
        }
        
        // Allow float to accept int (widening) if desired, or keep strict. 
        // GUL spec implies explicit casting, but for builtins we might want flexibility.
        // For now, keep strict but allow aliases if we had them.
        
        if expected == "str".to_string() && (actual == "int".to_string() || actual == "float".to_string()) {
             return true; // Auto convert
        }
        // bool <- str logic requires runtime check if value is "true"/"false", but at static analysis time we only know it's "str".
        // Spec: @str "true" and "false" can auto convert to @bool.
        // Static analyzer sees type "str". Does it see the value? strict type check says str != bool.
        // Unless we implement literal analysis.
        // For now, allow str -> bool conversion and rely on runtime/codegen to handle it?
        // Or only if it's a string literal?
        if expected == "bool".to_string() && actual == "str".to_string() {
             return true; // Allow potential conversion
        }

        return false;
    }
// Public API
    pub fn analyze_struct_decl(&mut self, stmt: &StructDecl) {
        // Analyze struct declaration
        if self.current_scope.exists_in_current(stmt.name.clone()) {
            self.error(format!("Struct '{}' already defined", stmt.name), stmt.node.line, stmt.node.column);
        }
        
        // Define struct symbol
        self.current_scope.define(stmt.name.clone(), Symbol{
            name: stmt.name.clone(),
            symbol_type: "struct".to_string(), 
            is_mutable: false,
            is_function: false,
            signature: None,
            ownership_mode: "owned".to_string(),
            is_borrowed: false,
            borrow_count: 0,
            line: stmt.node.line,
            column: stmt.node.column,
        });
        
        // Enter struct scope (namespace for methods)
        self.enter_scope();
        
        // Analyze fields
        for field in &stmt.fields {
            // Check if type exists?
            // For now just accept type annotation string
        }
        
        // Analyze methods
        for method in &stmt.methods {
            self.analyze_function_decl(method);
        }
        
        self.exit_scope();
    }

    pub fn analyze_foreign_block(&mut self, stmt: &ForeignCodeBlock) {
        // Analyze foreign code block
        // Mostly opaque to analyzer, just pass.
        let valid_langs = vec!["python", "rust", "js", "sql"];
        if !valid_langs.contains(&stmt.language.as_str()) {
            self.warn(format!("Unknown foreign language '{}'", stmt.language), stmt.node.line, stmt.node.column);
        }
    }
}
pub fn analyze_semantics(program: Program)  ->  Vec<String> {
    // Perform semantic analysis on program, return errors// 
    let mut sem_analyzer = create_analyzer();
    sem_analyzer.analyze_program(program);
    return sem_analyzer.errors;
}