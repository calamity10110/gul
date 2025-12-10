#![allow(unused_mut)]
// Parser module - builds AST from tokens

use crate::ast::{Type, *};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    fn peek(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.position + offset)
            .unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                expected,
                self.current_token()
            ))
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current_token(), Token::Newline) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while self.current_token() != &Token::Eof {
            self.skip_newlines();

            if self.current_token() == &Token::Eof {
                break;
            }

            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    // Context-aware parsing methods
    pub fn parse_def_file(&mut self) -> Result<Program, String> {
        self.parse_tokens_strict(|stmt| {
            matches!(
                stmt,
                Statement::Import(_)
                    | Statement::Definition { .. }
                    | Statement::GlobalDef { .. }
                    | Statement::StructDef { .. }
                    | Statement::ForeignBlock { .. } // Allow extern blocks in def files
            )
        })
    }

    pub fn parse_fnc_file(&mut self) -> Result<Program, String> {
        self.parse_tokens_strict(|stmt| {
            matches!(
                stmt,
                Statement::Function { .. } |
            Statement::Import(_) | // Allow imports in fnc for now
            Statement::ForeignBlock { .. }
            )
        })
    }

    pub fn parse_mn_file(&mut self) -> Result<Program, String> {
        self.parse_tokens_strict(|stmt| {
            matches!(stmt, Statement::Main { .. } | Statement::Import(_))
        })
    }

    fn parse_tokens_strict<F>(&mut self, validator: F) -> Result<Program, String>
    where
        F: Fn(&Statement) -> bool,
    {
        let mut statements = Vec::new();
        while self.current_token() != &Token::Eof {
            self.skip_newlines();
            if self.current_token() == &Token::Eof {
                break;
            }

            let stmt = self.parse_statement()?;
            if !validator(&stmt) {
                return Err(format!(
                    "Statement type {:?} not allowed in this file context",
                    stmt
                ));
            }
            statements.push(stmt);
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token() {
            Token::Imp | Token::Import => self.parse_import(),
            Token::Def => self.parse_definition(),
            Token::Const => self.parse_const_definition(),
            Token::Mut => self.parse_mut_definition(),
            Token::Struct => self.parse_struct(),
            Token::Fn => self.parse_function(false),
            Token::Async => self.parse_function(true),
            Token::Asy => self.parse_function(true), // legacy support
            Token::Extern => self.parse_extern_block(),
            Token::Cs => self.parse_custom_block(), // legacy support
            Token::Main => self.parse_main(),
            Token::Mn => self.parse_main(), // legacy support
            Token::If => self.parse_if(),
            Token::Loop => self.parse_loop(),
            Token::For => self.parse_for(),
            Token::While => self.parse_while(),
            Token::Return => self.parse_return(),
            Token::Break => {
                self.advance();
                self.skip_newlines();
                Ok(Statement::Break)
            }
            Token::Continue => {
                self.advance();
                self.skip_newlines();
                Ok(Statement::Continue)
            }
            Token::Try => self.parse_try_catch(),
            Token::At => self.parse_annotation_statement(),
            Token::QuestionMark => self.parse_mutable_assignment(),
            Token::Throw => {
                self.advance();
                let expr = self.parse_expression()?;
                self.skip_newlines();
                Ok(Statement::Throw(expr))
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_annotation_statement(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip '@'

        match self.current_token() {
            Token::Global => self.parse_global_def(),
            Token::Fn => self.parse_function(false),
            Token::Asy => self.parse_function(true),
            Token::Cs => self.parse_custom_block(), // @cs syntax
            _ => Err("Unexpected annotation".to_string()),
        }
    }

    fn parse_global_def(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'global'

        let mutable = if self.current_token() == &Token::QuestionMark {
            self.advance();
            true
        } else {
            false
        };

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            // Handle optional dotted path for state (e.g. ?game_state.high_score)
            let mut full_name = var_name;
            while self.current_token() == &Token::Dot {
                self.advance();
                if let Token::Identifier(part) = self.current_token() {
                    full_name.push('.');
                    full_name.push_str(part);
                    self.advance();
                }
            }

            self.expect(Token::Equal)?;
            let value = self.parse_expression()?;
            self.skip_newlines();

            Ok(Statement::GlobalDef {
                name: full_name,
                value,
                mutable,
            })
        } else {
            Err("Expected identifier after @global".to_string())
        }
    }

    fn parse_mutable_assignment(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip '?'

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            self.expect(Token::Equal)?;
            let value = self.parse_expression()?;
            self.skip_newlines();

            // Treat as assignment to mutable variable, or definition if new?
            // For now map to Assignment, but name includes '?' implicit check in interpreter?
            // Actually, let's map to Definition if we want it to declare.
            // But spec says "?count = 0" is mutable variable.
            // Reuse Definition node but perhaps mark it?
            // Or just convention: name starts with `?`?
            // Let's prepend `?` to name to keep convention from `parse_definition`.

            Ok(Statement::Assignment {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after '?'".to_string())
        }
    }

    fn parse_import(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'imp' or 'import'

        // Grouped imports: imp [ ... ] or imp { ... }
        if matches!(
            self.current_token(),
            Token::LeftBracket | Token::LeftBrace | Token::LeftParen
        ) {
            self.advance(); // Skip opener

            // Very basic grouped import skipping for now, just to pass parser
            // In real v2, we would parse list of modules
            while !matches!(
                self.current_token(),
                Token::RightBracket | Token::RightBrace | Token::RightParen | Token::Eof
            ) {
                self.advance();
            }
            if self.current_token() != &Token::Eof {
                self.advance(); // Skip closer
            }
            self.skip_newlines();
            return Ok(Statement::Import("grouped_placeholder".to_string()));
        }

        if let Token::Identifier(first_part) = self.current_token() {
            let mut module_path = first_part.clone();
            self.advance();

            // Handle dotted imports and colons
            while matches!(self.current_token(), Token::Dot | Token::Colon) {
                let is_colon = self.current_token() == &Token::Colon;
                self.advance();
                if is_colon {
                    module_path.push(':');
                } else {
                    module_path.push('.');
                }

                if let Token::Identifier(part) = self.current_token() {
                    module_path.push_str(part);
                    self.advance();
                } else if matches!(self.current_token(), Token::LeftParen | Token::LeftBrace) {
                    // Handle module: (a, b)
                    self.advance();
                    while !matches!(
                        self.current_token(),
                        Token::RightParen | Token::RightBrace | Token::Eof
                    ) {
                        self.advance();
                    }
                    self.advance(); // closer
                }
            }

            self.skip_newlines();
            Ok(Statement::Import(module_path))
        } else {
            Err("Expected module name".to_string())
        }
    }

    fn parse_struct(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'struct'

        if let Token::Identifier(name) = self.current_token() {
            let struct_name = name.clone();
            self.advance();

            self.expect(Token::Colon)?;
            self.expect(Token::Newline)?;
            self.expect(Token::Indent)?;

            let mut fields = Vec::new();
            let mut methods = Vec::new();

            while self.current_token() != &Token::Dedent && self.current_token() != &Token::Eof {
                self.skip_newlines();
                if self.current_token() == &Token::Dedent || self.current_token() == &Token::Eof {
                    break;
                }

                if matches!(self.current_token(), Token::Fn | Token::Asy) {
                    let is_async = self.current_token() == &Token::Asy;
                    methods.push(self.parse_function(is_async)?);
                } else if let Token::Identifier(field_name) = self.current_token() {
                    let fname = field_name.clone();
                    self.advance();
                    self.expect(Token::Colon)?;

                    let type_name = self.parse_type_annotation()?;
                    fields.push((fname, type_name));

                    self.skip_newlines();
                } else {
                    return Err(format!(
                        "Unexpected token in struct: {:?}",
                        self.current_token()
                    ));
                }
            }
            self.expect(Token::Dedent)?;

            Ok(Statement::StructDef {
                name: struct_name,
                fields,
                methods,
            })
        } else {
            Err("Expected struct name".to_string())
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.current_token() {
            Token::Identifier(name) => {
                let type_name = name.clone();
                self.advance();

                match type_name.as_str() {
                    "int" => Ok(Type::Int),
                    "float" => Ok(Type::Float),
                    "str" => Ok(Type::String),
                    "bool" => Ok(Type::Bool),
                    "any" => Ok(Type::Any),
                    _unit_name => {
                        // Check for unit types
                        if let Token::Unit(unit) = self.current_token() {
                            let unit_name = unit.clone();
                            self.advance();
                            Ok(Type::Unit(unit_name))
                        } else {
                            // For now, treat unknown identifiers as custom types
                            // In the future, this could resolve to struct names
                            Ok(Type::Any) // Fallback to any for gradual typing
                        }
                    }
                }
            }
            _ => Err("Expected type name".to_string()),
        }
    }

    fn parse_try_catch(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'try'

        self.expect(Token::Colon)?;
        self.skip_newlines();

        // Parse try block
        let try_body = self.parse_block()?;

        let mut catch_var = None;
        let mut catch_body = None;
        let mut finally_body = None;

        // Check for catch
        if self.current_token() == &Token::Catch {
            self.advance(); // Skip 'catch'

            // Optional exception variable
            if let Token::Identifier(var_name) = self.current_token() {
                catch_var = Some(var_name.clone());
                self.advance();
            }

            self.expect(Token::Colon)?;
            self.skip_newlines();
            catch_body = Some(self.parse_block()?);
        }

        // Check for finally
        if self.current_token() == &Token::Finally {
            self.advance(); // Skip 'finally'
            self.expect(Token::Colon)?;
            self.skip_newlines();
            finally_body = Some(self.parse_block()?);
        }

        Ok(Statement::Try {
            try_body,
            catch_var,
            catch_body,
            finally_body,
        })
    }

    fn parse_type_annotation(&mut self) -> Result<String, String> {
        if let Token::Identifier(name) = self.current_token() {
            let mut type_str = name.clone();
            self.advance();

            if self.current_token() == &Token::Less {
                self.advance();
                type_str.push('<');
                type_str.push_str(&self.parse_type_annotation()?);

                while self.current_token() == &Token::Comma {
                    self.advance();
                    type_str.push_str(", ");
                    type_str.push_str(&self.parse_type_annotation()?);
                }

                self.expect(Token::Greater)?;
                type_str.push('>');
            }
            Ok(type_str)
        } else {
            // Handle primitive types like 'int' if they are tokens, or fallback
            if let Token::Identifier(s) = self.current_token() {
                let t = s.clone();
                self.advance();
                Ok(t)
            } else {
                // For now return "any" if unknown
                Ok("any".to_string())
            }
        }
    }

    fn parse_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'def'

        // Check for ownership keywords FIRST
        let mut _ownership: Option<Ownership> = None;
        if matches!(self.current_token(), Token::Own | Token::Ref | Token::Copy) {
            self.advance();
            // Map to local if needed, currently just consuming
        }

        // Check for mutability marker (?)
        let _is_mutable = if self.current_token() == &Token::QuestionMark {
            self.advance();
            true
        } else {
            false
        };

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone(); // Store without ? prefix
            self.advance();

            // Handle optional type annotation (@int, @str, etc.)
            if self.current_token() == &Token::Colon {
                self.advance();
                // Skip type annotation for now
                if self.current_token() == &Token::At {
                    self.advance();
                }
                if let Token::Identifier(_type_name) = self.current_token() {
                    self.advance();
                }
            }

            self.expect(Token::Equal)?;

            let value = self.parse_expression()?;
            self.skip_newlines();

            Ok(Statement::Definition {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after 'def'".to_string())
        }
    }

    fn parse_function(&mut self, is_async_fn: bool) -> Result<Statement, String> {
        self.advance(); // Skip 'fn' or 'asy'

        if let Token::Identifier(name) = self.current_token() {
            let func_name = name.clone();
            self.advance();

            self.expect(Token::LeftParen)?;

            let mut params = Vec::new();
            while self.current_token() != &Token::RightParen {
                // Handle ownership keywords
                let _ownership = match self.current_token() {
                    Token::Own => {
                        self.advance();
                        Some(Ownership::Own)
                    }
                    Token::Ref => {
                        self.advance();
                        Some(Ownership::Ref)
                    }
                    Token::Copy => {
                        self.advance();
                        Some(Ownership::Copy)
                    }
                    _ => None,
                };

                if let Token::Identifier(param) = self.current_token() {
                    params.push(param.clone());
                    self.advance();

                    if self.current_token() == &Token::Comma {
                        self.advance();
                    }
                } else {
                    break;
                }
            }

            self.expect(Token::RightParen)?;
            self.expect(Token::Colon)?;
            self.skip_newlines();

            // Parse function body
            let body = self.parse_block()?;

            Ok(Statement::Function {
                name: func_name,
                params,
                body,
                is_async: is_async_fn,
            })
        } else {
            Err("Expected function name".to_string())
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        self.expect(Token::Indent)?;

        loop {
            self.skip_newlines();

            match self.current_token() {
                Token::Eof => break,
                Token::Dedent => {
                    self.advance();
                    break;
                }
                _ => {
                    statements.push(self.parse_statement()?);
                }
            }
        }

        Ok(statements)
    }

    #[allow(dead_code)]
    fn is_block_end(&self) -> bool {
        // Simple heuristic: if we see certain keywords at the start, it's likely a new block
        matches!(self.current_token(), Token::Elif | Token::Else | Token::Eof)
    }

    fn parse_custom_block(&mut self) -> Result<Statement, String> {
        // Handle both 'cs' and '@cs' syntax
        if self.current_token() == &Token::At {
            self.advance(); // Skip '@'
        }
        self.advance(); // Skip 'cs'

        if let Token::Identifier(lang) = self.current_token() {
            let language = lang.clone();
            self.advance();

            self.expect(Token::Colon)?;
            self.skip_newlines();

            // Collect all lines until we hit a top-level statement
            let mut code_lines: Vec<String> = Vec::new();

            while self.current_token() != &Token::Eof {
                if matches!(
                    self.current_token(),
                    Token::Def
                        | Token::Fn
                        | Token::Asy
                        | Token::Cs
                        | Token::Mn
                        | Token::Main
                        | Token::Imp
                        | Token::At
                ) {
                    break;
                }

                // Collect the line (simplified - just skip for now)
                // In real implementation, we'd collect the actual code text
                self.advance();

                if self.current_token() == &Token::Newline {
                    self.advance();
                    // Check if next line is dedented (starts with top-level keyword)
                    if matches!(
                        self.current_token(),
                        Token::Def
                            | Token::Fn
                            | Token::Asy
                            | Token::Cs
                            | Token::Mn
                            | Token::Main
                            | Token::Imp
                            | Token::At
                    ) {
                        break;
                    }
                }
            }

            Ok(Statement::ForeignBlock {
                language,
                code: code_lines.join("\n"),
            })
        } else {
            Err("Expected language name after 'cs'".to_string())
        }
    }

    fn parse_main(&mut self) -> Result<Statement, String> {
        let is_new_syntax = matches!(self.current_token(), Token::Main);
        self.advance(); // Skip 'mn' or 'main'

        if is_new_syntax {
            // New syntax: main(): (no 'main' identifier needed)
            self.expect(Token::LeftParen)?;
            self.expect(Token::RightParen)?;
            self.expect(Token::Colon)?;
            self.skip_newlines();

            let body = self.parse_block()?;
            Ok(Statement::Main { body })
        } else {
            // Legacy syntax: mn main():
            // Note: 'main' might be tokenized as Token::Main (keyword) or Token::Identifier("main")
            match self.current_token() {
                Token::Main => {
                    // 'main' was tokenized as keyword
                    self.advance(); // Skip 'main' keyword

                    self.expect(Token::LeftParen)?;
                    self.expect(Token::RightParen)?;
                    self.expect(Token::Colon)?;
                    self.skip_newlines();

                    let body = self.parse_block()?;
                    Ok(Statement::Main { body })
                }
                Token::Identifier(name) if name == "main" => {
                    // 'main' was tokenized as identifier (shouldn't happen but handle it)
                    self.advance(); // Skip 'main' identifier

                    self.expect(Token::LeftParen)?;
                    self.expect(Token::RightParen)?;
                    self.expect(Token::Colon)?;
                    self.skip_newlines();

                    let body = self.parse_block()?;
                    Ok(Statement::Main { body })
                }
                _ => Err("Expected 'main' after 'mn'".to_string()),
            }
        }
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'if'

        let condition = self.parse_expression()?;
        self.expect(Token::Colon)?;
        self.skip_newlines();

        let then_body = self.parse_block()?;

        // Check for elif/else
        let mut else_body = None;

        if self.current_token() == &Token::Elif {
            // Parse elif as nested if-else
            else_body = Some(vec![self.parse_if()?]);
        } else if self.current_token() == &Token::Else {
            self.advance();
            self.expect(Token::Colon)?;
            self.skip_newlines();
            else_body = Some(self.parse_block()?);
        }

        Ok(Statement::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_loop(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'loop'
        self.expect(Token::Colon)?;
        self.skip_newlines();

        let body = self.parse_block()?;

        Ok(Statement::Loop { body })
    }

    fn parse_for(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'for'

        if let Token::Identifier(var) = self.current_token() {
            let variable = var.clone();
            self.advance();

            self.expect(Token::In)?;

            let iterable = self.parse_expression()?;
            self.expect(Token::Colon)?;
            self.skip_newlines();

            let body = self.parse_block()?;

            Ok(Statement::For {
                variable,
                iterable,
                body,
            })
        } else {
            Err("Expected variable name after 'for'".to_string())
        }
    }

    fn parse_while(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'while'

        let condition = self.parse_expression()?;
        self.expect(Token::Colon)?;
        self.skip_newlines();

        let body = self.parse_block()?;

        Ok(Statement::While { condition, body })
    }

    fn parse_return(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'return'

        // Check if there's an expression to return
        let value = if matches!(self.current_token(), Token::Newline | Token::Eof) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.skip_newlines();

        Ok(Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        // Check if this is an assignment (identifier = expression)
        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();

            // Look ahead to see if next token is '='
            if self.peek(1) == &Token::Equal {
                // This is an assignment
                self.advance(); // Skip identifier
                self.advance(); // Skip '='

                let value = self.parse_expression()?;
                self.skip_newlines();

                return Ok(Statement::Assignment {
                    name: var_name,
                    value,
                });
            }
        }

        // Otherwise, parse as expression statement
        let expr = self.parse_expression()?;
        self.skip_newlines();
        Ok(Statement::Expression(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_logical_and()?;

        while self.current_token() == &Token::Or {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_equality()?;

        while self.current_token() == &Token::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison()?;

        while matches!(self.current_token(), Token::EqualEqual | Token::BangEqual) {
            let op = match self.current_token() {
                Token::EqualEqual => BinaryOp::Equal,
                Token::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_addition()?;

        while matches!(
            self.current_token(),
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual
        ) {
            let op = match self.current_token() {
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_addition()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplication()?;

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = match self.current_token() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplication()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_power()?;

        while matches!(
            self.current_token(),
            Token::Star | Token::Slash | Token::Percent
        ) {
            let op = match self.current_token() {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_unary()?;

        if self.current_token() == &Token::Caret {
            self.advance();
            let right = self.parse_power()?; // Right associative
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Power,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        match self.current_token() {
            Token::Not => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Negate,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current_token() {
                Token::LeftParen => {
                    // Function call
                    self.advance();
                    let mut args = Vec::new();

                    while self.current_token() != &Token::RightParen {
                        args.push(self.parse_expression()?);

                        if self.current_token() == &Token::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    self.expect(Token::RightParen)?;

                    expr = Expression::Call {
                        function: Box::new(expr),
                        args,
                    };
                }
                Token::Dot => {
                    // Member access
                    self.advance();

                    if let Token::Identifier(member) = self.current_token() {
                        let member_name = member.clone();
                        self.advance();

                        expr = Expression::Member {
                            object: Box::new(expr),
                            member: member_name,
                        };
                    } else {
                        return Err("Expected member name after '.'".to_string());
                    }
                }
                Token::LeftBracket => {
                    // Index access
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(Token::RightBracket)?;

                    expr = Expression::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.current_token() {
            Token::Integer(n) => {
                let value = *n;
                self.advance();
                Ok(Expression::Integer(value))
            }
            Token::Float(n) => {
                let value = *n;
                self.advance();
                Ok(Expression::Float(value))
            }
            Token::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expression::String(value))
            }
            Token::Bool(b) => {
                let value = *b;
                self.advance();
                Ok(Expression::Bool(value))
            }
            Token::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Ok(Expression::Identifier(id))
            }
            Token::UiSprite(content) => {
                let sprite_content = content.clone();
                self.advance();

                // Parser for Universal Languagent (simplified)
                // Format: sprite_type{prop1=val1, prop2=val2} or just sprite_type
                let parts: Vec<&str> = sprite_content.splitn(2, '{').collect();
                let sprite_type = parts[0].to_string();
                let properties = Vec::<(String, Expression)>::new(); // TODO: Parse properties

                Ok(Expression::UiSprite {
                    sprite_type,
                    properties,
                })
            }
            Token::Await => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Expression::Await(Box::new(expr)))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::LeftBracket => {
                // List literal
                self.advance();
                let mut elements = Vec::new();

                while self.current_token() != &Token::RightBracket {
                    elements.push(self.parse_expression()?);

                    if self.current_token() == &Token::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }

                self.expect(Token::RightBracket)?;
                Ok(Expression::List(elements))
            }
            Token::LeftBrace => {
                // Dict literal
                self.advance();
                let mut pairs = Vec::new();

                while self.current_token() != &Token::RightBrace {
                    if let Token::Identifier(key) = self.current_token() {
                        let key_name = key.clone();
                        self.advance();

                        self.expect(Token::Colon)?;

                        let value = self.parse_expression()?;
                        pairs.push((key_name, value));

                        if self.current_token() == &Token::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    } else {
                        return Err("Expected key identifier".to_string());
                    }
                }

                self.expect(Token::RightBrace)?;
                Ok(Expression::Dict(pairs))
            }
            _ => Err(format!(
                "Unexpected token in expression: {:?}",
                self.current_token()
            )),
        }
    }

    // v2.0 parsing methods
    fn parse_const_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'const'

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            // Optional type annotation
            let type_annotation = if self.current_token() == &Token::Colon {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            self.expect(Token::Equal)?;
            let mut value = self.parse_expression()?;
            self.skip_newlines();

            // If we have a type annotation, wrap the expression
            if let Some(ty) = type_annotation {
                value = Expression::Typed {
                    expr: Box::new(value),
                    ty,
                };
            }

            Ok(Statement::Definition {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after 'const'".to_string())
        }
    }

    fn parse_mut_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'mut'

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            // Optional type annotation
            let type_annotation = if self.current_token() == &Token::Colon {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            self.expect(Token::Equal)?;
            let mut value = self.parse_expression()?;
            self.skip_newlines();

            // If we have a type annotation, wrap the expression
            if let Some(ty) = type_annotation {
                value = Expression::Typed {
                    expr: Box::new(value),
                    ty,
                };
            }

            Ok(Statement::Definition {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after 'mut'".to_string())
        }
    }

    fn parse_extern_block(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'extern'

        // Parse language specifier
        let language = if let Token::Identifier(lang) = self.current_token() {
            let lang_name = lang.clone();
            self.advance();
            lang_name
        } else {
            return Err("Expected language identifier after 'extern'".to_string());
        };

        // Parse block content - for now, expect a simple block
        self.expect(Token::LeftBrace)?;

        // For v2.0, we'll parse function definitions within extern blocks
        // For now, collect everything until closing brace
        let mut code = String::new();
        let mut brace_depth = 1;

        while self.current_token() != &Token::Eof && brace_depth > 0 {
            match self.current_token() {
                Token::LeftBrace => {
                    brace_depth += 1;
                    code.push('{');
                    self.advance();
                }
                Token::RightBrace => {
                    brace_depth -= 1;
                    if brace_depth > 0 {
                        code.push('}');
                    }
                    self.advance();
                }
                Token::Newline => {
                    code.push('\n');
                    self.advance();
                }
                Token::String(s) => {
                    code.push('"');
                    code.push_str(s);
                    code.push('"');
                    self.advance();
                }
                Token::Identifier(id) => {
                    code.push_str(id);
                    self.advance();
                }
                _ => {
                    // For other tokens, just add their string representation
                    code.push_str(&format!("{:?}", self.current_token()).to_lowercase());
                    self.advance();
                }
            }
        }

        Ok(Statement::ForeignBlock {
            language,
            code: code.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_definition() {
        let mut lexer = Lexer::new("def x = 10");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, .. } => assert_eq!(name, "x"),
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_parse_expression() {
        let mut lexer = Lexer::new("def result = 2 + 3 * 4");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_function() {
        let mut lexer = Lexer::new("fn add(a, b):\n    return a + b");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Function { name, params, .. } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
            }
            _ => panic!("Expected function"),
        }
    }

    #[test]
    fn test_parse_async_function() {
        let mut lexer =
            Lexer::new("asy fetch(url):\n    res = await http.get(url)\n    return res.text()");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Function { name, is_async, .. } => {
                assert_eq!(name, "fetch");
                assert!(*is_async);
            }
            _ => panic!("Expected async function"),
        }
    }

    #[test]
    fn test_parse_ownership_in_parameters() {
        let mut lexer = Lexer::new("fn process(own data, ref config):\n    result = data + config.value\n    return result");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Function { name, params, .. } => {
                assert_eq!(name, "process");
                assert_eq!(params.len(), 2);
                // Note: ownership parsing would need to be implemented in the AST
            }
            _ => panic!("Expected function with ownership parameters"),
        }
    }

    #[test]
    fn test_parse_ui_sprite_expression() {
        let mut lexer = Lexer::new("def slider = ^&^[slider{min=0, max=100, value=50}]");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "slider");
                // Check if the value is a UiSprite expression
                match value {
                    Expression::UiSprite { .. } => {} // Success
                    _ => panic!("Expected UiSprite expression"),
                }
            }
            _ => panic!("Expected definition with UI sprite"),
        }
    }

    #[test]
    fn test_parse_custom_block() {
        let mut lexer = Lexer::new("cs rust:\n    let x = 5");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::ForeignBlock { language, .. } => {
                assert_eq!(language, "rust");
            }
            _ => panic!("Expected custom block"),
        }
    }

    #[test]
    fn test_parse_if_elif_else() {
        let mut lexer =
            Lexer::new("if x > 0:\n    return x\nelif x < 0:\n    return -x\nelse:\n    return 0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::If {
                then_body,
                else_body,
                ..
            } => {
                assert_eq!(then_body.len(), 1);
                assert!(else_body.is_some());
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn test_parse_for_loop() {
        let mut lexer = Lexer::new("for item in collection:\n    process(item)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::For { variable, .. } => {
                assert_eq!(variable, "item");
            }
            _ => panic!("Expected for loop"),
        }
    }

    #[test]
    fn test_parse_while_loop() {
        let mut lexer = Lexer::new("while condition:\n    update()");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::While { .. } => {
                // Success
            }
            _ => panic!("Expected while loop"),
        }
    }

    #[test]
    fn test_parse_main_function() {
        let mut lexer = Lexer::new("mn main():\n    print(\"Hello, World!\")\n    ui.print(^&^[tree])\n    data = await fetch(\"https://api.example.com\")\n    print(data)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Main { .. } => {
                // Success
            }
            _ => panic!("Expected main function"),
        }
    }

    #[test]
    fn test_parse_import_statement() {
        // Test simple import (just module name, not dotted path)
        let mut lexer = Lexer::new("imp std");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Import(module) => {
                assert_eq!(module, "std");
            }
            _ => panic!("Expected import statement"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        let mut lexer = Lexer::new("def result = (a + b) * c / d - e");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        // The expression parsing should handle operator precedence correctly
    }

    #[test]
    fn test_parse_nested_function_calls() {
        let mut lexer = Lexer::new("def result = max(min(a, b), c)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        // Should parse nested function calls correctly
    }
}
