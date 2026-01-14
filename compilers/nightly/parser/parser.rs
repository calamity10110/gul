// GUL v3.2 Compiler - Recursive Descent Parser
use crate::lexer::token::*;
use crate::ast::nodes::*;
use std::collections::HashMap;

// Enumeration for operator precedence
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    NoPrec,
    Assignment,
    Pipeline, // |>
    Or,
    And,
    Comparison,
    Term,
    Factor,
    Power,
    Range,
    Prefix,
    Call,
    // Pipeline, // Moved up
    Index,
    Unpack,

}
pub fn get_precedence(token_type: TokenType)  ->  Precedence {
    // Get the precedence level for a given token type// 
    if token_type == TokenType::Equal {
        return Precedence::NoPrec;
    }
    else if token_type == TokenType::Pipeline {
        return Precedence::Pipeline;
    }
    else if token_type == TokenType::Or || token_type == TokenType::Not {
        return Precedence::Or;
    }
    else if token_type == TokenType::And {
        return Precedence::And;
    }
    else if token_type == TokenType::EqualEqual || token_type == TokenType::NotEqual {
        return Precedence::Comparison;
    }
    else if token_type == TokenType::Less || token_type == TokenType::LessEq || token_type == TokenType::Greater || token_type == TokenType::GreaterEq {
        return Precedence::Comparison;
    }
    else if token_type == TokenType::Plus || token_type == TokenType::Minus {
        return Precedence::Term;
    }
    else if token_type == TokenType::Star || token_type == TokenType::Slash || token_type == TokenType::Percent {
        return Precedence::Factor;
    }
    else if token_type == TokenType::DoubleStar {
        return Precedence::Power;
    }
    else if token_type == TokenType::LeftParen || token_type == TokenType::LeftBracket || token_type == TokenType::Dot {
        return Precedence::Call;
    }
    else if token_type == TokenType::DotDot {
        return Precedence::Range;
    }
    return Precedence::NoPrec;

}
#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current_pos: usize,
    pub errors: Vec<String>,

}
impl Parser {
    pub fn create(tokens: Vec<Token>)  ->  Parser {
        return Parser{tokens: tokens, current_pos: 0usize, errors: vec![]};

    }
    pub fn is_at_end(&mut self)  ->  bool {
        let size = (self.tokens).len();
        return self.current_pos >= size - 1;

    }
    pub fn current(&mut self)  ->  Token {
        return self.tokens[self.current_pos].clone();

    }
    pub fn advance(&mut self)  ->  Token {
        let token = self.tokens[self.current_pos].clone();
        let size = (self.tokens).len();
        if self.current_pos < size - 1 {
            self.current_pos = self.current_pos + 1;
        }
        return token;

    }
    pub fn match_token(&mut self, expected_type: TokenType)  ->  bool {
        let tok = self.tokens[self.current_pos].clone();
        if tok.token_type == expected_type {
            let size = (self.tokens).len();
            if self.current_pos < size - 1 {
                self.current_pos = self.current_pos + 1;
            }
            return true;
        }
        return false;

    }
    pub fn expect(&mut self, expected_type: TokenType, message: String)  ->  Token {
        let tok = self.tokens[self.current_pos].clone();
        if tok.token_type == expected_type {
            let size = (self.tokens).len();
            if self.current_pos < size - 1 {
                self.current_pos = self.current_pos + 1;
            }
            return tok;
        }
        println!("{}", "Parse error: ".to_string() + &message);
        return tok; // fallthrough

    }
    pub fn parse_expression(&mut self, min_precedence: Precedence)  ->  Expression {
        let mut left = self.parse_prefix();
        loop {
            if self.current_pos >= (self.tokens).len() - 1 { break }
            let tok = self.tokens[self.current_pos].clone();
            let prec = get_precedence(tok.token_type.clone());
            if prec <= min_precedence { break }
            left = self.parse_infix(left, prec);
        }
        return left;

    }
    pub fn parse_prefix(&mut self)  ->  Expression {
        let token = self.advance();
        let t_type = token.token_type.clone();
        if t_type == TokenType::Identifier {
            return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: token.value});
        }
        else if t_type == TokenType::Integer {
            return Expression::Literal(LiteralExpr{node: ASTNode{line: 1, column: 1}, value: token.value, value_type: TokenType::Integer});
        }
        else if t_type == TokenType::Float {
            return Expression::Literal(LiteralExpr{node: ASTNode{line: 1, column: 1}, value: token.value, value_type: TokenType::Float});
        }
        else if t_type == TokenType::String {
            return Expression::Literal(LiteralExpr{node: ASTNode{line: 1, column: 1}, value: token.value, value_type: TokenType::String});
        }
        else if t_type == TokenType::TrueKeyword {
            return Expression::Literal(LiteralExpr{node: ASTNode{line: 1, column: 1}, value: "true".to_string(), value_type: TokenType::TrueKeyword});
        }
        else if t_type == TokenType::FalseKeyword {
             return Expression::Literal(LiteralExpr{node: ASTNode{line: 1, column: 1}, value: "false".to_string(), value_type: TokenType::FalseKeyword});
        }
        else if t_type == TokenType::LeftBracket {
             // List: [1, 2, 3]
             let mut elements = vec![];
             while self.current().token_type != TokenType::RightBracket && self.current().token_type != TokenType::Eof {
                  self.skip_newlines();
                  if self.current().token_type == TokenType::RightBracket { break; }
                  elements.push(self.parse_expression(Precedence::NoPrec));
                  if !self.match_token(TokenType::Comma) {
                      self.skip_newlines(); 
                      break; 
                  }
                  self.skip_newlines(); 
             }
             self.expect(TokenType::RightBracket, "Expected ']'".to_string());
             return Expression::List(ListExpr{node: ASTNode{line: 1, column: 1}, elements: elements});
        }
        else if t_type == TokenType::LeftBrace {
             // Dict: {k:v} or Set: {v}
             self.skip_newlines();
             if self.current().token_type == TokenType::RightBrace {
                 self.advance(); // consume }
                 return Expression::Dict(DictExpr{node: ASTNode{line: 1, column: 1}, pairs: vec![]});
             }
             let first = self.parse_expression(Precedence::NoPrec);
             if self.match_token(TokenType::Colon) {
                 let val = self.parse_expression(Precedence::NoPrec);
                 let mut pairs = vec![(first, val)];
                 while self.match_token(TokenType::Comma) {
                     self.skip_newlines();
                     if self.current().token_type == TokenType::RightBrace { break; }
                     let k = self.parse_expression(Precedence::NoPrec);
                     self.expect(TokenType::Colon, "Expected ':'".to_string());
                     let v = self.parse_expression(Precedence::NoPrec);
                     pairs.push((k, v));
                 }
                 self.expect(TokenType::RightBrace, "Expected '}'".to_string());
                 return Expression::Dict(DictExpr{node: ASTNode{line: 1, column: 1}, pairs: pairs});
             } else {
                 let mut elements = vec![first];
                 while self.match_token(TokenType::Comma) {
                     self.skip_newlines();
                     if self.current().token_type == TokenType::RightBrace { break; }
                     elements.push(self.parse_expression(Precedence::NoPrec));
                 }
                 self.expect(TokenType::RightBrace, "Expected '}'".to_string());
                 return Expression::Set(SetExpr{node: ASTNode{line: 1, column: 1}, elements: elements});
             }
        }
        else if t_type == TokenType::LeftParen {
            let expr = self.parse_expression(Precedence::NoPrec);
            self.expect(TokenType::RightParen, "Expected ')'".to_string());
            return expr;
        }
        else if t_type == TokenType::Minus {
            let op = self.parse_expression(Precedence::Prefix);
            return Expression::UnaryOp(UnaryOpExpr{node: ASTNode{line: 1, column: 1}, operator: TokenType::Minus, operand: Box::new(op)});
        }
        else if t_type == TokenType::Newline || t_type == TokenType::Indent || t_type == TokenType::Dedent {
            // Skip whitespace tokens and try again
            return self.parse_prefix();
        }
        else if t_type == TokenType::Await {
             // Await expression: await future
             let await_token = self.advance(); // consume await
             let right = self.parse_expression(Precedence::Prefix);
             return Expression::Await(AwaitExpr{
                 node: create_node(await_token.line, await_token.column), 
                 value: Box::new(right)
             });
        }
        else if t_type == TokenType::AtTabl {
             return self.parse_tabl_literal();
        }
        else if t_type == TokenType::AtFrame {
             return self.parse_dataframe_literal();
        }
        // Handle type constructor tokens as identifiers for backend processing
        else if t_type == TokenType::AtInt {
             return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: "@int".to_string()});
        }
        else if t_type == TokenType::AtFloat {
             return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: "@float".to_string()});
        }
        else if t_type == TokenType::AtStr {
             return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: "@str".to_string()});
        }
        else if t_type == TokenType::AtBool {
             return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: "@bool".to_string()});
        }
        else if t_type == TokenType::AtList {
             // @list[1, 2, 3] syntax
             if self.current().token_type == TokenType::LeftBracket {
                 self.advance(); // consume '['
                 let mut elements = vec![];
                 while self.current().token_type != TokenType::RightBracket && self.current().token_type != TokenType::Eof {
                     elements.push(self.parse_expression(Precedence::NoPrec));
                     if !self.match_token(TokenType::Comma) {
                         break;
                     }
                 }
                 self.expect(TokenType::RightBracket, "Expected ']' after list elements".to_string());
                 return Expression::List(ListExpr{node: ASTNode{line: 1, column: 1}, elements: elements});
             }
             return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: "@list".to_string()});
        }
        else if t_type == TokenType::AtDict {
             // @dict{key: value, ...} syntax
             if self.current().token_type == TokenType::LeftBrace {
                 self.advance(); // consume '{'
                 self.skip_newlines();
                 let mut pairs = vec![];
                 while self.current().token_type != TokenType::RightBrace && self.current().token_type != TokenType::Eof {
                     // Parse key
                     let key = if self.current().token_type == TokenType::Identifier {
                         let key_token = self.advance();
                         Expression::Literal(LiteralExpr{node: ASTNode{line: 1, column: 1}, value: key_token.value, value_type: TokenType::String})
                     } else {
                         self.parse_expression(Precedence::NoPrec)
                     };
                     self.expect(TokenType::Colon, "Expected ':' after dict key".to_string());
                     let value = self.parse_expression(Precedence::NoPrec);
                     pairs.push((key, value));
                     if !self.match_token(TokenType::Comma) {
                         self.skip_newlines();
                         if self.current().token_type != TokenType::RightBrace {
                             continue;
                         }
                         break;
                     }
                     self.skip_newlines();
                 }
                 self.expect(TokenType::RightBrace, "Expected '}' after dict entries".to_string());
                 return Expression::Dict(DictExpr{node: ASTNode{line: 1, column: 1}, pairs: pairs});
             }
             return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: "@dict".to_string()});
        }
        else if t_type == TokenType::AtSet {
             // @set{1, 2, 3} syntax
             if self.current().token_type == TokenType::LeftBrace {
                 self.advance(); // consume '{'
                 let mut elements = vec![];
                 while self.current().token_type != TokenType::RightBrace && self.current().token_type != TokenType::Eof {
                     elements.push(self.parse_expression(Precedence::NoPrec));
                     if !self.match_token(TokenType::Comma) {
                         break;
                     }
                 }
                 self.expect(TokenType::RightBrace, "Expected '}' after set elements".to_string());
                 return Expression::Set(SetExpr{node: ASTNode{line: 1, column: 1}, elements: elements});
             }
             return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: "@set".to_string()});
        }
        else if t_type == TokenType::LeftBracket {
             // List: [1, 2, 3]
             let mut elements = vec![];
             while self.current().token_type != TokenType::RightBracket && self.current().token_type != TokenType::Eof {
                  self.skip_newlines();
                  if self.current().token_type == TokenType::RightBracket { break; }
                  elements.push(self.parse_expression(Precedence::NoPrec));
                  if !self.match_token(TokenType::Comma) {
                      self.skip_newlines(); 
                      break; 
                  }
                  self.skip_newlines(); 
             }
             self.expect(TokenType::RightBracket, "Expected ']'".to_string());
             return Expression::List(ListExpr{node: ASTNode{line: 1, column: 1}, elements: elements});
        }
        else if t_type == TokenType::LeftBrace {
             // Dict: {k:v} or Set: {v}
             self.skip_newlines();
             if self.current().token_type == TokenType::RightBrace {
                 self.advance();
                 return Expression::Dict(DictExpr{node: ASTNode{line: 1, column: 1}, pairs: vec![]});
             }
             let first = self.parse_expression(Precedence::NoPrec);
             if self.match_token(TokenType::Colon) {
                 let val = self.parse_expression(Precedence::NoPrec);
                 let mut pairs = vec![(first, val)];
                 while self.match_token(TokenType::Comma) {
                     self.skip_newlines();
                     if self.current().token_type == TokenType::RightBrace { break; }
                     let k = self.parse_expression(Precedence::NoPrec);
                     self.expect(TokenType::Colon, "Expected ':'".to_string());
                     let v = self.parse_expression(Precedence::NoPrec);
                     pairs.push((k, v));
                 }
                 self.expect(TokenType::RightBrace, "Expected '}'".to_string());
                 return Expression::Dict(DictExpr{node: ASTNode{line: 1, column: 1}, pairs: pairs});
             } else {
                 let mut elements = vec![first];
                 while self.match_token(TokenType::Comma) {
                     self.skip_newlines();
                     if self.current().token_type == TokenType::RightBrace { break; }
                     elements.push(self.parse_expression(Precedence::NoPrec));
                 }
                 self.expect(TokenType::RightBrace, "Expected '}'".to_string());
                 return Expression::Set(SetExpr{node: ASTNode{line: 1, column: 1}, elements: elements});
             }
        }
        else if t_type == TokenType::AtTensor {
             return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: "@tensor".to_string()});
        }
        else if t_type == TokenType::AtTuple {
             return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: "@tuple".to_string()});
        }
        return Expression::Literal(LiteralExpr{node: ASTNode{line: 1, column: 1}, value: "None".to_string(), value_type: TokenType::NoneLiteral});

    }
    pub fn parse_infix(&mut self, left: Expression, precedence: Precedence)  ->  Expression {
        let token = self.advance();
        if token.token_type == TokenType::Plus || token.token_type == TokenType::Minus || token.token_type == TokenType::Star || token.token_type == TokenType::Slash || token.token_type == TokenType::DotDot {
            let right = self.parse_expression(precedence);
            return Expression::BinaryOp(BinaryOpExpr{node: ASTNode{line: 1, column: 1}, left: Box::new(left), operator: token.token_type, right: Box::new(right)});
        }
        else if token.token_type == TokenType::Pipeline {
            // Pipeline Operator: left |> right
            let right = self.parse_expression(precedence);
            
            match right {
                 Expression::Call(mut call_expr) => {
                     call_expr.arguments.insert(0, left);
                     return Expression::Call(call_expr);
                 },
                 Expression::Identifier(ident_expr) => {
                     return Expression::Call(CallExpr{
                         node: ident_expr.node.clone(),
                         callee: Box::new(Expression::Identifier(ident_expr)),
                         arguments: vec![left],
                         keyword_args: HashMap::new()
                     });
                 },
                 _ => {
                      return Expression::Call(CallExpr{
                         node: ASTNode{line: 1, column: 1}, 
                         callee: Box::new(right),
                         arguments: vec![left],
                         keyword_args: HashMap::new()
                     });
                 }
            }
        }
        else if token.token_type == TokenType::LeftParen {
            let mut args = vec![];
            if ! (self.tokens[self.current_pos].token_type == TokenType::RightParen) {
                args.push(self.parse_expression(Precedence::NoPrec));
                while self.match_token(TokenType::Comma) {
                    args.push(self.parse_expression(Precedence::NoPrec));
                }
            }
            self.expect(TokenType::RightParen, "Expected ')'".to_string());
            return Expression::Call(CallExpr{node: ASTNode{line: 1, column: 1}, callee: Box::new(left), arguments: args, keyword_args: HashMap::new()});
        }
        else if token.token_type == TokenType::Dot {
            // Attribute access or method call: obj.attr or obj.method(args)
            let attr_token = self.expect(TokenType::Identifier, "Expected attribute name after '.'".to_string());
            let attr_name = attr_token.value;
            
            // Check if this is a method call (followed by parentheses)
            if self.current().token_type == TokenType::LeftParen {
                self.advance(); // consume '('
                let mut args = vec![];
                if self.current().token_type != TokenType::RightParen {
                    args.push(self.parse_expression(Precedence::NoPrec));
                    while self.match_token(TokenType::Comma) {
                        args.push(self.parse_expression(Precedence::NoPrec));
                    }
                }
                self.expect(TokenType::RightParen, "Expected ')' after method arguments".to_string());
                
                // Create a method call expression
                // Transform obj.method(args) into a Call with Attribute as callee
                let attr_expr = Expression::Attribute(AttributeExpr{
                    node: ASTNode{line: 1, column: 1},
                    object: Box::new(left),
                    attribute: attr_name
                });
                return Expression::Call(CallExpr{
                    node: ASTNode{line: 1, column: 1},
                    callee: Box::new(attr_expr),
                    arguments: args,
                    keyword_args: HashMap::new()
                });
            } else {
                // Just attribute access
                return Expression::Attribute(AttributeExpr{
                    node: ASTNode{line: 1, column: 1},
                    object: Box::new(left),
                    attribute: attr_name
                });
            }
        }
        else if token.token_type == TokenType::LeftBracket {
            // Index access: obj[index]
            let index = self.parse_expression(Precedence::NoPrec);
            self.expect(TokenType::RightBracket, "Expected ']' after index".to_string());
            return Expression::Index(IndexExpr{
                node: ASTNode{line: 1, column: 1},
                object: Box::new(left),
                index: Box::new(index)
            });
        }
        else if token.token_type == TokenType::EqualEqual || token.token_type == TokenType::NotEqual || 
                token.token_type == TokenType::Less || token.token_type == TokenType::LessEq || 
                token.token_type == TokenType::Greater || token.token_type == TokenType::GreaterEq {
            let right = self.parse_expression(precedence);
            return Expression::BinaryOp(BinaryOpExpr{
                node: ASTNode{line: 1, column: 1}, 
                left: Box::new(left), 
                operator: token.token_type, 
                right: Box::new(right)
            });
        }
        else if token.token_type == TokenType::Or || token.token_type == TokenType::And {
             let right = self.parse_expression(precedence);
             return Expression::BinaryOp(BinaryOpExpr{
                node: ASTNode{line: 1, column: 1},
                left: Box::new(left),
                operator: token.token_type,
                right: Box::new(right)
             });
        }
        return left;

    }

    pub fn parse_program(&mut self)  ->  Program {
        let mut statements = vec![];
        let mut imports = vec![];
        let mut main_entry = vec![];
        let mut functions = vec![];
        println!("{}", "    Parser: parse_program start".to_string());
        
        while self.current_pos < (self.tokens).len() {
            let tok = self.tokens[self.current_pos].clone();
            if tok.token_type == TokenType::Eof {
                break;
            }
            else if tok.token_type == TokenType::Newline || tok.token_type == TokenType::Dedent {
                self.advance();
            }
            else if tok.token_type == TokenType::Mn {
                // Parse main entry point: mn: followed by indented block
                self.advance(); // consume 'mn'
                
                // Expect colon
                if self.current_pos < (self.tokens).len() && self.tokens[self.current_pos].token_type == TokenType::Colon {
                    self.advance(); // consume ':'
                }
                
                // Skip ALL newlines after mn:
                while self.current_pos < (self.tokens).len() && self.tokens[self.current_pos].token_type == TokenType::Newline {
                    self.advance();
                }
                
                // Expect indent
                if self.current_pos < (self.tokens).len() && self.tokens[self.current_pos].token_type == TokenType::Indent {
                    self.advance();
                }
                
                // Parse main body until dedent or EOF
                while self.current_pos < (self.tokens).len() {
                    let t = self.tokens[self.current_pos].clone();
                    if t.token_type == TokenType::Dedent {
                        self.advance();
                        break;
                    }
                    else if t.token_type == TokenType::Newline || t.token_type == TokenType::Indent {
                        self.advance();
                    }
                    else if t.token_type == TokenType::Eof {
                        break;
                    }
                    else {
                        main_entry.push(self.parse_statement());
                    }
                }
            }
            else if tok.token_type == TokenType::AtImp {
                let imp = self.parse_import();
                if let Statement::ImportStmt(i) = imp {
                    imports.push(i);
                }
            }
            else {
                let stmt = self.parse_statement();
                if let Statement::FunctionDecl(f) = stmt {
                    functions.push(f);
                } else {
                    statements.push(stmt);
                }
            }
        }
        return Program{statements: statements, imports: imports, functions: functions, main_entry: main_entry};

    }
}
pub fn parse(tokens: Vec<Token>)  ->  Program {
    let mut parser = Parser::create(tokens);
    return parser.parse_program();
// GUL v3.2 Compiler - Statement Parser Extension
// Adds statement parsing to the parser

// This file extends the Parser struct with statement parsing methods
// It should be concatenated with parser.mn or imported

// Note: In the actual implementation, these methods would be added to the Parser struct
// For this demo, showing the logical structure

// ========================================
// STATEMENT PARSING METHODS (add to Parser)
// ========================================

}
impl Parser {
    pub fn parse_statement(&mut self)  ->  Statement {
        // Parse a single statement// 
        self.skip_newlines();

        let token = self.current();

        match token.token_type {
            // Check for decorators (Identifier starting with @)
            TokenType::Identifier if token.value.starts_with("@") => {
                 return self.parse_decorated_declaration();
            },
            
            // Declarations
            TokenType::Let => return self.parse_let_statement(),
            TokenType::Var => return self.parse_var_statement(),
            TokenType::Fn => return self.parse_function_declaration(vec![]), // Function declarations without decorators
            TokenType::AtFn => return self.parse_function_declaration(vec![]),
            // TokenType.Async => return self.parse_async_function_declaration()
            TokenType::Struct => return self.parse_struct_declaration(),
            // TokenType.Enum => return self.parse_enum_declaration()

            // Control flow
            TokenType::If => return self.parse_if_statement(),
            TokenType::While => return self.parse_while_statement(),
            TokenType::For => return self.parse_for_statement(),
            TokenType::Parallel => return self.parse_parallel_for_statement(),
            TokenType::AtFlow => return self.parse_flow_declaration(),
            // TokenType.Loop => return self.parse_loop_statement()
            // TokenType.Match => return self.parse_match_statement()

            // Flow control
            // TokenType.Break => return self.parse_break_statement()
            // TokenType.Continue => return self.parse_continue_statement()
            TokenType::Return => return self.parse_return_statement(),

            // Error handling
            TokenType::Try => return self.parse_try_statement(),

            // Imports: @imp is a single token type usually
            TokenType::AtImp => return self.parse_import(),

            // Foreign code: specific tokens
            TokenType::AtPython => return self.parse_foreign_block("python"),
            TokenType::AtRust => return self.parse_foreign_block("rust"),
            TokenType::AtJs => return self.parse_foreign_block("js"),
            TokenType::AtSql => return self.parse_foreign_block("sql"),

            _ =>
                // Try parsing as expression statement or assignment
                return self.parse_expression_or_assignment(),

        }
    }
    pub fn parse_let_statement(&mut self)  ->  Statement {
        // Parse let declaration: let x = 5 || let x: int = 5// 
        let let_token = self.advance();

        let name_token = self.expect(TokenType::Identifier, "Expected variable name after 'let'".to_string());
        let name = name_token.value;

        let mut type_annotation = "".to_string();
        if self.match_token(TokenType::Colon) {
            let type_token = self.expect(TokenType::Identifier, "Expected type after ':'".to_string());
            type_annotation = type_token.value;

        }
        self.expect(TokenType::Equal, "Expected '=' in let declaration".to_string());
        let value = self.parse_expression(Precedence::NoPrec);

        return Statement::LetDecl(LetStmt{
            node: create_node(let_token.line, let_token.column),
            name: name,
            type_annotation: type_annotation,
            value: value,
            decorators: vec![],
        });

    }
    pub fn parse_var_statement(&mut self)  ->  Statement {
        // Parse var declaration: var x = 5// 
        let var_token = self.advance();

        let name_token = self.expect(TokenType::Identifier, "Expected variable name after 'var'".to_string());
        let name = name_token.value;

        let mut type_annotation = "".to_string();
        if self.match_token(TokenType::Colon) {
            let type_token = self.expect(TokenType::Identifier, "Expected type after ':'".to_string());
            type_annotation = type_token.value;

        }
        self.expect(TokenType::Equal, "Expected '=' in var declaration".to_string());
        let value = self.parse_expression(Precedence::NoPrec);

        return Statement::VarDecl(VarStmt{
            node: create_node(var_token.line, var_token.column),
            name: name,
            type_annotation: type_annotation,
            value: value,
            decorators: vec![],
        });

    }
    pub fn parse_decorated_declaration(&mut self) -> Statement {
        // Parse declaration preceded by decorators
        let mut decorators = vec![];
        
        // In current lexer, @name is tokenized as Identifier(value="@name") unless it's a specific keyword.
        // So we look for Identifiers starting with @.
        while self.current().token_type == TokenType::Identifier && self.current().value.starts_with("@") {
            let token = self.advance();
            decorators.push(token.value);
            self.skip_newlines(); 
        }
        
        // Now expect a function declaration (or potentially others later)
        if self.match_token(TokenType::Fn) || self.match_token(TokenType::AtFn) {
             return self.parse_function_declaration(decorators);
        } else if self.current().token_type == TokenType::Fn || self.current().token_type == TokenType::AtFn {
             return self.parse_function_declaration(decorators);
        } else {
             // If we consumed decorators but didn't find a function, it's an error (or a standalone statement?)
             // For now, error.
             if !decorators.is_empty() {
                 println!("Parse Error: Expected declaration after decorators");
             }
             // Fallback or re-dispatch? 
             // If no decorators were found, we shouldn't be here really, but the match arm calls this for 'At'.
             // Wait, the match arm in parse_statement calls this for 'TokenType::At'.
             // But we just established 'At' doesn't exist.
             // We need to fix parse_statement too!
             
             return Statement::PassStmt(PassStmt{node: create_node(0,0)}); 
        }
    }

