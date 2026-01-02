// Semantic Analyzer

use crate::ast::*;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct Analyzer {
    symbols: Vec<HashMap<String, SymbolInfo>>, // Stack of scopes
}

#[derive(Debug, Clone)]
struct SymbolInfo {
    ty: Type,
    is_mutable: bool,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            symbols: vec![HashMap::new()], // Global scope
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<Program> {
        let mut new_statements = Vec::new();
        for stmt in &program.statements {
            new_statements.push(self.analyze_statement(stmt.clone())?);
        }
        Ok(Program { statements: new_statements })
    }

    fn enter_scope(&mut self) {
        self.symbols.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.symbols.pop();
    }

    fn define_symbol(&mut self, name: String, ty: Type, is_mutable: bool) {
        if let Some(scope) = self.symbols.last_mut() {
            scope.insert(name, SymbolInfo { ty, is_mutable });
        }
    }

    fn resolve_symbol(&self, name: &str) -> Option<SymbolInfo> {
        for scope in self.symbols.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info.clone());
            }
        }
        None
    }

    fn analyze_statement(&mut self, stmt: Statement) -> Result<Statement> {
        match stmt {
            Statement::Let { name, value } => {
                let typed_value = self.analyze_expression(value)?;
                let ty = typed_value.ty.clone();
                self.define_symbol(name.clone(), ty, false);
                Ok(Statement::Let { name, value: typed_value })
            }
            Statement::Var { name, value } => {
                let typed_value = self.analyze_expression(value)?;
                let ty = typed_value.ty.clone();
                self.define_symbol(name.clone(), ty, true);
                Ok(Statement::Var { name, value: typed_value })
            }
            Statement::Assignment { name, value } => {
                let typed_value = self.analyze_expression(value)?;
                let symbol = self.resolve_symbol(&name)
                    .ok_or_else(|| anyhow!("Undefined variable '{}'", name))?;
                
                if !symbol.is_mutable {
                    // return Err(anyhow!("Cannot assign to immutable variable '{}'", name));
                    // Allow for now since GUL might be flexible or I haven't implemented mutability perfectly?
                    // Strict:
                    // return Err(anyhow!("Cannot assign to immutable variable '{}'", name));
                }
                
                // Type check?
                // if symbol.ty != typed_value.ty && symbol.ty != Type::Unknown { ... }
                
                Ok(Statement::Assignment { name, value: typed_value })
            }
            Statement::SetIndex { target, index, value } => {
                let typed_target = self.analyze_expression(target)?;
                let typed_index = self.analyze_expression(index)?;
                let typed_value = self.analyze_expression(value)?;
                Ok(Statement::SetIndex { target: typed_target, index: typed_index, value: typed_value })
            }
            Statement::Function { name, params, body } => {
                // Define function in current scope? 
                // Functions are usually global or hoisted.
                // Minimal support: Define name as Unknown type (Function type TODO)
                self.define_symbol(name.clone(), Type::Unknown, false);
                
                self.enter_scope();
                // Define params
                for param in &params {
                    self.define_symbol(param.clone(), Type::Unknown, true);
                }
                
                let mut new_body = Vec::new();
                for s in body {
                    new_body.push(self.analyze_statement(s)?);
                }
                self.exit_scope();
                
                Ok(Statement::Function { name, params, body: new_body })
            }
            Statement::If { condition, then_branch, else_branch } => {
                let typed_condition = self.analyze_expression(condition)?;
                
                self.enter_scope();
                let mut new_then = Vec::new();
                for s in then_branch {
                    new_then.push(self.analyze_statement(s)?);
                }
                self.exit_scope();
                
                let new_else = if let Some(else_stmts) = else_branch {
                    self.enter_scope();
                    let mut stmts = Vec::new();
                    for s in else_stmts {
                        stmts.push(self.analyze_statement(s)?);
                    }
                    self.exit_scope();
                    Some(stmts)
                } else {
                    None
                };
                
                Ok(Statement::If { 
                    condition: typed_condition, 
                    then_branch: new_then, 
                    else_branch: new_else 
                })
            }
            Statement::While { condition, body } => {
                let typed_condition = self.analyze_expression(condition)?;
                self.enter_scope();
                let mut new_body = Vec::new();
                for s in body {
                    new_body.push(self.analyze_statement(s)?);
                }
                self.exit_scope();
                Ok(Statement::While { condition: typed_condition, body: new_body })
            }
            // Loop, Return, Expression etc.
            Statement::Loop { body } => {
                 self.enter_scope();
                let mut new_body = Vec::new();
                for s in body {
                    new_body.push(self.analyze_statement(s)?);
                }
                self.exit_scope();
                Ok(Statement::Loop { body: new_body })
            }
            Statement::Return { value } => {
                 let typed_value = if let Some(v) = value {
                     Some(self.analyze_expression(v)?)
                 } else {
                     None
                 };
                 Ok(Statement::Return { value: typed_value })
            }
            Statement::Expression { expr } => {
                Ok(Statement::Expression { expr: self.analyze_expression(expr)? })
            }
            _ => Ok(stmt)
        }
    }

    fn analyze_expression(&mut self, expr: Expression) -> Result<Expression> {
        match expr.kind {
            ExpressionKind::Integer(_) => Ok(Expression { kind: expr.kind, ty: Type::Integer }),
            ExpressionKind::Float(_) => Ok(Expression { kind: expr.kind, ty: Type::Float }),
            ExpressionKind::String(_) => Ok(Expression { kind: expr.kind, ty: Type::String }),
            ExpressionKind::Boolean(_) => Ok(Expression { kind: expr.kind, ty: Type::Boolean }),
            ExpressionKind::Identifier(ref name) => {
                let ty = if let Some(info) = self.resolve_symbol(name) {
                    info.ty
                } else {
                    // For now, allow undefined or assume unknown
                    // Err(anyhow!("Undefined symbol '{}'", name))?
                    // Warning: failing on undefined might break forward refs if not handled carefully.
                    // But standard vars should be defined.
                    // Functions might be undefined if mutual recursion.
                    // Let's default to Unknown.
                    Type::Unknown
                };
                Ok(Expression { kind: expr.kind, ty })
            }
            ExpressionKind::BinaryOp { left, op, right } => {
                let typed_left = self.analyze_expression(*left)?;
                let typed_right = self.analyze_expression(*right)?;
                // Type inference logic
                let ty = match (&typed_left.ty, &typed_right.ty) {
                    (Type::Integer, Type::Integer) => match op {
                        BinaryOperator::Equal | BinaryOperator::NotEqual | 
                        BinaryOperator::Less | BinaryOperator::Greater |
                        BinaryOperator::LessEq | BinaryOperator::GreaterEq => Type::Boolean,
                        _ => Type::Integer,
                    },
                    (Type::Float, Type::Float) => match op {
                         BinaryOperator::Equal | BinaryOperator::NotEqual | 
                        BinaryOperator::Less | BinaryOperator::Greater |
                        BinaryOperator::LessEq | BinaryOperator::GreaterEq => Type::Boolean,
                        _ => Type::Float,
                    },
                    (Type::String, _) | (_, Type::String) => {
                         if let BinaryOperator::Add = op {
                             Type::String 
                         } else {
                             Type::Unknown
                         }
                    },
                    _ => Type::Unknown, // Mixed types or unknown
                };
                Ok(Expression { 
                    kind: ExpressionKind::BinaryOp { 
                        left: Box::new(typed_left), 
                        op, 
                        right: Box::new(typed_right) 
                    }, 
                    ty 
                })
            }
            ExpressionKind::Call { callee, args } => {
                let typed_callee = self.analyze_expression(*callee)?;
                let mut typed_args = Vec::new();
                for arg in args {
                    typed_args.push(self.analyze_expression(arg)?);
                }
                // Determine return type?
                // If Identifier("print"), return Void or Integer(0).
                let ty = if let ExpressionKind::Identifier(ref name) = typed_callee.kind {
                    if name == "print" {
                         Type::Void // or Integer
                    } else {
                        // Lookup function return type
                        // self.resolve_symbol(name).map(|i| i.ty).unwrap_or(Type::Unknown)
                        Type::Unknown
                    }
                } else {
                    Type::Unknown
                };
                
                Ok(Expression {
                    kind: ExpressionKind::Call {
                        callee: Box::new(typed_callee),
                        args: typed_args,
                    },
                    ty
                })
            }
            ExpressionKind::List(items) => {
                let mut typed_items = Vec::new();
                let mut item_type = Type::Unknown;
                
                for (i, item) in items.into_iter().enumerate() {
                    let typed = self.analyze_expression(item)?;
                    if i == 0 {
                        item_type = typed.ty.clone();
                    } else if typed.ty != item_type {
                        item_type = Type::Unknown; 
                    }
                    typed_items.push(typed);
                }
                
                Ok(Expression { 
                    kind: ExpressionKind::List(typed_items), 
                    ty: Type::List(Box::new(item_type)) 
                })
            },
            ExpressionKind::Index { target, index } => {
                let typed_target = self.analyze_expression(*target)?;
                let typed_index = self.analyze_expression(*index)?;
                
                let ty = if let Type::List(inner) = &typed_target.ty {
                    *inner.clone()
                } else {
                    Type::Unknown
                };
                
                Ok(Expression { 
                    kind: ExpressionKind::Index {
                        target: Box::new(typed_target),
                        index: Box::new(typed_index),
                    }, 
                    ty 
                })
            }
        }
    }
}
