#![allow(dead_code)]
// Semantic analyzer - type checking and validation

use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Bool,
    List(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Unit(String), // Unit types with name
    Any,          // Gradual typing
    Unknown,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: Type,
    pub is_mutable: bool,
    pub ownership: Ownership,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()], // Global scope
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        if let Some(current_scope) = self.scopes.last_mut() {
            if current_scope.contains_key(&name) {
                return Err(format!(
                    "Symbol '{}' already defined in current scope",
                    name
                ));
            }
            current_scope.insert(name, symbol);
            Ok(())
        } else {
            Err("No active scope".to_string())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }
}

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    errors: Vec<String>,
    current_function_is_async: bool,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            current_function_is_async: false,
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), Vec<String>> {
        for statement in &program.statements {
            self.analyze_statement(statement);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Import(modules) => {
                for module in modules {
                   let _ = module;
                }
            }
            Statement::GlobalDef { name, value, .. } => {
                let value_type = self.infer_type(value);
                let symbol = Symbol {
                    name: name.clone(),
                    symbol_type: value_type,
                    is_mutable: true,
                    ownership: Ownership::Own,
                };
                if let Err(e) = self.symbol_table.define(name.clone(), symbol) {
                    self.errors.push(e);
                }
            }
            Statement::StructDef {
                name,
                fields,
                methods,
            } => {
                let symbol = Symbol {
                    name: name.clone(),
                    symbol_type: Type::Unknown,
                    is_mutable: false,
                    ownership: Ownership::Own,
                };
                if let Err(e) = self.symbol_table.define(name.clone(), symbol) {
                    self.errors.push(e);
                }

                // Analyze methods within struct scope
                self.symbol_table.enter_scope();

                // Define fields in scope (simplified)
                for (field_name, _) in fields {
                    let field_sym = Symbol {
                        name: field_name.clone(),
                        symbol_type: Type::Unknown,
                        is_mutable: true,
                        ownership: Ownership::Ref,
                    };
                    if let Err(e) = self.symbol_table.define(field_name.clone(), field_sym) {
                        self.errors.push(e);
                    }
                }

                for stmt in methods {
                    self.analyze_statement(stmt);
                }
                self.symbol_table.exit_scope();
            }
            Statement::Definition { name, value } => {
                let value_type = self.infer_type(value);
                let symbol = Symbol {
                    name: name.clone(),
                    symbol_type: value_type,
                    is_mutable: false,
                    ownership: Ownership::Own,
                };
                if let Err(e) = self.symbol_table.define(name.clone(), symbol) {
                    self.errors.push(e);
                }
            }
            Statement::Function {
                name,
                params,
                outputs,
                body,
                is_async,
            } => {
                // Enter function scope
                self.symbol_table.enter_scope();
                self.current_function_is_async = *is_async;

                // Define parameters
                for (param_name, _ty) in params {
                    let symbol = Symbol {
                        name: param_name.clone(),
                        symbol_type: Type::Unknown, // Ideally convert ty to semantic::Type
                        is_mutable: true,
                        ownership: Ownership::Own,
                    };
                    if let Err(e) = self.symbol_table.define(param_name.clone(), symbol) {
                        self.errors.push(e);
                    }
                }

                // Define named outputs in scope (v3.2)
                for (out_name, _ty) in outputs {
                    if !out_name.is_empty() {
                         let symbol = Symbol {
                             name: out_name.clone(),
                             symbol_type: Type::Unknown,
                             is_mutable: true,
                             ownership: Ownership::Own,
                         };
                         if let Err(e) = self.symbol_table.define(out_name.clone(), symbol) {
                             self.errors.push(e);
                         }
                    }
                }

                // Analyze body
                for stmt in body {
                    self.analyze_statement(stmt);
                }

                self.symbol_table.exit_scope();
                self.current_function_is_async = false;

                // Define function in outer scope
                let func_symbol = Symbol {
                    name: name.clone(),
                    symbol_type: Type::Function(vec![], Box::new(Type::Unknown)),
                    is_mutable: false,
                    ownership: Ownership::Own,
                };
                if let Err(e) = self.symbol_table.define(name.clone(), func_symbol) {
                    self.errors.push(e);
                }
            }
            Statement::Main { body } => {
                self.symbol_table.enter_scope();
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                self.symbol_table.exit_scope();
            }
            Statement::If {
                condition,
                then_body,
                else_body,
            } => {
                let cond_type = self.infer_type(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown {
                    self.errors.push("If condition must be boolean".to_string());
                }

                self.symbol_table.enter_scope();
                for stmt in then_body {
                    self.analyze_statement(stmt);
                }
                self.symbol_table.exit_scope();

                if let Some(else_stmts) = else_body {
                    self.symbol_table.enter_scope();
                    for stmt in else_stmts {
                        self.analyze_statement(stmt);
                    }
                    self.symbol_table.exit_scope();
                }
            }
            Statement::Loop { body } => {
                self.symbol_table.enter_scope();
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                self.symbol_table.exit_scope();
            }
            Statement::For {
                variable,
                iterable,
                body,
                is_parallel: _,
            } => {
                let _iter_type = self.infer_type(iterable);

                self.symbol_table.enter_scope();

                // Define loop variable
                let symbol = Symbol {
                    name: variable.clone(),
                    symbol_type: Type::Unknown,
                    is_mutable: false,
                    ownership: Ownership::Own,
                };
                if let Err(e) = self.symbol_table.define(variable.clone(), symbol) {
                    self.errors.push(e);
                }

                for stmt in body {
                    self.analyze_statement(stmt);
                }
                self.symbol_table.exit_scope();
            }
            Statement::While {
                condition,
                body,
                is_parallel: _,
            } => {
                let cond_type = self.infer_type(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown {
                    self.errors
                        .push("While condition must be boolean".to_string());
                }

                self.symbol_table.enter_scope();
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                self.symbol_table.exit_scope();
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let _return_type = self.infer_type(e);
                }
            }
            Statement::Assignment { name, value } => {
                // Check if variable exists
                if self.symbol_table.lookup(name).is_none() {
                    self.errors
                        .push(format!("Undefined variable '{}' in assignment", name));
                }
                let _value_type = self.infer_type(value);
            }
            Statement::Expression(expr) => {
                let _expr_type = self.infer_type(expr);
            }
            Statement::ForeignBlock { .. } => {
                // Custom blocks are validated separately
            }
            Statement::Break | Statement::Continue => {
                // Valid in loop context
            }
            Statement::Try {
                try_body,
                catch_body,
                finally_body,
                ..
            } => {
                // Analyze try body
                for stmt in try_body {
                    self.analyze_statement(stmt);
                }
                // Analyze catch body if present
                if let Some(catch) = catch_body {
                    for stmt in catch {
                        self.analyze_statement(stmt);
                    }
                }
                // Analyze finally body if present
                if let Some(finally) = finally_body {
                    for stmt in finally {
                        self.analyze_statement(stmt);
                    }
                }
            }
            Statement::Throw(expr) => {
                let _throw_type = self.infer_type(expr);
                // Throw can throw any type
            }
        }
    }

    fn infer_type(&mut self, expr: &Expression) -> Type {
        match expr {
            Expression::Integer(_) => Type::Integer,
            Expression::Float(_) => Type::Float,
            Expression::String(_) => Type::String,
            Expression::Bool(_) => Type::Bool,
            Expression::Identifier(name) => {
                if let Some(symbol) = self.symbol_table.lookup(name) {
                    symbol.symbol_type.clone()
                } else {
                    self.errors.push(format!("Undefined variable '{}'", name));
                    Type::Unknown
                }
            }
            Expression::Binary { left, op, right } => {
                let left_type = self.infer_type(left);
                let right_type = self.infer_type(right);

                match op {
                    BinaryOp::Add
                    | BinaryOp::Subtract
                    | BinaryOp::Multiply
                    | BinaryOp::Divide
                    | BinaryOp::Modulo
                    | BinaryOp::Power => {
                        if left_type == Type::Integer && right_type == Type::Integer {
                            Type::Integer
                        } else if matches!(left_type, Type::Integer | Type::Float)
                            && matches!(right_type, Type::Integer | Type::Float)
                        {
                            Type::Float
                        } else {
                            Type::Unknown
                        }
                    }
                    BinaryOp::Equal
                    | BinaryOp::NotEqual
                    | BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual => Type::Bool,
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type == Type::Bool && right_type == Type::Bool {
                            Type::Bool
                        } else {
                            Type::Unknown
                        }
                    }
                }
            }
            Expression::Unary { op, operand } => {
                let operand_type = self.infer_type(operand);
                match op {
                    UnaryOp::Negate => operand_type,
                    UnaryOp::Not => Type::Bool,
                }
            }
            Expression::Call { function, args } => {
                let _func_type = self.infer_type(function);
                for arg in args {
                    let _arg_type = self.infer_type(arg);
                }
                Type::Unknown
            }
            Expression::Member { object, .. } => {
                let _obj_type = self.infer_type(object);
                Type::Unknown
            }
            Expression::Index { object, index } => {
                let obj_type = self.infer_type(object);
                let _index_type = self.infer_type(index);

                if let Type::List(elem_type) = obj_type {
                    *elem_type
                } else {
                    Type::Unknown
                }
            }
            Expression::List(elements) => {
                if elements.is_empty() {
                    Type::List(Box::new(Type::Unknown))
                } else {
                    let first_type = self.infer_type(&elements[0]);
                    Type::List(Box::new(first_type))
                }
            }
            Expression::Dict(_) => Type::Dict(Box::new(Type::String), Box::new(Type::Unknown)),
            Expression::UiSprite { .. } => Type::Unknown,
            Expression::Await(expr) => {
                if !self.current_function_is_async {
                    self.errors
                        .push("'await' can only be used in async functions".to_string());
                }
                self.infer_type(expr)
            }
            Expression::Ownership { value, .. } => self.infer_type(value),
            Expression::ListOp { .. } => Type::Unknown,
            Expression::Typed { expr, ty } => {
                // Convert AST Type to semantic Type
                let expected_type = match ty {
                    crate::ast::Type::Int => Type::Integer,
                    crate::ast::Type::Float => Type::Float,
                    crate::ast::Type::String => Type::String,
                    crate::ast::Type::Bool => Type::Bool,
                    crate::ast::Type::List(_) => Type::List(Box::new(Type::Unknown)), // TODO: Handle nested types
                    crate::ast::Type::Dict(_, _) => {
                        Type::Dict(Box::new(Type::Unknown), Box::new(Type::Unknown))
                    }
                    crate::ast::Type::Any => Type::Any,
                    crate::ast::Type::Unit(name) => Type::Unit(name.clone()),
                    crate::ast::Type::Function(_, _) => {
                        Type::Function(vec![], Box::new(Type::Unknown))
                    } // TODO: Handle function types
                };

                // Check if the inferred type matches the annotation
                let inferred = self.infer_type(expr);
                if inferred != Type::Unknown && inferred != expected_type {
                    self.errors.push(format!(
                        "Type mismatch: expected {:?}, got {:?}",
                        expected_type, inferred
                    ));
                }
                expected_type // Return the annotated type
            }
            Expression::Lambda { params, body } => {
                // Infer lambda type - simplified
                // We should ideally push a scope for params
                let _ = body; // used
                Type::Function(vec![Type::Any; params.len()], Box::new(Type::Any))
            }
        }
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// Public API function
pub fn analyze(program: &Program) -> Result<(), String> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer
        .analyze(program)
        .map_err(|errors| errors.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut table = SymbolTable::new();

        let symbol = Symbol {
            name: "x".to_string(),
            symbol_type: Type::Integer,
            is_mutable: false,
            ownership: Ownership::Own,
        };

        table.define("x".to_string(), symbol).unwrap();
        assert!(table.lookup("x").is_some());
        assert!(table.lookup("y").is_none());
    }

    #[test]
    fn test_undefined_variable() {
        let mut analyzer = SemanticAnalyzer::new();

        let program = Program {
            statements: vec![Statement::Expression(Expression::Identifier(
                "undefined".to_string(),
            ))],
        };

        let result = analyzer.analyze(&program);
        assert!(result.is_err());
        assert_eq!(analyzer.errors.len(), 1);
    }

    #[test]
    fn test_valid_definition() {
        let mut analyzer = SemanticAnalyzer::new();

        let program = Program {
            statements: vec![Statement::Definition {
                name: "x".to_string(),
                value: Expression::Integer(42),
            }],
        };

        let result = analyzer.analyze(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_await_in_sync_function() {
        let mut analyzer = SemanticAnalyzer::new();

        let program = Program {
            statements: vec![Statement::Function {
                name: "sync_fn".to_string(),
                params: vec![],
                body: vec![Statement::Expression(Expression::Await(Box::new(
                    Expression::Integer(42),
                )))],
                is_async: false,
            }],
        };

        let result = analyzer.analyze(&program);
        assert!(result.is_err());
        assert!(analyzer
            .errors
            .iter()
            .any(|e| e.contains("await") && e.contains("async")));
    }
}