    pub fn parse_function_declaration(&mut self, decorators: Vec<String>)  ->  Statement {
        // Parse function: fn add(a, b) -> int: || @fn add(a, b) -> int: ...// 
        let fn_token = self.advance(); // Consumes 'fn' or '@fn'

        // Optional return type annotation before name
        let mut return_type = "".to_string();
        if self.current().token_type == TokenType::AtInt || self.current().token_type == TokenType::AtStr {
            let type_token = self.advance();
            return_type = type_token.value.chars().skip(1).collect(); // Remove @

        }
        let name_token = self.expect(TokenType::Identifier, "Expected function name".to_string());
        let name = name_token.value;

        // Parameters
        self.expect(TokenType::LeftParen, "Expected '(' after function name".to_string());
        let mut parameters = vec![];

        while ! self.match_token(TokenType::RightParen) {
            let param = self.parse_parameter();
            parameters.push(param);
            if ! self.match_token(TokenType::Comma) {
                self.expect(TokenType::RightParen, "Expected ')' || ',' in parameters".to_string());
                break;

        // Return type after params
            }
        }
        if self.match_token(TokenType::Arrow) {
            let type_token = self.expect(TokenType::Identifier, "Expected return type after '->'".to_string());
            return_type = type_token.value;

        // Body
        }
        self.expect(TokenType::Colon, "Expected ':' before function body".to_string());
        self.skip_newlines();
        self.expect(TokenType::Indent, "Expected indented block for function body".to_string());

        let body = self.parse_block();

        return Statement::FunctionDecl(FunctionDecl{
            node: create_node(fn_token.line, fn_token.column),
            name: name,
            is_async: false,
            parameters: parameters,
            return_type: return_type,
            body: body,
            decorators: decorators,
        });

    }
    pub fn parse_parameter(&mut self)  ->  Parameter {
        // Parse function parameter: name: type || {x,y}: type || [a,b]: type
        let mut name = "".to_string();
        let mut pattern = None;
        let mut type_annotation = "".to_string();
        let mut ownership_mode = "".to_string();
        let mut default_value = None;

        if self.current().token_type == TokenType::Identifier {
             let name_token = self.advance();
             name = name_token.value;
        } else if self.current().token_type == TokenType::LeftBrace || self.current().token_type == TokenType::LeftBracket {
             // Destructuring pattern
             pattern = Some(self.parse_expression(Precedence::NoPrec)); 
        } else {
             self.expect(TokenType::Identifier, "Expected parameter name or destructuring pattern".to_string());
        }

        // Optional type annotation
        if self.match_token(TokenType::Colon) {
            // Check for ownership mode
            if self.current().token_type == TokenType::Borrow || self.current().token_type == TokenType::Ref || self.current().token_type == TokenType::Move || self.current().token_type == TokenType::Kept {
                ownership_mode = self.advance().value;

            }
            let type_token = self.expect(TokenType::Identifier, "Expected parameter type".to_string());
            type_annotation = type_token.value;

        }
        
        // Default value
        if self.match_token(TokenType::Equal) {
             default_value = Some(self.parse_expression(Precedence::NoPrec));
        }

        return Parameter{
            name: name,
            pattern: pattern,
            type_annotation: type_annotation,
            ownership_mode: ownership_mode,
            default_value: default_value, 
        };

    }
    pub fn parse_block(&mut self)  ->  Vec<Statement> {
        // Parse indented block of statements// 
        let mut statements = vec![];

        while ! self.is_at_end() && self.current().token_type != TokenType::Dedent {
            self.skip_newlines();
            if self.current().token_type == TokenType::Dedent {
                break;
            }
            statements.push(self.parse_statement());

        }
        self.match_token(TokenType::Dedent); // Consume dedent
        return statements;

    }
    pub fn parse_if_statement(&mut self)  ->  Statement {
        // Parse if statement with elif && else// 
    let if_token = self.advance();

        let condition = self.parse_expression(Precedence::NoPrec);
        self.expect(TokenType::Colon, "Expected ':' after if condition".to_string());
        self.skip_newlines();
        self.expect(TokenType::Indent, "Expected indented block".to_string());

        let then_body = self.parse_block();

        let mut elif_clauses = vec![];
        let mut else_body = vec![];

        // Parse elif clauses
        while self.current().token_type == TokenType::Elif {
            self.advance();
            let elif_condition = self.parse_expression(Precedence::NoPrec);
            self.expect(TokenType::Colon, "Expected ':' after elif condition".to_string());
            self.skip_newlines();
            self.expect(TokenType::Indent, "Expected indented block".to_string());
            let elif_body = self.parse_block();

            let clause = ElifClause{
                condition: elif_condition,
                body: elif_body,
            };
            elif_clauses.push(clause);
        }

        // Parse else clause
        if self.match_token(TokenType::Else) {
            self.expect(TokenType::Colon, "Expected ':' after else".to_string());
            self.skip_newlines();
            self.expect(TokenType::Indent, "Expected indented block".to_string());
            else_body = self.parse_block();

        }
        return Statement::IfStmt(IfStmt{
            node: create_node(if_token.line, if_token.column),
            condition: condition,
            then_body: then_body,
            elif_clauses: elif_clauses,
            else_body: else_body,
        });

    }
    pub fn parse_while_statement(&mut self)  ->  Statement {
        // Parse while loop// 
        let while_token = self.advance();
        let condition = self.parse_expression(Precedence::NoPrec);
        self.expect(TokenType::Colon, "Expected ':' after while condition".to_string());
        self.skip_newlines();
        self.expect(TokenType::Indent, "Expected indented block".to_string());
        let body = self.parse_block();

        return Statement::WhileStmt(WhileStmt{
            node: create_node(while_token.line, while_token.column),
            condition: condition,
            body: body,
        });

    }
    pub fn parse_tabl_literal(&mut self) -> Expression {
        // @tabl { (col1, col2): row1: {1, 2} }
        // Note: @tabl token was already consumed by parse_prefix
        let mut is_sparse = false;
        
        if self.current().token_type == TokenType::Identifier && self.current().value == "sparse".to_string() {
             is_sparse = true;
             self.advance();
        }

        self.expect(TokenType::LeftBrace, "Expected '{' for table".to_string());
        self.skip_newlines();
        
        // Columns: (c1, c2):
        self.expect(TokenType::LeftParen, "Expected '(' for columns".to_string());
        let mut columns = vec![];
        while !self.match_token(TokenType::RightParen) {
            let col = self.expect(TokenType::Identifier, "Expected column name".to_string());
            columns.push(col.value);
            if !self.match_token(TokenType::Comma) {
                 if self.current().token_type != TokenType::RightParen {
                      // Skip if not comma and not paren
                 }
            }
        }
        self.expect(TokenType::Colon, "Expected ':' after columns".to_string());
        self.skip_newlines();

        // Rows
        let mut rows = vec![];
        while self.current().token_type != TokenType::RightBrace && self.current().token_type != TokenType::Eof {
             if self.current().token_type == TokenType::Newline {
                 self.advance();
                 continue;
             }
             if self.current().token_type == TokenType::Dedent {
                 self.advance();
                 continue;
             }
             
             let row_name = self.expect(TokenType::Identifier, "Expected row name".to_string());
             self.expect(TokenType::Colon, "Expected ':'".to_string());
             self.expect(TokenType::LeftBrace, "Expected '{'".to_string());
             let mut values = vec![];
             while self.current().token_type != TokenType::RightBrace && self.current().token_type != TokenType::Eof {
                  values.push(self.parse_expression(Precedence::NoPrec));
                  if !self.match_token(TokenType::Comma) {
                       break;
                  }
             }
             self.expect(TokenType::RightBrace, "Expected '}'".to_string());
             rows.push(TableRow{name: row_name.value, values: values});
             
             self.match_token(TokenType::Comma);
             self.skip_newlines();
        }
        self.match_token(TokenType::RightBrace);

        return Expression::Table(TableExpr{
             node: create_node(1, 1),
             columns: columns,
             rows: rows,
             is_sparse: is_sparse
        });
    }

