// GUL v3.2 Compiler - Semantic Analyzer
// Type checking, scope management, and semantic validation

use std::collections::{HashMap, HashSet};
use crate::ast::nodes::*;
use crate::lexer::token::*;

// Symbol table for scope management
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: String,
    pub is_mutable: bool,
    pub is_function: bool,
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

// Semantic analyzer
    }
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
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("println".to_string(), Symbol{
        name: "println".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("str".to_string(), Symbol{
        name: "str".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("int".to_string(), Symbol{
        name: "int".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("len".to_string(), Symbol{
        name: "len".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        line: 0,
        column: 0,
    });
    analyzer.current_scope.symbols.insert("input".to_string(), Symbol{
        name: "input".to_string(),
        symbol_type: "fn".to_string(),
        is_mutable: false,
        is_function: true,
        line: 0,
        column: 0,
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
        self.current_scope.define(stmt.name.clone(), Symbol{
            name: stmt.name.clone(),
            symbol_type: "function".to_string(),
            is_mutable: false,
            is_function: true,
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

        // Define loop variable
        self.current_scope.define(stmt.variable.clone(), Symbol{
            name: stmt.variable.clone(),
            symbol_type: "any".to_string(), // TODO: infer from iterable type
            is_mutable: false,
            is_function: false,
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

        // TODO: Add type compatibility check between target_type and value_type

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
        // Analyze import statement// 
        // pass // For now, just note that it exists
        // TODO: Validate module exists

    // ========================================
    // EXPRESSION ANALYSIS
    // ========================================

    }
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
            else if left_type == "float".to_string() || right_type == "float".to_string() {
                return "float".to_string();
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
        // Analyze function call// 
        // Analyze callee
        let callee_type = self.analyze_expression(&expr.callee);

        // Analyze arguments
        for arg in &expr.arguments {
            self.analyze_expression(arg);

        // TODO: Check function signature matches
        }
        return "any".to_string(); // Unknown return type for now

    }
    pub fn analyze_index(&mut self, expr: &IndexExpr)  ->  String {
        // Analyze index operation// 
        let object_type = self.analyze_expression(&expr.object);
        let index_type = self.analyze_expression(&expr.index);

        // TODO: Better type inference
        return "any".to_string();

    }
    pub fn analyze_attribute(&mut self, expr: &AttributeExpr)  ->  String {
        // Analyze attribute access// 
        let object_type = self.analyze_expression(&expr.object);

        // TODO: Check if attribute exists on type
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

    // ========================================
    // HELPER METHODS
    // ========================================

    }
    pub fn types_compatible(&self, expected: String, actual: String)  ->  bool {
        // Check if two types are compatible// 
        if expected == actual {
            return true;
        }
        if expected == "any".to_string() || actual == "any".to_string() {
            return true;
        // TODO: Add more sophisticated type checking
        }
        return false;

// Public API
    }
}
pub fn analyze_semantics(program: Program)  ->  Vec<String> {
    // Perform semantic analysis on program, return errors// 
    let mut sem_analyzer = create_analyzer();
    sem_analyzer.analyze_program(program);
    return sem_analyzer.errors;
}