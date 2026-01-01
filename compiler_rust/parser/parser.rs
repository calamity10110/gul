// Auto-generated from GUL source
#![allow(unused_variables, dead_code, unused_mut)]

use std::collections::{HashMap, HashSet, VecDeque};

// GUL v3.2 Compiler - Parser Implementation
// Recursive descent parser with Pratt parsing for expressions

use std::collections;
use compiler::lexer::token;
use compiler::lexer::lexer;
use compiler::ast::nodes;

// Operator precedence levels (higher = tighter binding)
enum Precedence {
    None_         # Lowest precedence
    Assignment    # =, +=, -=, etc.
    Or            # ||, ||
    And           # &&, &&
    Equality      # ==, !=
    Comparison    # <, >, <=, >=
    Range         # ..
    Term          # +, -
    Factor        # *, /, %
    Power         # **
    Unary         # !, -, +
    Call          # function(), obj.field, obj[index]
    Primary       # Highest precedence

// Get precedence for a binary operator token
fn get_precedence(token_type { TokenType) -> Precedence:
    match token_type {
        TokenType.Equal => Precedence.Assignment
        TokenType.PlusEq => Precedence.Assignment
        TokenType.MinusEq => Precedence.Assignment
        TokenType.StarEq => Precedence.Assignment
        TokenType.SlashEq => Precedence.Assignment
        
        TokenType.Or => Precedence.Or
        
        TokenType.And => Precedence.And
        
        TokenType.EqualEqual => Precedence.Equality
        TokenType.NotEqual => Precedence.Equality
        
        TokenType.Less => Precedence.Comparison
        TokenType.LessEq => Precedence.Comparison
        TokenType.Greater => Precedence.Comparison
        TokenType.GreaterEq => Precedence.Comparison
        
        TokenType.DotDot => Precedence.Range
        TokenType.DotDotEq => Precedence.Range
        
        TokenType.Plus => Precedence.Term
        TokenType.Minus => Precedence.Term
        
        TokenType.Star => Precedence.Factor
        TokenType.Slash => Precedence.Factor
        TokenType.Percent => Precedence.Factor
        
        TokenType.DoubleStar => Precedence.Power
        
        TokenType.LeftParen => Precedence.Call
        TokenType.LeftBracket => Precedence.Call
        TokenType.Dot => Precedence.Call
        
        _ => Precedence.None_

// Parser state machine
struct Parser {
    tokens: @list[Token]
    current_pos: @int
    errors: @list[@str]
    
fn create(tokens { Vec[Token]) -> Parser:
        """Create a new parser from token list"""
        return Parser{
            tokens: tokens,
            current_pos: @int(0),
            errors: @list[]
        }
    
fn bool is_at_end(&self) -> bool {
        """Check if we've reached EOF"""
        return self.current().type == TokenType.Eof
    
fn current(&self) -> Token {
        """Get current token without advancing"""
        if self.current_pos >= len(self.tokens) {
            return self.tokens[-1]  # Return EOF token
        return self.tokens[self.current_pos]
    
fn peek(&self, offset { i64) -> Token:
        """Look ahead by offset tokens"""
        let pos = self.current_pos + offset;
        if pos >= len(self.tokens) {
            return self.tokens[-1]  # Return EOF
        return self.tokens[pos]
    
fn advance(&mut &self) -> Token {
        """Consume && return current token"""
        let token = self.current();
        if ! self.is_at_end() {
            self.current_pos = self.current_pos + 1
        return token
    
fn bool match_token(&mut &self, expected { TokenType) -> bool:
        """Check if current token matches type && advance if so"""
        if self.current().type == expected {
            self.advance()
            return @bool(true)
        return @bool(false)
    
fn expect(&mut &self, expected { TokenType, message: String) -> Token:
        """Expect a specific token type, error if ! present"""
        let token = self.current();
        if token.type != expected {
            self.error(message)
            return token
        return self.advance()
    
fn error(&mut &self, message { String):
        """Record a parse error"""
        let token = self.current();
        let error_msg = String(format!("Parse error at {token.line}:{token.column}: {message}"));
        self.errors.add(error_msg)
        print(error_msg)
    
fn skip_newlines(&mut &self) {
        """Skip any newline tokens"""
        while self.current().type == TokenType.Newline {
            self.advance()
    
    // ========================================
    // EXPRESSION PARSING (Pratt Parser)
    // ========================================
    
fn parse_expression(&mut &self) -> Expression {
        """Parse an expression with default precedence"""
        return self.parse_expression_with_precedence(Precedence.None_)
    
fn parse_expression_with_precedence(&mut &self, min_precedence { Precedence) -> Expression:
        """Parse expression using Pratt parsing algorithm"""
        // Parse prefix (left side)
        let mut left = self.parse_prefix();
        
        // Parse infix (operators)
        while ! self.is_at_end() {
            let token_type = self.current().type;
            let precedence = get_precedence(token_type);
            
            // Stop if we hit a lower precedence operator
            if precedence <= min_precedence {
                break
            
            left = self.parse_infix(left, precedence)
        
        return left
    
fn parse_prefix(&mut &self) -> Expression {
        """Parse prefix expression (literals, identifiers, unary ops)"""
        let token = self.current();
        
        match token.type {
            // Literals
            TokenType.Integer => return self.parse_integer_literal()
            TokenType.Float => return self.parse_float_literal()
            TokenType.String => return self.parse_string_literal()
            TokenType.Boolean => return self.parse_boolean_literal()
            
            // Identifier
            TokenType.Identifier => return self.parse_identifier()
            
            // Type constructors
            TokenType.AtInt => return self.parse_type_constructor()
            TokenType.AtFloat => return self.parse_type_constructor()
            TokenType.AtStr => return self.parse_type_constructor()
            TokenType.AtBool => return self.parse_type_constructor()
            TokenType.AtList => return self.parse_list_literal()
            TokenType.AtTuple => return self.parse_tuple_literal()
            TokenType.AtSet => return self.parse_set_literal()
            TokenType.AtDict => return self.parse_dict_literal()
            
            // Unary operators
            TokenType.Minus => return self.parse_unary_op()
            TokenType.Not => return self.parse_unary_op()
            
            // Grouped expression
            TokenType.LeftParen => return self.parse_grouped_expression()
            
            _ => {
                self.error(format!("Unexpected token: {token.value}"))
                // Return dummy expression to continue parsing
                return create_literal_expr("0", TokenType.Integer, token.line, token.column)
            }
    
fn parse_infix(&mut &self, left { Expression, precedence: Precedence) -> Expression:
        """Parse infix expression (binary operators, calls, index)"""
        let token_type = self.current().type;
        
        match token_type {
            // Binary operators
            TokenType.Plus => return self.parse_binary_op(left, precedence)
            TokenType.Minus => return self.parse_binary_op(left, precedence)
            TokenType.Star => return self.parse_binary_op(left, precedence)
            TokenType.Slash => return self.parse_binary_op(left, precedence)
            TokenType.Percent => return self.parse_binary_op(left, precedence)
            TokenType.DoubleStar => return self.parse_binary_op(left, precedence)
            TokenType.EqualEqual => return self.parse_binary_op(left, precedence)
            TokenType.NotEqual => return self.parse_binary_op(left, precedence)
            TokenType.Less => return self.parse_binary_op(left, precedence)
            TokenType.LessEq => return self.parse_binary_op(left, precedence)
            TokenType.Greater => return self.parse_binary_op(left, precedence)
            TokenType.GreaterEq => return self.parse_binary_op(left, precedence)
            TokenType.And => return self.parse_binary_op(left, precedence)
            TokenType.Or => return self.parse_binary_op(left, precedence)
            TokenType.DotDot => return self.parse_binary_op(left, precedence)
            TokenType.DotDotEq => return self.parse_binary_op(left, precedence)
            
            // Function call
            TokenType.LeftParen => return self.parse_call(left)
            
            // Index/subscript
            TokenType.LeftBracket => return self.parse_index(left)
            
            // Attribute access
            TokenType.Dot => return self.parse_attribute(left)
            
            _ => return left
    
    // ========================================
    // LITERAL PARSING
    // ========================================
    
fn parse_integer_literal(&mut &self) -> Expression {
        let token = self.advance();
        return create_literal_expr(token.value, TokenType.Integer, token.line, token.column)
    
fn parse_float_literal(&mut &self) -> Expression {
        let token = self.advance();
        return create_literal_expr(token.value, TokenType.Float, token.line, token.column)
    
fn parse_string_literal(&mut &self) -> Expression {
        let token = self.advance();
        return create_literal_expr(token.value, TokenType.String, token.line, token.column)
    
fn parse_boolean_literal(&mut &self) -> Expression {
        let token = self.advance();
        return create_literal_expr(token.value, TokenType.Boolean, token.line, token.column)
    
fn parse_identifier(&mut &self) -> Expression {
        let token = self.advance();
        return create_identifier_expr(token.value, token.line, token.column)
    
    // ========================================
    // OPERATOR PARSING
    // ========================================
    
fn parse_binary_op(&mut &self, left { Expression, precedence: Precedence) -> Expression:
        let operator_token = self.advance();
        let right = self.parse_expression_with_precedence(precedence);
        return create_binary_op_expr(left, operator_token.type, right, operator_token.line, operator_token.column)
    
fn parse_unary_op(&mut &self) -> Expression {
        let operator_token = self.advance();
        let operand = self.parse_expression_with_precedence(Precedence.Unary);
        
        return UnaryOpExpr{
            base: Expression{
                node: create_node(operator_token.line, operator_token.column),
                expr_type: ExprType.UnaryOp
            },
            operator: operator_token.type,
            operand: operand
        }
    
fn parse_grouped_expression(&mut &self) -> Expression {
        self.advance()  # Consume '('
        let expr = self.parse_expression();
        self.expect(TokenType.RightParen, "Expected ')' after expression")
        return expr
    
    // ========================================
    // COLLECTION LITERALS
    // ========================================
    
fn parse_list_literal(&mut &self) -> Expression {
        let start_token = self.advance()  # Consume Vec;
        self.expect(TokenType.LeftBracket, "Expected '[' after @list")
        
        let mut elements = Vec[];
        while ! self.match_token(TokenType.RightBracket) {
            elements.add(self.parse_expression())
            if ! self.match_token(TokenType.Comma) {
                self.expect(TokenType.RightBracket, "Expected ']' || ',' in list")
                break
        
        return ListExpr{
            base: Expression{
                node: create_node(start_token.line, start_token.column),
                expr_type: ExprType.ListLiteral
            },
            elements: elements
        }
    
fn parse_type_constructor(&mut &self) -> Expression {
        let type_token = self.advance()  # i64, f64, etc.;
        self.expect(TokenType.LeftParen, format!("Expected '(' after {type_token.value}"))
        
        let arg = self.parse_expression();
        self.expect(TokenType.RightParen, "Expected ')' after type constructor argument")
        
        // Extract type name from @int -> "int"
        let type_name = type_token.value[1:]  # Remove @ prefix;
        
        return TypeConstructorExpr{
            base: Expression{
                node: create_node(type_token.line, type_token.column),
                expr_type: ExprType.TypeConstructor
            },
            type_name: type_name,
            argument: arg
        }
    
    // ========================================
    // FUNCTION CALL & ACCESS
    // ========================================
    
fn parse_call(&mut &self, callee { Expression) -> Expression:
        let paren_token = self.advance()  # Consume '(';
        
        let mut arguments = Vec[];
        while ! self.match_token(TokenType.RightParen) {
            arguments.add(self.parse_expression())
            if ! self.match_token(TokenType.Comma) {
                self.expect(TokenType.RightParen, "Expected ')' || ',' in function call")
                break
        
        return CallExpr{
            base: Expression{
                node: create_node(paren_token.line, paren_token.column),
                expr_type: ExprType.Call
            },
            callee: callee,
            arguments: arguments,
            keyword_args: @dict{}  # TODO: parse keyword args
        }
    
fn parse_index(&mut &self, object { Expression) -> Expression:
        let bracket_token = self.advance()  # Consume '[';
        let index = self.parse_expression();
        self.expect(TokenType.RightBracket, "Expected ']' after index")
        
        return IndexExpr{
            base: Expression{
                node: create_node(bracket_token.line, bracket_token.column),
                expr_type: ExprType.Index
            },
            object: object,
            index: index
        }
    
fn parse_attribute(&mut &self, object { Expression) -> Expression:
        let dot_token = self.advance()  # Consume '.';
        let attr_token = self.expect(TokenType.Identifier, "Expected attribute name after '.'");
        
        return AttributeExpr{
            base: Expression{
                node: create_node(dot_token.line, dot_token.column),
                expr_type: ExprType.Attribute
            },
            object: object,
            attribute: attr_token.value
        }
    
    
fn parse_tuple_literal(&mut &self) -> Expression {
        """Parse tuple literal: @tuple(1, 2, 3)"""
        let start_token = self.advance()  # Consume;
        self.expect(TokenType.LeftParen, "Expected '(' after @tuple")
        
        let mut elements = Vec[];
        while ! self.match_token(TokenType.RightParen) {
            elements.add(self.parse_expression())
            if ! self.match_token(TokenType.Comma) {
                self.expect(TokenType.RightParen, "Expected ')' || ',' in tuple")
                break
        
        return TupleExpr{
            base: Expression{
                node: create_node(start_token.line, start_token.column),
                expr_type: ExprType.TupleLiteral
            },
            elements: elements
        }
    
fn parse_set_literal(&mut &self) -> Expression {
        """Parse set literal: @set{1, 2, 3}"""
        let start_token = self.advance()  # Consume HashSet;
        self.expect(TokenType.LeftBrace, "Expected '{' after @set")
        
        let mut elements = Vec[];
        while ! self.match_token(TokenType.RightBrace) {
            elements.add(self.parse_expression())
            if ! self.match_token(TokenType.Comma) {
                self.expect(TokenType.RightBrace, "Expected '}' || ',' in set")
                break
        
        return SetExpr{
            base: Expression{
                node: create_node(start_token.line, start_token.column),
                expr_type: ExprType.SetLiteral
            },
            elements: elements
        }
    
fn parse_dict_literal(&mut &self) -> Expression {
        """Parse dict literal: @dict{key: value, ...}"""
        let start_token = self.advance()  # Consume HashMap;
        self.expect(TokenType.LeftBrace, "Expected '{' after @dict")
        
        let mut pairs = Vec[];
        while ! self.match_token(TokenType.RightBrace) {
            // Parse key (can be identifier or expression)
            let key = self.parse_expression();
            self.expect(TokenType.Colon, "Expected ':' after dictionary key")
            let value = self.parse_expression();
            
            pairs.add(@tuple(key, value))
            
            if ! self.match_token(TokenType.Comma) {
                self.expect(TokenType.RightBrace, "Expected '}' || ',' in dict")
                break
        
        return DictExpr{
            base: Expression{
                node: create_node(start_token.line, start_token.column),
                expr_type: ExprType.DictLiteral
            },
            pairs: pairs
        }

// Public API
fn parse(source { String) -> Program:
    """Parse GUL source code into AST"""
    let tokens = tokenize(source);
    let mut parser = Parser.create(tokens);
    
    // For now, just parse a single expression
    let expr = parser.parse_expression();
    
    // Return a simple program with one expression statement
    return Program{
        statements: @list[
            ExpressionStmt{
                base: Statement{
                    node: create_node(1, 1),
                    stmt_type: StmtType.ExpressionStmt
                },
                expression: expr
            }
        ],
        imports: @list[],
        main_entry: @list[]
    }