    pub fn parse_try_statement(&mut self) -> Statement {
         // try: block catch e: block finally: block
         let try_token = self.advance(); // consume try
         self.expect(TokenType::Colon, "Expected ':' after try".to_string());
         self.skip_newlines();
         self.expect(TokenType::Indent, "Expected indented block for try".to_string());
         let try_body = self.parse_block();
         
         let mut catch_clauses = vec![];
         let mut finally_body = vec![];
         
         // Catch clauses
         while self.match_token(TokenType::Catch) {
              let mut var_name = "e".to_string();
              let mut exc_type = "Exception".to_string();
              
              if self.current().token_type == TokenType::Identifier {
                   var_name = self.advance().value;
              }
              
              self.expect(TokenType::Colon, "Expected ':' after catch".to_string());
              self.skip_newlines();
              self.expect(TokenType::Indent, "Expected indented block for catch".to_string());
              let catch_body = self.parse_block();
              
              catch_clauses.push(CatchClause{
                  exception_type: exc_type,
                  variable_name: var_name,
                  body: catch_body
              });
         }
         
         // Finally
         if self.match_token(TokenType::Finally) {
              self.expect(TokenType::Colon, "Expected ':' after finally".to_string());
              self.skip_newlines();
              self.expect(TokenType::Indent, "Expected indented block for finally".to_string());
              finally_body = self.parse_block();
         }
         
         return Statement::TryStmt(TryStmt{
             node: create_node(try_token.line, try_token.column),
             try_body: try_body,
             catch_clauses: catch_clauses,
             finally_body: finally_body
         });
    }
    pub fn parse_for_statement(&mut self)  ->  Statement {
        // Parse for loop: for x in items:// 
        let for_token = self.advance();
        let var_token = self.expect(TokenType::Identifier, "Expected variable in for loop".to_string());
        self.expect(TokenType::In, "Expected 'in' in for loop".to_string());
        let iterable = self.parse_expression(Precedence::NoPrec);
        self.expect(TokenType::Colon, "Expected ':' after for".to_string());
        self.skip_newlines();
        self.expect(TokenType::Indent, "Expected indented block".to_string());
        let body = self.parse_block();

        return Statement::ForStmt(ForStmt{
            node: create_node(for_token.line, for_token.column),
            variable: var_token.value,
            iterable: iterable,
            body: body,
            is_parallel: false,
        });

    }
    pub fn parse_return_statement(&mut self)  ->  Statement {
        // Parse return statement// 
        let return_token = self.advance();
        let mut value = None;

        // Check if there's a value to return
        if self.current().token_type != TokenType::Newline && self.current().token_type != TokenType::Eof {
            value = Some(self.parse_expression(Precedence::NoPrec));

        }
        return Statement::ReturnStmt(ReturnStmt{
            node: create_node(return_token.line, return_token.column),
            value: value,
        });

    }
    pub fn parse_import(&mut self)  ->  Statement {
        // Parse import: @imp std.io || @imp std{io, http}// 
        let imp_token = self.advance();

        let mut module_path = vec![];
        let mut items = vec![];

        // Parse module path: std.io.file
        let first_part = self.expect(TokenType::Identifier, "Expected module name".to_string());
        module_path.push(first_part.value);

        while self.match_token(TokenType::Dot) {
            let part = self.expect(TokenType::Identifier, "Expected module part".to_string());
            module_path.push(part.value);

        // Check for {item1, item2} syntax
        }
        if self.match_token(TokenType::LeftBrace) {
            while ! self.match_token(TokenType::RightBrace) {
                let item = self.expect(TokenType::Identifier, "Expected import item".to_string());
                items.push(item.value);
                if ! self.match_token(TokenType::Comma) {
                    self.expect(TokenType::RightBrace, "Expected '}' || ','".to_string());
                    break;

                }
            }
        }
        return Statement::ImportStmt(ImportStmt{
            node: create_node(imp_token.line, imp_token.column),
            module_path: module_path,
            import_type: "single".to_string(),
            items: items,
        });

    }
    pub fn parse_expression_or_assignment(&mut self)  ->  Statement {
        // Parse expression statement || assignment// 
        let start_pos = self.current_pos;
        let expr = self.parse_expression(Precedence::NoPrec);

        // Check if it's an assignment
        if self.current().token_type == TokenType::Equal || self.current().token_type == TokenType::PlusEq || self.current().token_type == TokenType::MinusEq || self.current().token_type == TokenType::StarEq || self.current().token_type == TokenType::SlashEq {
            let op = self.advance();
            let value = self.parse_expression(Precedence::NoPrec);

            return Statement::AssignmentStmt(AssignmentStmt{
                node: create_node(op.line, op.column), // Use OP node as base, or pass expr node manually if possible
                target: expr,
                operator: op.token_type,
                value: value,
            });

        // It's just an expression statement
        }
        return Statement::ExpressionStmt(ExpressionStmt{
            node: create_node(0, 0), // Dummy node since expression doesn't expose it
            expression: expr,
        });

    }
    pub fn skip_newlines(&mut self) {
        while self.current_pos < (self.tokens).len() && self.tokens[self.current_pos].token_type == TokenType::Newline {
            self.advance();
        }
    }

