// GUL v3.2 Compiler - Recursive Descent Parser
use crate::lexer::token::*;
use crate::ast::nodes::*;

// Enumeration for operator precedence
#[derive(Debug, Clone, PartialEq)]
pub enum Precedence {
    NoPrec,
    Assignment,
    Or,
    And,
    Comparison,
    Term,
    Factor,
    Power,
    Range,
    Prefix,
    Call,
    Index,
    Unpack,

}
pub fn get_precedence(token_type: TokenType)  ->  Precedence {
    // Get the precedence level for a given token type// 
    if token_type == TokenType::Equal {
        return Precedence::Assignment;
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
        return self.tokens[self.current_pos];

    }
    pub fn advance(&mut self)  ->  Token {
        let token = self.tokens[self.current_pos];
        let size = (self.tokens).len();
        if self.current_pos < size - 1 {
            self.current_pos = self.current_pos + 1;
        }
        return token;

    }
    pub fn match_token(&mut self, expected_type: TokenType)  ->  bool {
        let tok = self.tokens[self.current_pos];
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
        let tok = self.tokens[self.current_pos];
        if tok.token_type == expected_type {
            let size = (self.tokens).len();
            if self.current_pos < size - 1 {
                self.current_pos = self.current_pos + 1;
            }
            return tok;
        }
        println!("{}", "Parse error: ".to_string() + message);
        return tok; // fallthrough

    }
    pub fn parse_expression(&mut self, min_precedence: Precedence)  ->  Expression {
        let mut left = self.parse_prefix();
        while true {
            if self.current_pos >= (self.tokens).len() - 1 { break }
            let tok = self.tokens[self.current_pos];
            let prec = get_precedence(tok.token_type);
            if prec <= min_precedence { break }
            left = self.parse_infix(left, prec);
        }
        return left;

    }
    pub fn parse_prefix(&mut self)  ->  Expression {
        let token = self.advance();
        let t_type = token.token_type;
        if t_type == TokenType::Identifier {
            return Expression::Identifier(IdentifierExpr{node: ASTNode{line: 1, column: 1}, name: token.value});
        }
        else if t_type == TokenType::Integer {
            return Expression::Literal(LiteralExpr{node: ASTNode{line: 1, column: 1}, value: token.value, value_type: TokenType::Integer});
        }
        else if t_type == TokenType::LeftParen {
            let expr = self.parse_expression(Precedence::NoPrec);
            self.expect(TokenType::RightParen, "Expected ')'".to_string());
            return expr;
        }
        else if t_type == TokenType::Minus {
            let op = self.parse_expression(Precedence::Prefix);
            return Expression::UnaryOp(UnaryOpExpr{node: ASTNode{line: 1, column: 1}, operator: TokenType::Minus, operand: op});
        }
        return Expression::Literal(LiteralExpr{node: ASTNode{line: 1, column: 1}, value: "None", value_type: TokenType::NoneLiteral});

    }
    pub fn parse_infix(&mut self, left: Expression, precedence: Precedence)  ->  Expression {
        let token = self.advance();
        if token.token_type == TokenType::Plus || token.token_type == TokenType::Minus || token.token_type == TokenType::Star || token.token_type == TokenType::Slash {
            let right = self.parse_expression(precedence);
            return Expression::BinaryOp(BinaryOpExpr{node: ASTNode{line: 1, column: 1}, left: left, operator: token.token_type, right: right});
        }
        else if token.token_type == TokenType::LeftParen {
            let args = vec![];
            if ! (self.tokens[self.current_pos].token_type == TokenType::RightParen) {
                args.push(self.parse_expression(Precedence::NoPrec));
                while self.match_token(TokenType::Comma) {
                    args.push(self.parse_expression(Precedence::NoPrec));
                }
            }
            self.expect(TokenType::RightParen, "Expected ')'".to_string());
            return Expression::Call(CallExpr{node: ASTNode{line: 1, column: 1}, callee: left, arguments: args});
        }
        return left;

    }
    pub fn parse_statement(&mut self)  ->  Statement {
        let tok = self.tokens[self.current_pos];
        if tok.token_type == TokenType::Let {
            self.advance();
            let name = self.expect(TokenType::Identifier, "Expected identifier".to_string()).value;
            self.expect(TokenType::Equal, "Expected '='".to_string());
            let val = self.parse_expression(Precedence::NoPrec);
            return Statement::LetDecl(LetStmt{node: ASTNode{line: 1, column: 1}, name: name, value: val, is_var: false});
        }
        else if tok.token_type == TokenType::Return {
            self.advance();
            let val = self.parse_expression(Precedence::NoPrec);
            return Statement::ReturnStmt(ReturnStmt{node: ASTNode{line: 1, column: 1}, value: val});
        }
        else if tok.token_type == TokenType::Fn {
            self.advance();
            let name = self.expect(TokenType::Identifier, "Expected function name".to_string()).value;
            self.expect(TokenType::LeftParen, "Expected '('".to_string());
            let params = vec![]; // simplified
            while ! (self.tokens[self.current_pos].token_type == TokenType::RightParen) {
                self.advance(); // consume param
                if self.tokens[self.current_pos].token_type == TokenType::Comma { self.advance() }
            }
            self.advance(); // )
            self.expect(TokenType::Colon, "Expected ':'".to_string());
            self.expect(TokenType::Newline, "Expected Newline".to_string());
            self.expect(TokenType::Indent, "Expected Indent".to_string());
            let body = vec![];
            while ! (self.tokens[self.current_pos].token_type == TokenType::Dedent) {
                body.push(self.parse_statement());
                if self.tokens[self.current_pos].token_type == TokenType::Newline { self.advance() }
            }
            self.advance(); // Dedent
            return Statement::FunctionDecl(FunctionDecl{node: ASTNode{line: 1, column: 1}, name: name, params: vec![], body: body, return_type: "Any"});

        // Expression statement
        }
        let expr = self.parse_expression(Precedence::NoPrec);
        return Statement::ExpressionStmt(ExpressionStmt{node: ASTNode{line: 1, column: 1}, expression: expr});

    }
    pub fn parse_program(&mut self)  ->  Program {
        let statements = vec![];
        println!("{}", "    Parser: parse_program start".to_string());
        while self.current_pos < (self.tokens).len() - 1 {
            println!("{}", "    Parser POS: ".to_string() + &format!("{}", self.current_pos));
            let tok = self.tokens[self.current_pos];
            if tok.token_type == TokenType::Newline {
                self.advance();
            }
            else {
                statements.push(self.parse_statement());
            }
        }
        return Program{statements: statements, imports: vec![], main_entry: vec![]};

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
            // Declarations (TODO)
            TokenType::Let => return self.parse_let_statement(),
            TokenType::Var => return self.parse_var_statement(),
            TokenType::Fn => return self.parse_function_declaration(),
            // TokenType.Async => return self.parse_async_function_declaration()
            // TokenType.Struct => return self.parse_struct_declaration()
            // TokenType.Enum => return self.parse_enum_declaration()

            // Control flow
            TokenType::If => return self.parse_if_statement(),
            TokenType::While => return self.parse_while_statement(),
            TokenType::For => return self.parse_for_statement(),
            // TokenType.Loop => return self.parse_loop_statement()
            // TokenType.Match => return self.parse_match_statement()

            // Flow control
            // TokenType.Break => return self.parse_break_statement()
            // TokenType.Continue => return self.parse_continue_statement()
            TokenType::Return => return self.parse_return_statement(),

            // Error handling
            // TokenType.Try => return self.parse_try_statement()

            // Imports
            TokenType::AtImp => return self.parse_import(),

            // Foreign code
            // TokenType.AtPython => return self.parse_foreign_block("python")
            // TokenType.AtRust => return self.parse_foreign_block("rust")
            // TokenType.AtJs => return self.parse_foreign_block("js")
            // TokenType.AtSql => return self.parse_foreign_block("sql")

            _ =>
                // Try parsing as expression statement or assignment
                return self.parse_expression_or_assignment();

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
        let value = self.parse_expression();

        return Statement::LetDecl(LetStmt{
            node: create_node(let_token.line, let_token.column),
            name: name,
            type_annotation: type_annotation,
            value: value,
        });
        ;

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
        let value = self.parse_expression();

        return Statement::VarDecl(VarStmt{
            node: create_node(var_token.line, var_token.column),
            name: name,
            type_annotation: type_annotation,
            value: value,
        });
        ;

    }
    pub fn parse_function_declaration(&mut self)  ->  Statement {
        // Parse function: fn add(a, b) -> int: ...// 
        let fn_token = self.advance();

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
            decorators: vec![],
        });
        ;

    }
    pub fn parse_parameter(&mut self)  ->  Parameter {
        // Parse function parameter: name: type || name// 
        let name_token = self.expect(TokenType::Identifier, "Expected parameter name".to_string());
        let mut type_annotation = "".to_string();
        let mut ownership_mode = "".to_string();

        // Optional type annotation
        if self.match_token(TokenType::Colon) {
            // Check for ownership mode
            if self.current().token_type == TokenType::Borrow || self.current().token_type == TokenType::Ref || self.current().token_type == TokenType::Move || self.current().token_type == TokenType::Kept {
                ownership_mode = self.advance().value;

            }
            let type_token = self.expect(TokenType::Identifier, "Expected parameter type".to_string());
            type_annotation = type_token.value;

        }
        return Parameter{
            name: name_token.value,
            type_annotation: type_annotation,
            ownership_mode: ownership_mode,
            default_value: None, // TODO: parse default values
        };
        ;

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

        let condition = self.parse_expression();
        self.expect(TokenType::Colon, "Expected ':' after if condition".to_string());
        self.skip_newlines();
        self.expect(TokenType::Indent, "Expected indented block".to_string());

        let then_body = self.parse_block();

        let mut elif_clauses = vec![];
        let mut else_body = vec![];

        // Parse elif clauses
        while self.current().token_type == TokenType::Elif {
            self.advance();
            let elif_condition = self.parse_expression();
            self.expect(TokenType::Colon, "Expected ':' after elif condition".to_string());
            self.skip_newlines();
            self.expect(TokenType::Indent, "Expected indented block".to_string());
            let elif_body = self.parse_block();

            let clause = ElifClause{
                condition: elif_condition,
                body: elif_body,
            };
            ;
        }
        elif_clauses.push(clause);


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
        ;

    }
    pub fn parse_while_statement(&mut self)  ->  Statement {
        // Parse while loop// 
        let while_token = self.advance();
        let condition = self.parse_expression();
        self.expect(TokenType::Colon, "Expected ':' after while condition".to_string());
        self.skip_newlines();
        self.expect(TokenType::Indent, "Expected indented block".to_string());
        let body = self.parse_block();

        return Statement::WhileStmt(WhileStmt{
            node: create_node(while_token.line, while_token.column),
            condition: condition,
            body: body,
        });
        ;

    }
    pub fn parse_for_statement(&mut self)  ->  Statement {
        // Parse for loop: for x in items:// 
        let for_token = self.advance();
        let var_token = self.expect(TokenType::Identifier, "Expected variable in for loop".to_string());
        self.expect(TokenType::In, "Expected 'in' in for loop".to_string());
        let iterable = self.parse_expression();
        self.expect(TokenType::Colon, "Expected ':' after for".to_string());
        self.skip_newlines();
        self.expect(TokenType::Indent, "Expected indented block".to_string());
        let body = self.parse_block();

        return Statement::ForStmt(ForStmt{
            node: create_node(for_token.line, for_token.column),
            variable: var_token.value,
            iterable: iterable,
            body: body,
        });
        ;

    }
    pub fn parse_return_statement(&mut self)  ->  Statement {
        // Parse return statement// 
        let return_token = self.advance();
        let mut value = None;

        // Check if there's a value to return
        if self.current().token_type != TokenType::Newline && self.current().token_type != TokenType::Eof {
            value = self.parse_expression();

        }
        return Statement::ReturnStmt(ReturnStmt{
            node: create_node(return_token.line, return_token.column),
            value: value,
        });
        ;

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
            import_type: "single",
            items: items,
        });
        ;

    }
    pub fn parse_expression_or_assignment(&mut self)  ->  Statement {
        // Parse expression statement || assignment// 
        let start_pos = self.current_pos;
        let expr = self.parse_expression();

        // Check if it's an assignment
        if self.current().token_type == TokenType::Equal || self.current().token_type == TokenType::PlusEq || self.current().token_type == TokenType::MinusEq || self.current().token_type == TokenType::StarEq || self.current().token_type == TokenType::SlashEq {
            let op = self.advance();
            let value = self.parse_expression();

            return Statement::AssignmentStmt(AssignmentStmt{
                node: expr.node,
                target: expr,
                operator: op.token_type,
                value: value,
            });
            ;

        // It's just an expression statement
        }
        return Statement::ExpressionStmt(ExpressionStmt{
            node: expr.node,
            expression: expr,
        });
        ;

    }
    pub fn skip_newlines(&mut self) {
        while self.current_pos < (self.tokens).len() && self.tokens[self.current_pos].token_type == TokenType::Newline {
            self.advance();
        }
    }
}