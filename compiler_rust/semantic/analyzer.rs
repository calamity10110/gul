// Auto-generated from GUL source
#![allow(unused_variables, dead_code, unused_mut)]

use std::collections::{HashMap, HashSet, VecDeque};

// GUL v3.2 Compiler - Semantic Analyzer
// Type checking, scope management, and semantic validation

use std::collections;
use compiler::ast::nodes;
use compiler::lexer::token;

// Symbol table for scope management
struct Symbol {
    name: @str
    type: @str
    is_mutable: @bool
    is_function: @bool
    line: @int
    column: @int

struct Scope {
    symbols: @dict  # name -> Symbol
    parent: Scope  # Can be Option::None
    
fn create() -> Scope {
        return Scope{
            symbols: @dict{},
            parent: Option::None
        }
    
fn create_child(parent { Scope) -> Scope:
        return Scope{
            symbols: @dict{},
            parent: parent
        }
    
fn define(&mut &self, name { String, symbol: Symbol):
        """Define a symbol in this scope"""
        self.symbols[name] = symbol
    
fn resolve(&self, name { String) -> Symbol:
        """Resolve a symbol, checking parent scopes"""
        if name in self.symbols {
            return self.symbols[name]
        
        if self.parent {
            return self.parent.resolve(name)
        
        return Option::None  # Not found
    
fn exists_in_current(&self, name { String) -> bool:
        """Check if symbol exists in current scope only"""
        return name in self.symbols

// Semantic analyzer
struct SemanticAnalyzer {
    current_scope: Scope
    errors: @list[@str]
    warnings: @list[@str]
    
fn create() -> SemanticAnalyzer {
        return SemanticAnalyzer{
            current_scope: Scope.create(),
            errors: @list[],
            warnings: @list[]
        }
    
fn analyze_program(&mut &self, program { Program):
        """Analyze entire program"""
        // Analyze imports first
        for import_stmt in program.imports {
            self.analyze_import(import_stmt)
        
        // Analyze statements
        for stmt in program.statements {
            self.analyze_statement(stmt)
        
        // Analyze main entry point
        for stmt in program.main_entry {
            self.analyze_statement(stmt)
    
fn enter_scope(&mut &self) {
        """Enter a new scope"""
        self.current_scope = Scope.create_child(self.current_scope)
    
fn exit_scope(&mut &self) {
        """Exit current scope"""
        if self.current_scope.parent {
            self.current_scope = self.current_scope.parent
    
fn error(&mut &self, message { String, line: i64, column: i64):
        """Record a semantic error"""
        let error_msg = String(format!("Semantic error at {line}:{column}: {message}"));
        self.errors.add(error_msg)
        print(error_msg)
    
fn warn(&mut &self, message { String, line: i64, column: i64):
        """Record a warning"""
        let warn_msg = String(format!("Warning at {line}:{column}: {message}"));
        self.warnings.add(warn_msg)
    
    // ========================================
    // STATEMENT ANALYSIS
    // ========================================
    
fn analyze_statement(&mut &self, stmt { Statement):
        """Analyze a statement"""
        match stmt.stmt_type {
            StmtType.LetDecl => self.analyze_let_stmt(stmt)
            StmtType.VarDecl => self.analyze_var_stmt(stmt)
            StmtType.FunctionDecl => self.analyze_function_decl(stmt)
            StmtType.IfStmt => self.analyze_if_stmt(stmt)
            StmtType.WhileStmt => self.analyze_while_stmt(stmt)
            StmtType.ForStmt => self.analyze_for_stmt(stmt)
            StmtType.ReturnStmt => self.analyze_return_stmt(stmt)
            StmtType.AssignmentStmt => self.analyze_assignment(stmt)
            StmtType.ExpressionStmt => self.analyze_expression_stmt(stmt)
            _ => pass  # Handle other statement types
    
fn analyze_let_stmt(&mut &self, stmt { LetStmt):
        """Analyze let declaration"""
        // Check if already defined in current scope
        if self.current_scope.exists_in_current(stmt.name) {
            self.error(format!("Variable '{stmt.name}' already defined"), stmt.base.node.line, stmt.base.node.column)
        
        // Analyze the value expression
        let value_type = self.analyze_expression(stmt.value);
        
        // Check type annotation if provided
        if stmt.type_annotation && value_type {
            if ! self.types_compatible(stmt.type_annotation, value_type) {
                self.error(format!("Type mismatch: expected {stmt.type_annotation}, got {value_type}"), 
                          stmt.base.node.line, stmt.base.node.column)
        
        // Define the symbol
        self.current_scope.define(stmt.name, Symbol{
            name: stmt.name,
            type: stmt.type_annotation || value_type,
            is_mutable: @bool(false),
            is_function: @bool(false),
            line: stmt.base.node.line,
            column: stmt.base.node.column
        })
    
fn analyze_var_stmt(&mut &self, stmt { VarStmt):
        """Analyze var declaration (similar to let but mutable)"""
        if self.current_scope.exists_in_current(stmt.name) {
            self.error(format!("Variable '{stmt.name}' already defined"), stmt.base.node.line, stmt.base.node.column)
        
        let value_type = self.analyze_expression(stmt.value);
        
        if stmt.type_annotation && value_type {
            if ! self.types_compatible(stmt.type_annotation, value_type) {
                self.error(format!("Type mismatch: expected {stmt.type_annotation}, got {value_type}"), 
                          stmt.base.node.line, stmt.base.node.column)
        
        self.current_scope.define(stmt.name, Symbol{
            name: stmt.name,
            type: stmt.type_annotation || value_type,
            is_mutable: @bool(true),
            is_function: @bool(false),
            line: stmt.base.node.line,
            column: stmt.base.node.column
        })
    
fn analyze_function_decl(&mut &self, stmt { FunctionDecl):
        """Analyze function declaration"""
        if self.current_scope.exists_in_current(stmt.name) {
            self.error(format!("Function '{stmt.name}' already defined"), stmt.base.node.line, stmt.base.node.column)
        
        // Define function in current scope
        self.current_scope.define(stmt.name, Symbol{
            name: stmt.name,
            type: stmt.return_type || "void",
            is_mutable: @bool(false),
            is_function: @bool(true),
            line: stmt.base.node.line,
            column: stmt.base.node.column
        })
        
        // Enter function scope
        self.enter_scope()
        
        // Define parameters in function scope
        for param in stmt.parameters {
            self.current_scope.define(param.name, Symbol{
                name: param.name,
                type: param.type_annotation || "any",
                is_mutable: param.ownership_mode == "ref",
                is_function: @bool(false),
                line: stmt.base.node.line,
                column: stmt.base.node.column
            })
        
        // Analyze function body
        for body_stmt in stmt.body {
            self.analyze_statement(body_stmt)
        
        // Exit function scope
        self.exit_scope()
    
fn analyze_if_stmt(&mut &self, stmt { IfStmt):
        """Analyze if statement"""
        // Analyze condition
        let cond_type = self.analyze_expression(stmt.condition);
        if cond_type && cond_type != "bool" {
            self.warn(format!("Condition should be bool, got {cond_type}"), stmt.base.node.line, stmt.base.node.column)
        
        // Analyze then branch
        self.enter_scope()
        for s in stmt.then_body {
            self.analyze_statement(s)
        self.exit_scope()
        
        // Analyze elif branches
        for elif_clause in stmt.elif_clauses {
            let elif_cond_type = self.analyze_expression(elif_clause.condition);
            self.enter_scope()
            for s in elif_clause.body {
                self.analyze_statement(s)
            self.exit_scope()
        
        // Analyze else branch
        if stmt.else_body {
            self.enter_scope()
            for s in stmt.else_body {
                self.analyze_statement(s)
            self.exit_scope()
    
fn analyze_while_stmt(&mut &self, stmt { WhileStmt):
        """Analyze while loop"""
        let cond_type = self.analyze_expression(stmt.condition);
        
        self.enter_scope()
        for s in stmt.body {
            self.analyze_statement(s)
        self.exit_scope()
    
fn analyze_for_stmt(&mut &self, stmt { ForStmt):
        """Analyze for loop"""
        let iterable_type = self.analyze_expression(stmt.iterable);
        
        self.enter_scope()
        
        // Define loop variable
        self.current_scope.define(stmt.variable, Symbol{
            name: stmt.variable,
            type: "any",  # TODO: infer from iterable type
            is_mutable: @bool(false),
            is_function: @bool(false),
            line: stmt.base.node.line,
            column: stmt.base.node.column
        })
        
        for s in stmt.body {
            self.analyze_statement(s)
        
        self.exit_scope()
    
fn analyze_return_stmt(&mut &self, stmt { ReturnStmt):
        """Analyze return statement"""
        if stmt.value {
            self.analyze_expression(stmt.value)
    
fn analyze_assignment(&mut &self, stmt { AssignmentStmt):
        """Analyze assignment"""
        // Analyze target (must be a variable)
        if stmt.target.expr_type == ExprType.Identifier {
            let ident = stmt.target;
            let symbol = self.current_scope.resolve(ident.name);
            
            if ! symbol {
                self.error(format!("Undefined variable '{ident.name}'"), stmt.base.node.line, stmt.base.node.column)
            else if ! symbol.is_mutable {
                self.error(format!("Cannot assign to immutable variable '{ident.name}'"), 
                          stmt.base.node.line, stmt.base.node.column)
        
        // Analyze value
        self.analyze_expression(stmt.value)
    
fn analyze_expression_stmt(&mut &self, stmt { ExpressionStmt):
        """Analyze expression statement"""
        self.analyze_expression(stmt.expression)
    
fn analyze_import(&mut &self, stmt { ImportStmt):
        """Analyze import statement"""
        // For now, just note that it exists
        // TODO: Validate module exists
        pass
    
    // ========================================
    // EXPRESSION ANALYSIS
    // ========================================
    
fn analyze_expression(&mut &self, expr { Expression) -> String:
        """Analyze expression && return its type"""
        match expr.expr_type {
            ExprType.IntegerLiteral => return "int"
            ExprType.FloatLiteral => return "float"
            ExprType.StringLiteral => return "str"
            ExprType.BooleanLiteral => return "bool"
            ExprType.Identifier => return self.analyze_identifier(expr)
            ExprType.BinaryOp => return self.analyze_binary_op(expr)
            ExprType.UnaryOp => return self.analyze_unary_op(expr)
            ExprType.Call => return self.analyze_call(expr)
            ExprType.Index => return self.analyze_index(expr)
            ExprType.Attribute => return self.analyze_attribute(expr)
            ExprType.ListLiteral => return "list"
            ExprType.TupleLiteral => return "tuple"
            ExprType.SetLiteral => return "set"
            ExprType.DictLiteral => return "dict"
            ExprType.TypeConstructor => return self.analyze_type_constructor(expr)
            _ => return "any"
    
fn analyze_identifier(&mut &self, expr { IdentifierExpr) -> String:
        """Analyze identifier && return type"""
        let symbol = self.current_scope.resolve(expr.name);
        
        if ! symbol {
            self.error(format!("Undefined variable '{expr.name}'"), expr.base.node.line, expr.base.node.column)
            return "any"
        
        return symbol.type
    
fn analyze_binary_op(&mut &self, expr { BinaryOpExpr) -> String:
        """Analyze binary operation"""
        let left_type = self.analyze_expression(expr.left);
        let right_type = self.analyze_expression(expr.right);
        
        // Type checking for operators
        if expr.operator == TokenType.Plus || 
           expr.operator == TokenType.Minus ||
           expr.operator == TokenType.Star ||
           expr.operator == TokenType.Slash {
            // Arithmetic operators
            if left_type == "int" && right_type == "int" {
                return "int"
            else if left_type == "float" || right_type == "float" {
                return "float"
            else {
                return "any"
        
        else if expr.operator == TokenType.EqualEqual ||
             expr.operator == TokenType.NotEqual ||
             expr.operator == TokenType.Less ||
             expr.operator == TokenType.Greater ||
             expr.operator == TokenType.LessEq ||
             expr.operator == TokenType.GreaterEq {
            // Comparison operators
            return "bool"
        
        else if expr.operator == TokenType.And || expr.operator == TokenType.Or {
            // Logical operators
            return "bool"
        
        else {
            return "any"
    
fn analyze_unary_op(&mut &self, expr { UnaryOpExpr) -> String:
        """Analyze unary operation"""
        let operand_type = self.analyze_expression(expr.operand);
        
        if expr.operator == TokenType.Minus {
            return operand_type  # Negation preserves type
        else if expr.operator == TokenType.Not {
            return "bool"
        else {
            return "any"
    
fn analyze_call(&mut &self, expr { CallExpr) -> String:
        """Analyze function call"""
        // Analyze callee
        let callee_type = self.analyze_expression(expr.callee);
        
        // Analyze arguments
        for arg in expr.arguments {
            self.analyze_expression(arg)
        
        // TODO: Check function signature matches
        return "any"  # Unknown return type for now
    
fn analyze_index(&mut &self, expr { IndexExpr) -> String:
        """Analyze index operation"""
        let object_type = self.analyze_expression(expr.object);
        let index_type = self.analyze_expression(expr.index);
        
        // TODO: Better type inference
        return "any"
    
fn analyze_attribute(&mut &self, expr { AttributeExpr) -> String:
        """Analyze attribute access"""
        let object_type = self.analyze_expression(expr.object);
        
        // TODO: Check if attribute exists on type
        return "any"
    
fn analyze_type_constructor(&mut &self, expr { TypeConstructorExpr) -> String:
        """Analyze type constructor"""
        self.analyze_expression(expr.argument)
        return expr.type_name
    
    // ========================================
    // HELPER METHODS
    // ========================================
    
fn types_compatible(&self, expected { String, actual: String) -> bool:
        """Check if two types are compatible"""
        if expected == actual {
            return @bool(true)
        if expected == "any" || actual == "any" {
            return @bool(true)
        // TODO: Add more sophisticated type checking
        return @bool(false)

// Public API
fn analyze_semantics(program { Program) -> Vec[String]:
    """Perform semantic analysis on program, return errors"""
    let mut analyzer = SemanticAnalyzer.create();
    analyzer.analyze_program(program)
    return analyzer.errors