    pub fn parse_struct_declaration(&mut self) -> Statement {
        // Parse struct: struct Point:
        //     x: int
        //     y: int
        //     fn new(): ...
        let struct_token = self.advance();
        let name_token = self.expect(TokenType::Identifier, "Expected struct name".to_string());
        let name = name_token.value;
        
        self.expect(TokenType::Colon, "Expected ':' after struct name".to_string());
        self.skip_newlines();
        self.expect(TokenType::Indent, "Expected indented block for struct body".to_string());
        
        let mut fields = vec![];
        let mut methods = vec![];
        
        while !self.is_at_end() && self.current().token_type != TokenType::Dedent {
            self.skip_newlines();
            if self.current().token_type == TokenType::Dedent {
                break;
            }
            
            // Check for method (fn)
            if self.current().token_type == TokenType::Fn {
                let func_stmt = self.parse_function_declaration(vec![]);
                if let Statement::FunctionDecl(decl) = func_stmt {
                    methods.push(decl);
                }
            } else if self.current().token_type == TokenType::Identifier {
                // Parse field: name: type
                let field_name = self.advance().value;
                self.expect(TokenType::Colon, "Expected ':' after field name".to_string());
                let field_type = self.expect(TokenType::Identifier, "Expected field type".to_string());
                
                fields.push(StructField {
                    name: field_name,
                    type_annotation: field_type.value,
                });
            } else {
                // Skip invalid token or error
                // For robustness, maybe just error?
                // self.advance();
                // But let's error to be strict
                 // self.error("Expected field or method declaration in struct".to_string());
                 // Since error panic? No.
                 // Just break loop or advance.
                 self.advance();
            }
        }
        self.match_token(TokenType::Dedent);
        
        return Statement::StructDecl(StructDecl{
            node: create_node(struct_token.line, struct_token.column),
            name: name,
            fields: fields,
            methods: methods,
        });
    }

