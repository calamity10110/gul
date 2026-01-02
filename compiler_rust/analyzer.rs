// GUL v3.2 Compiler - Semantic Analyzer
// Type checking, scope management, and semantic validation

use crate::std::collections;
use crate::compiler::ast::nodes;
use crate::compiler::lexer::token;

// Symbol table for scope management
struct Symbol {
    name: String,
    type: String,
    is_mutable: bool,
    is_function: bool,
    line: i64,
    column: i64,

}
struct Scope {
    symbols: HashMap<String, String>, // name -> Symbol
    parent: Scope, // Can be None

}
fn create_scope()  ->  Scope {
    return Scope{symbols: dict!{}, parent: None}

}
fn create_child_scope(parent: Scope)  ->  Scope {
    return Scope{symbols: dict!{}, parent: parent}
}
impl Scope {
    fn define(&mut self, name: String, symbol: Symbol) {
        // Define a symbol in this scope// 
        self.symbols[name] = symbol;

    }
    fn resolve(&self, name: String)  ->  Symbol {
        // Resolve a symbol, checking parent scopes// 
        if name in self.symbols {
            return self.symbols[name];

        }
        if self.parent {
            return self.parent.resolve(name);

        }
        return None; // Not found

    }
    fn exists_in_current(&self, name: String)  ->  bool {
        // Check if symbol exists in current scope only// 
        return name in self.symbols;

// Semantic analyzer
    }
}
struct SemanticAnalyzer {
    current_scope: Scope,
    errors: vec![String],
    warnings: vec![String],

}
fn create_analyzer()  ->  SemanticAnalyzer {
    return SemanticAnalyzer{current_scope: create_scope(), errors: vec![], warnings: vec![]}

}
impl SemanticAnalyzer {
    fn analyze_program(&mut self, program: Program) {
        // Analyze entire program// 
        // Analyze imports first
        for import_stmt in program.imports {
            self.analyze_import(import_stmt);

        // Analyze statements
        }
        for stmt in program.statements {
            self.analyze_statement(stmt);

        // Analyze main entry point
        }
        for stmt in program.main_entry {
            self.analyze_statement(stmt);

        }
    }
    fn enter_scope(&mut self) {
        // Enter a new scope// 
        self.current_scope = create_child_scope(self.current_scope);

    }
    fn exit_scope(&mut self) {
        // Exit current scope// 
        if self.current_scope.parent {
            self.current_scope = self.current_scope.parent;

        }
    }
    fn error(&mut self, message: String, line: i64, column: i64) {
        // Record a semantic error// 
        let error_msg = "Semantic error at " + format!("{}", line) + ":" + format!("{}", column) + ": " + message;
        self.errors.add(error_msg);
        println!("{}", error_msg);

    }
    fn warn(&mut self, message: String, line: i64, column: i64) {
        // Record a warning// 
        let warn_msg = "Warning at " + format!("{}", line) + ":" + format!("{}", column) + ": " + message;
        self.warnings.add(warn_msg);

    // ========================================
    // STATEMENT ANALYSIS
    // ========================================

    }
    fn analyze_statement(&mut self, stmt: Statement) {
        // Analyze a statement// 
        let st = stmt.stmt_type;
        if st == StmtType.LetDecl { self.analyze_let_stmt((stmt as LetStmt)) }
        else if st == StmtType.VarDecl { self.analyze_var_stmt((stmt as VarStmt)) }
        else if st == StmtType.FunctionDecl { self.analyze_function_decl((stmt as FunctionDecl)) }
        else if st == StmtType.IfStmt { self.analyze_if_stmt((stmt as IfStmt)) }
        else if st == StmtType.WhileStmt { self.analyze_while_stmt((stmt as WhileStmt)) }
        else if st == StmtType.ForStmt { self.analyze_for_stmt((stmt as ForStmt)) }
        else if st == StmtType.ReturnStmt { self.analyze_return_stmt((stmt as ReturnStmt)) }
        else if st == StmtType.AssignmentStmt { self.analyze_assignment((stmt as AssignmentStmt)) }
        else if st == StmtType.ExpressionStmt { self.analyze_expression_stmt((stmt as ExpressionStmt)) }
        else if st == StmtType.ImportStmt { self.analyze_import((stmt as ImportStmt)) }
        else { pass }

    }
    fn analyze_let_stmt(&mut self, stmt: LetStmt) {
        // Analyze let declaration// 
        // Check if already defined in current scope
        if self.current_scope.exists_in_current(stmt.name) {
            format!(self.error("Variable '{}' already defined", stmt.base.node.line, stmt.base.node.column), stmt.name);

        // Analyze the value expression
        }
        let value_type = self.analyze_expression(stmt.value);

        // Check type annotation if provided
        if stmt.type_annotation && value_type {
            if ! self.types_compatible(stmt.type_annotation, value_type) {
                format!(self.error("Type mismatch: expected {}, got {}",, stmt.type_annotation, value_type);
                        stmt.base.node.line, stmt.base.node.column);

        // Define the symbol
            }
        }
        self.current_scope.define(stmt.name, Symbol{
            name: stmt.name,
            type: stmt.type_annotation || value_type,
            is_mutable: bool(false),
            is_function: bool(false),
            line: stmt.base.node.line,
            column: stmt.base.node.column,
        },
        });

    }
    fn analyze_var_stmt(&mut self, stmt: VarStmt) {
        // Analyze var declaration (similar to let but mutable)// 
        if self.current_scope.exists_in_current(stmt.name) {
            format!(self.error("Variable '{}' already defined", stmt.base.node.line, stmt.base.node.column), stmt.name);

        }
        let value_type = self.analyze_expression(stmt.value);

        if stmt.type_annotation && value_type {
            if ! self.types_compatible(stmt.type_annotation, value_type) {
                format!(self.error("Type mismatch: expected {}, got {}",, stmt.type_annotation, value_type);
                        stmt.base.node.line, stmt.base.node.column);

            }
        }
        self.current_scope.define(stmt.name, Symbol{
            name: stmt.name,
            type: stmt.type_annotation || value_type,
            is_mutable: bool(true),
            is_function: bool(false),
            line: stmt.base.node.line,
            column: stmt.base.node.column,
        },
        });

    }
    fn analyze_function_decl(&mut self, stmt: FunctionDecl) {
        // Analyze function declaration// 
        if self.current_scope.exists_in_current(stmt.name) {
            format!(self.error("Function '{}' already defined", stmt.base.node.line, stmt.base.node.column), stmt.name);

        // Define function in current scope
        }
        self.current_scope.define(stmt.name, Symbol{
            name: stmt.name,
            type: stmt.return_type || "void",
            is_mutable: bool(false),
            is_function: bool(true),
            line: stmt.base.node.line,
            column: stmt.base.node.column,
        },
        });

        // Enter function scope
        self.enter_scope();

        // Define parameters in function scope
        for param in stmt.parameters {
            self.current_scope.define(param.name, Symbol{
                name: param.name,
                type: param.type_annotation || "any",
                is_mutable: param.ownership_mode == "ref",
                is_function: bool(false),
                line: stmt.base.node.line,
                column: stmt.base.node.column,
            },
            });

        // Analyze function body
        }
        for body_stmt in stmt.body {
            self.analyze_statement(body_stmt);

        // Exit function scope
        }
        self.exit_scope();

    }
    fn analyze_if_stmt(&mut self, stmt: IfStmt) {
        // Analyze if statement// 
        // Analyze condition
        let cond_type = self.analyze_expression(stmt.condition);
        if cond_type && cond_type != "bool" {
            format!(self.warn("Condition should be bool, got {}", stmt.base.node.line, stmt.base.node.column), cond_type);

        // Analyze then branch
        }
        self.enter_scope();
        for s in stmt.then_body {
            self.analyze_statement(s);
        }
        self.exit_scope();

        // Analyze elif branches
        for elif_clause in stmt.elif_clauses {
            let elif_cond_type = self.analyze_expression(elif_clause.condition);
            self.enter_scope();
            for s in elif_clause.body {
                self.analyze_statement(s);
            }
            self.exit_scope();

        // Analyze else branch
        }
        if stmt.else_body {
            self.enter_scope();
            for s in stmt.else_body {
                self.analyze_statement(s);
            }
            self.exit_scope();

        }
    }
    fn analyze_while_stmt(&mut self, stmt: WhileStmt) {
        // Analyze while loop// 
        let cond_type = self.analyze_expression(stmt.condition);

        self.enter_scope();
        for s in stmt.body {
            self.analyze_statement(s);
        }
        self.exit_scope();

    }
    fn analyze_for_stmt(&mut self, stmt: ForStmt) {
        // Analyze for loop// 
        let iterable_type = self.analyze_expression(stmt.iterable);

        self.enter_scope();

        // Define loop variable
        self.current_scope.define(stmt.variable, Symbol{
            name: stmt.variable,
            type: "any", // TODO: infer from iterable type
            is_mutable: bool(false),
            is_function: bool(false),
            line: stmt.base.node.line,
            column: stmt.base.node.column,
        },
        });

        for s in stmt.body {
            self.analyze_statement(s);

        }
        self.exit_scope();

    }
    fn analyze_return_stmt(&mut self, stmt: ReturnStmt) {
        // Analyze return statement// 
        if stmt.value {
            self.analyze_expression(stmt.value);

        }
    }
    fn analyze_assignment(&mut self, stmt: AssignmentStmt) {
        // Analyze assignment// 
        // Analyze target (must be a variable)
        if stmt.target.expr_type == ExprType.Identifier {
            let ident = (stmt.target as IdentifierExpr);
            let symbol = self.current_scope.resolve(ident.name);

            if ! symbol {
                format!(self.error("Undefined variable '{}'", stmt.base.node.line, stmt.base.node.column), ident.name);
            }
            else if ! symbol.is_mutable {
                format!(self.error("Cannot assign to immutable variable '{}'",, ident.name);
                        stmt.base.node.line, stmt.base.node.column);

        // Analyze value
            }
        }
        self.analyze_expression(stmt.value);

    }
    fn analyze_expression_stmt(&mut self, stmt: ExpressionStmt) {
        // Analyze expression statement// 
        self.analyze_expression(stmt.expression);

    }
    fn analyze_import(&mut self, stmt: ImportStmt) {
        // Analyze import statement// 
        // For now, just note that it exists
        // TODO: Validate module exists
        pass;

    // ========================================
    // EXPRESSION ANALYSIS
    // ========================================

    }
    fn analyze_expression(&mut self, expr: Expression)  ->  String {
        // Analyze expression && return its type// 
        let et = expr.expr_type;
        if et == ExprType.IntegerLiteral { return "int" }
        else if et == ExprType.FloatLiteral { return "float" }
        else if et == ExprType.StringLiteral { return "str" }
        else if et == ExprType.BooleanLiteral { return "bool" }
        else if et == ExprType.Identifier { return self.analyze_identifier((expr as IdentifierExpr)) }
        else if et == ExprType.BinaryOp { return self.analyze_binary_op((expr as BinaryOpExpr)) }
        else if et == ExprType.UnaryOp { return self.analyze_unary_op((expr as UnaryOpExpr)) }
        else if et == ExprType.Call { return self.analyze_call((expr as CallExpr)) }
        else if et == ExprType.Index { return self.analyze_index((expr as IndexExpr)) }
        else if et == ExprType.Attribute { return self.analyze_attribute((expr as AttributeExpr)) }
        else if et == ExprType.ListLiteral { return self.analyze_list_literal((expr as ListLiteralExpr)) }
        else if et == ExprType.TupleLiteral { return self.analyze_tuple_literal((expr as TupleLiteralExpr)) }
        else if et == ExprType.SetLiteral { return self.analyze_set_literal((expr as SetLiteralExpr)) }
        else if et == ExprType.DictLiteral { return self.analyze_dict_literal((expr as DictLiteralExpr)) }
        else if et == ExprType.TypeConstructor { return self.analyze_type_constructor((expr as TypeConstructorExpr)) }
        else { return "any" }

    }
    fn analyze_identifier(&mut self, expr: IdentifierExpr)  ->  String {
        // Analyze identifier && return type// 
        let symbol = self.current_scope.resolve(expr.name);

        if ! symbol {
            format!(self.error("Undefined variable '{}'", expr.base.node.line, expr.base.node.column), expr.name);
            return "any";

        }
        return symbol.type;

    }
    fn analyze_binary_op(&mut self, expr: BinaryOpExpr)  ->  String {
        // Analyze binary operation// 
        let left_type = self.analyze_expression(expr.left);
        let right_type = self.analyze_expression(expr.right);

        // Type checking for operators
        if expr.operator == TokenType.Plus ||
        expr.operator == TokenType.Minus ||;
        expr.operator == TokenType.Star ||;
        expr.operator == TokenType.Slash {
            // Arithmetic operators
            if left_type == "int" && right_type == "int" {
                return "int";
            }
            else if left_type == "float" || right_type == "float" {
                return "float";
            }
            else {
                return "any";

            }
        };
        else if expr.operator == TokenType.EqualEqual ||
            expr.operator == TokenType.NotEqual ||;
            expr.operator == TokenType.Less ||;
            expr.operator == TokenType.Greater ||;
            expr.operator == TokenType.LessEq ||;
            expr.operator == TokenType.GreaterEq {
            // Comparison operators
        };
            return "bool";

        else if expr.operator == TokenType.And || expr.operator == TokenType.Or {
            // Logical operators
            return "bool";

        }
        else {
            return "any";

        }
    }
    fn analyze_unary_op(&mut self, expr: UnaryOpExpr)  ->  String {
        // Analyze unary operation// 
        let operand_type = self.analyze_expression(expr.operand);

        if expr.operator == TokenType.Minus {
            return operand_type; // Negation preserves type
        }
        else if expr.operator == TokenType.Not {
            return "bool";
        }
        else {
            return "any";

        }
    }
    fn analyze_call(&mut self, expr: CallExpr)  ->  String {
        // Analyze function call// 
        // Analyze callee
        let callee_type = self.analyze_expression(expr.callee);

        // Analyze arguments
        for arg in expr.arguments {
            self.analyze_expression(arg);

        // TODO: Check function signature matches
        }
        return "any"; // Unknown return type for now

    }
    fn analyze_index(&mut self, expr: IndexExpr)  ->  String {
        // Analyze index operation// 
        let object_type = self.analyze_expression(expr.object);
        let index_type = self.analyze_expression(expr.index);

        // TODO: Better type inference
        return "any";

    }
    fn analyze_attribute(&mut self, expr: AttributeExpr)  ->  String {
        // Analyze attribute access// 
        let object_type = self.analyze_expression(expr.object);

        // TODO: Check if attribute exists on type
        return "any";

    }
    fn analyze_type_constructor(&mut self, expr: TypeConstructorExpr)  ->  String {
        // Analyze type constructor// 
        self.analyze_expression(expr.argument);
        return expr.type_name;

    // ========================================
    // HELPER METHODS
    // ========================================

    }
    fn types_compatible(&self, expected: String, actual: String)  ->  bool {
        // Check if two types are compatible// 
        if expected == actual {
            return bool(true);
        }
        if expected == "any" || actual == "any" {
            return bool(true);
        // TODO: Add more sophisticated type checking
        }
        return bool(false);

// Public API
    }
}
fn analyze_semantics(program: Program)  ->  vec![String] {
    // Perform semantic analysis on program, return errors// 
    let mut sem_analyzer = create_analyzer();
    sem_analyzer.analyze_program(program);
    return sem_analyzer.errors;
}