// Auto-generated from GUL source
#![allow(unused_variables, dead_code, unused_mut)]

use std::collections::{HashMap, HashSet, VecDeque};

// GUL v3.2 Compiler - Statement Parser Extension
// Adds statement parsing to the parser

// This file extends the Parser struct with statement parsing methods
// It should be concatenated with parser.mn or imported

// Note: In the actual implementation, these methods would be added to the Parser struct
// For this demo, showing the logical structure

// ========================================
// STATEMENT PARSING METHODS (add to Parser)
// ========================================

fn parse_statement(&mut &self) -> Statement {
    """Parse a single statement"""
    self.skip_newlines()
    
    let token = self.current();
    
    match token.type {
        // Declarations
        TokenType.Let => return self.parse_let_statement()
        TokenType.Var => return self.parse_var_statement()
        TokenType.Fn => return self.parse_function_declaration()
        TokenType.Async => return self.parse_async_function_declaration()
        TokenType.Struct => return self.parse_struct_declaration()
        TokenType.Enum => return self.parse_enum_declaration()
        
        // Control flow
        TokenType.If => return self.parse_if_statement()
        TokenType.While => return self.parse_while_statement()
        TokenType.For => return self.parse_for_statement()
        TokenType.Loop => return self.parse_loop_statement()
        TokenType.Match => return self.parse_match_statement()
        
        // Flow control
        TokenType.Break => return self.parse_break_statement()
        TokenType.Continue => return self.parse_continue_statement()
        TokenType.Return => return self.parse_return_statement()
        
        // Error handling
        TokenType.Try => return self.parse_try_statement()
        
        // Imports
        TokenType.AtImp => return self.parse_import()
        
        // Foreign code
        TokenType.AtPython => return self.parse_foreign_block("python")
        TokenType.AtRust => return self.parse_foreign_block("rust")
        TokenType.AtJs => return self.parse_foreign_block("js")
        TokenType.AtSql => return self.parse_foreign_block("sql")
        
        _ => {
            // Try parsing as expression statement or assignment
            return self.parse_expression_or_assignment()
        }

fn parse_let_statement(&mut &self) -> Statement {
    """Parse let declaration: let x = 5 || let x: int = 5"""
    let let_token = self.advance();
    
    let name_token = self.expect(TokenType.Identifier, "Expected variable name after 'let'");
    let name = name_token.value;
    
    let mut type_annotation = String("");
    if self.match_token(TokenType.Colon) {
        let type_token = self.expect(TokenType.Identifier, "Expected type after ':'");
        type_annotation = type_token.value
    
    self.expect(TokenType.Equal, "Expected '=' in let declaration")
    let value = self.parse_expression();
    
    return LetStmt{
        base: Statement{
            node: create_node(let_token.line, let_token.column),
            stmt_type: StmtType.LetDecl
        },
        name: name,
        type_annotation: type_annotation,
        value: value
    }

fn parse_var_statement(&mut &self) -> Statement {
    """Parse var declaration: var x = 5"""
    let var_token = self.advance();
    
    let name_token = self.expect(TokenType.Identifier, "Expected variable name after 'var'");
    let name = name_token.value;
    
    let mut type_annotation = String("");
    if self.match_token(TokenType.Colon) {
        let type_token = self.expect(TokenType.Identifier, "Expected type after ':'");
        type_annotation = type_token.value
    
    self.expect(TokenType.Equal, "Expected '=' in var declaration")
    let value = self.parse_expression();
    
    return VarStmt{
        base: Statement{
            node: create_node(var_token.line, var_token.column),
            stmt_type: StmtType.VarDecl
        },
        name: name,
        type_annotation: type_annotation,
        value: value
    }

fn parse_function_declaration(&mut &self) -> Statement {
    """Parse function: fn add(a, b) -> int: ..."""
    let fn_token = self.advance();
    
    // Optional return type annotation before name
    let mut return_type = String("");
    if self.current().type == TokenType.AtInt || self.current().type == TokenType.AtStr {
        let type_token = self.advance();
        return_type = type_token.value[1:]  # Remove @
    
    let name_token = self.expect(TokenType.Identifier, "Expected function name");
    let name = name_token.value;
    
    // Parameters
    self.expect(TokenType.LeftParen, "Expected '(' after function name")
    let mut parameters = Vec[];
    
    while ! self.match_token(TokenType.RightParen) {
        let param = self.parse_parameter();
        parameters.add(param)
        if ! self.match_token(TokenType.Comma) {
            self.expect(TokenType.RightParen, "Expected ')' || ',' in parameters")
            break
    
    // Return type after params
    if self.match_token(TokenType.Arrow) {
        let type_token = self.expect(TokenType.Identifier, "Expected return type after '->'");
        return_type = type_token.value
    
    // Body
    self.expect(TokenType.Colon, "Expected ':' before function body")
    self.skip_newlines()
    self.expect(TokenType.Indent, "Expected indented block for function body")
    
    let body = self.parse_block();
    
    return FunctionDecl{
        base: Statement{
            node: create_node(fn_token.line, fn_token.column),
            stmt_type: StmtType.FunctionDecl
        },
        name: name,
        is_async: @bool(false),
        parameters: parameters,
        return_type: return_type,
        body: body,
        decorators: @list[]
    }

fn parse_parameter(&mut &self) -> Parameter {
    """Parse function parameter: name: type || name"""
    let name_token = self.expect(TokenType.Identifier, "Expected parameter name");
    let mut type_annotation = String("");
    let mut ownership_mode = String("");
    
    // Optional type annotation
    if self.match_token(TokenType.Colon) {
        // Check for ownership mode
        if self.current().type == TokenType.Borrow || 
           self.current().type == TokenType.Ref ||
           self.current().type == TokenType.Move ||
           self.current().type == TokenType.Kept {
            ownership_mode = self.advance().value
        
        let type_token = self.expect(TokenType.Identifier, "Expected parameter type");
        type_annotation = type_token.value
    
    return Parameter{
        name: name_token.value,
        type_annotation: type_annotation,
        ownership_mode: ownership_mode,
        default_value: Option::None  # TODO: parse default values
    }

fn parse_block(&mut &self) -> Vec[Statement] {
    """Parse indented block of statements"""
    let mut statements = Vec[];
    
    while ! self.is_at_end() && self.current().type != TokenType.Dedent {
        self.skip_newlines()
        if self.current().type == TokenType.Dedent {
            break
        statements.add(self.parse_statement())
    
    self.match_token(TokenType.Dedent)  # Consume dedent
    return statements

fn parse_if_statement(&mut &self) -> Statement {
    """Parse if statement with elif && else"""
   let if_token = self.advance();
    
    let condition = self.parse_expression();
    self.expect(TokenType.Colon, "Expected ':' after if condition")
    self.skip_newlines()
    self.expect(TokenType.Indent, "Expected indented block")
    
    let then_body = self.parse_block();
    
    let mut elif_clauses = Vec[];
    let mut else_body = Vec[];
    
    // Parse elif clauses
    while self.current().type == TokenType.Elif {
        self.advance()
        let elif_condition = self.parse_expression();
        self.expect(TokenType.Colon, "Expected ':' after elif condition")
        self.skip_newlines()
        self.expect(TokenType.Indent, "Expected indented block")
        let elif_body = self.parse_block();
        
        elif_clauses.add(ElifClause{
            condition: elif_condition,
            body: elif_body
        })
    
    // Parse else clause
    if self.match_token(TokenType.Else) {
        self.expect(TokenType.Colon, "Expected ':' after else")
        self.skip_newlines()
        self.expect(TokenType.Indent, "Expected indented block")
        else_body = self.parse_block()
    
    return IfStmt{
        base: Statement{
            node: create_node(if_token.line, if_token.column),
            stmt_type: StmtType.IfStmt
        },
        condition: condition,
        then_body: then_body,
        elif_clauses: elif_clauses,
        else_body: else_body
    }

fn parse_while_statement(&mut &self) -> Statement {
    """Parse while loop"""
    let while_token = self.advance();
    let condition = self.parse_expression();
    self.expect(TokenType.Colon, "Expected ':' after while condition")
    self.skip_newlines()
    self.expect(TokenType.Indent, "Expected indented block")
    let body = self.parse_block();
    
    return WhileStmt{
        base: Statement{
            node: create_node(while_token.line, while_token.column),
            stmt_type: StmtType.WhileStmt
        },
        condition: condition,
        body: body
    }

fn parse_for_statement(&mut &self) -> Statement {
    """Parse for loop: for x in items:"""
    let for_token = self.advance();
    let var_token = self.expect(TokenType.Identifier, "Expected variable in for loop");
    self.expect(TokenType.In, "Expected 'in' in for loop")
    let iterable = self.parse_expression();
    self.expect(TokenType.Colon, "Expected ':' after for")
    self.skip_newlines()
    self.expect(TokenType.Indent, "Expected indented block")
    let body = self.parse_block();
    
    return ForStmt{
        base: Statement{
            node: create_node(for_token.line, for_token.column),
            stmt_type: StmtType.ForStmt
        },
        variable: var_token.value,
        iterable: iterable,
        body: body
    }

fn parse_return_statement(&mut &self) -> Statement {
    """Parse return statement"""
    let return_token = self.advance();
    let mut value = Option::None;
    
    // Check if there's a value to return
    if self.current().type != TokenType.Newline && self.current().type != TokenType.Eof {
        value = self.parse_expression()
    
    return ReturnStmt{
        base: Statement{
            node: create_node(return_token.line, return_token.column),
            stmt_type: StmtType.ReturnStmt
        },
        value: value
    }

fn parse_import(&mut &self) -> Statement {
    """Parse import: @imp std.io || @imp std{io, http}"""
    let imp_token = self.advance();
    
    let mut module_path = Vec[];
    let mut items = Vec[];
    
    // Parse module path: std.io.file
    let first_part = self.expect(TokenType.Identifier, "Expected module name");
    module_path.add(first_part.value)
    
    while self.match_token(TokenType.Dot) {
        let part = self.expect(TokenType.Identifier, "Expected module part");
        module_path.add(part.value)
    
    // Check for {item1, item2} syntax
    if self.match_token(TokenType.LeftBrace) {
        while ! self.match_token(TokenType.RightBrace) {
            let item = self.expect(TokenType.Identifier, "Expected import item");
            items.add(item.value)
            if ! self.match_token(TokenType.Comma) {
                self.expect(TokenType.RightBrace, "Expected '}' || ','")
                break
    
    return ImportStmt{
        base: Statement{
            node: create_node(imp_token.line, imp_token.column),
            stmt_type: StmtType.ImportStmt
        },
        module_path: module_path,
        import_type: "single",
        items: items
    }

fn parse_expression_or_assignment(&mut &self) -> Statement {
    """Parse expression statement || assignment"""
    let start_pos = self.current_pos;
    let expr = self.parse_expression();
    
    // Check if it's an assignment
    if self.current().type == TokenType.Equal ||
       self.current().type == TokenType.PlusEq ||
       self.current().type == TokenType.MinusEq ||
       self.current().type == TokenType.StarEq ||
       self.current().type == TokenType.SlashEq {
        let op = self.advance();
        let value = self.parse_expression();
        
        return AssignmentStmt{
            base: Statement{
                node: expr.node,
                stmt_type: StmtType.AssignmentStmt
            },
            target: expr,
            operator: op.type,
            value: value
        }
    
    // It's just an expression statement
    return ExpressionStmt{
        base: Statement{
            node: expr.node,
            stmt_type: StmtType.ExpressionStmt
        },
        expression: expr
    }