    pub fn parse_foreign_block(&mut self, lang: &str) -> Statement {
        // Parse foreign block: @python: ...
        let token = self.advance(); // The @python token
        
        self.expect(TokenType::Colon, "Expected ':' after foreign block tag".to_string());
        self.skip_newlines();
        
        self.expect(TokenType::Indent, "Expected indented block".to_string());
        let mut code = "".to_string();
        // Consume everything until Dedent?
        // Note: Lexer likely produces tokens for content? 
        // If content is pure text, Lexer might fail?
        // We assume Lexer tokenizes content as Identifiers/Symbols/Literals.
        // Reconstructing source is hard without raw text.
        // For now, we stub by consuming tokens.
        while !self.is_at_end() && self.current().token_type != TokenType::Dedent {
             // Append token value? Spaces lost?
             if self.current().token_type == TokenType::String {
                 code.push_str(&self.current().value);
             } else if self.current().token_type == TokenType::Identifier {
                 code.push_str(&self.current().value);
                 code.push(' ');
             } else {
                 code.push_str(" "); // Placeholder
             }
             self.advance();
        }
        self.match_token(TokenType::Dedent);
        
        return Statement::ForeignCodeBlock(ForeignCodeBlock{
            node: create_node(token.line, token.column),
            language: lang.to_string(),
            code: code,
        });
    }

    pub fn parse_parallel_for_statement(&mut self) -> Statement {
        let p_token = self.advance(); // Parallel
        if self.current().token_type == TokenType::For {
             let mut stmt = self.parse_for_statement();
             if let Statement::ForStmt(ref mut f) = stmt {
                 f.is_parallel = true;
             }
             return stmt;
        } else {
             self.expect(TokenType::For, "Expected 'for' after 'parallel'".to_string());
             return Statement::PassStmt(PassStmt{node: create_node(p_token.line, p_token.column)});
        }
    }

    pub fn parse_flow_declaration(&mut self) -> Statement {
         let token = self.advance(); // @flow
         if self.current().token_type == TokenType::Var {
             let mut stmt = self.parse_var_statement();
             if let Statement::VarDecl(ref mut v) = stmt {
                 v.decorators.push("@flow".to_string());
             }
             return stmt;
         } else {
             self.expect(TokenType::Var, "Expected 'var' declaration for flow".to_string());
             return Statement::PassStmt(PassStmt{node: create_node(token.line, token.column)});
         }
    }

    pub fn parse_dataframe_literal(&mut self) -> Expression {
        let token = self.advance(); // @frame
        self.expect(TokenType::LeftBrace, "Expected '{'".to_string());
        
        let mut columns = vec![];
        let data = std::collections::HashMap::new();

        while self.current().token_type != TokenType::RightBrace && self.current().token_type != TokenType::Eof {
             let key = self.expect(TokenType::Identifier, "Expected key".to_string());
             self.expect(TokenType::Colon, "Expected ':'".to_string());
             
             if key.value == "columns".to_string() {
                 self.expect(TokenType::LeftParen, "Expected '('".to_string());
                 while !self.match_token(TokenType::RightParen) {
                     if self.current().token_type == TokenType::String {
                          columns.push(self.advance().value);
                     } else {
                          self.advance();
                     }
                     self.match_token(TokenType::Comma);
                 }
             } else {
                 self.parse_expression(Precedence::NoPrec); 
             }
             self.match_token(TokenType::Comma);
             self.skip_newlines();
        }
        self.expect(TokenType::RightBrace, "Expected '}'".to_string());
        
        return Expression::DataFrame(DataFrameExpr{
            node: create_node(token.line, token.column),
            columns: columns,
            data: data,
        });
    }
}