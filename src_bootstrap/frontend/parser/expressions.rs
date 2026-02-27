// Expression parsing methods for Parser

use crate::frontend::ast::*;
use crate::frontend::lexer::Token;
use super::Parser;

impl Parser {
    pub(crate) fn parse_paren_or_lambda(&mut self) -> Result<Statement, String> {
        // We are at '('. Check if it's a lambda or expression.
        // Simplified lookahead: (x, y) => ...
        // If we see '=>' effectively after closing paren, it's a lambda.
        // This is tricky with simple lookahead.
        // Let's assume for now if we parse parens and see '=>', it turns into lambda?
        // But expression parsing consumes tokens.
        // We can try to parse as lambda first if it looks like params?
        // Or just enhance parse_expression to handle it.
        // Actually, lambda is an Expression, not a Statement (usually).
        // But if it's top level statement? " (x) => 2 " is a valid statement expression.

        let expr = self.parse_expression()?;
        self.skip_newlines();
        Ok(Statement::Expression(expr))
    }

    // Updated expression parsing to include Lambdas
    pub(crate) fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_lambda_or_logical_or()
    }

    fn parse_lambda_or_logical_or(&mut self) -> Result<Expression, String> {
        // Parse left-hand side (potential params)
        let expr = self.parse_logical_or()?;

        // Check for FatArrow
        if self.current_token() == &Token::FatArrow {
            self.advance(); // consume '=>'

            let params = self.extract_params_from_expr(&expr)?;
            let body = self.parse_expression()?; // Recursively parse body

            return Ok(Expression::Lambda {
                params,
                body: Box::new(body),
            });
        }

        Ok(expr)
    }

    fn extract_params_from_expr(&self, expr: &Expression) -> Result<Vec<String>, String> {
        match expr {
            Expression::Identifier(name) => Ok(vec![name.clone()]),
            Expression::List(elements) => {
                let mut params = Vec::new();
                for e in elements {
                    if let Expression::Identifier(name) = e {
                        params.push(name.clone());
                    } else {
                        return Err("Lambda parameters must be identifiers".to_string());
                    }
                }
                Ok(params)
            }
             // Handle (param) parenthesized which might return grouping logic if applicable
             // If parse_primary groups (x) as x (identifier), it falls into first match.
             // If (a,b) is generic List, it falls into second match.
            _ => Err("Invalid lambda parameters".to_string()),
        }
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
            Token::Await => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Await(Box::new(operand)))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current_token() {
                // v2.1: All bracket types can be used for function calls
                Token::LeftParen | Token::LeftBracket | Token::LeftBrace => {
                    let open_bracket = self.current_token().clone();

                    // Determine if this is a function call or index access
                    // For identifiers followed by brackets, treat as function call
                    let is_function_call =
                        matches!(expr, Expression::Identifier(_) | Expression::Member { .. });

                    if is_function_call || open_bracket == Token::LeftParen {
                        // Function call with any bracket type
                        self.advance();
                        let mut args = Vec::new();

                        while !self.current_token().is_close_bracket() {
                            // Check for named parameters (key=value or key:value)
                            args.push(self.parse_expression()?);

                            if self.current_token() == &Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }

                        // v2.1: Validate bracket matching
                        if !open_bracket.brackets_match(self.current_token()) {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        self.advance(); // consume closing bracket

                        expr = Expression::Call {
                            function: Box::new(expr),
                            args,
                        };
                    } else {
                        // Index access (only for LeftBracket in non-function context)
                        self.advance();
                        let index = self.parse_expression()?;

                        // v2.1: Accept any matching closing bracket
                        if !open_bracket.brackets_match(self.current_token()) {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        self.advance();

                        expr = Expression::Index {
                            object: Box::new(expr),
                            index: Box::new(index),
                        };
                    }
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
            Token::Main | Token::Mn => {
                self.advance();
                Ok(Expression::Identifier("main".to_string()))
            }
            Token::UiSprite(content) => {
                let sprite_content = content.clone();
                self.advance();

                // Parser for UI components (simplified)
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
            Token::At => {
                self.advance();
                if let Token::Identifier(name) = self.current_token() {
                    let id = format!("@{}", name);
                    self.advance();
                    Ok(Expression::Identifier(id))
                } else {
                    Err("Expected built-in name after '@'".to_string())
                }
            }
            // v2.1: All bracket types can be used for grouping or collections
            Token::LeftParen | Token::LeftBracket | Token::LeftBrace => {
                let open_bracket = self.current_token().clone();
                self.advance();

                // Check if empty
                if open_bracket.brackets_match(self.current_token()) {
                    self.advance();
                    return Ok(Expression::List(Vec::new()));
                }

                // Parse first element
                let first_expr = self.parse_expression()?;

                // Check what follows to determine type
                match self.current_token() {
                    // Colon after first element = dictionary
                    Token::Colon => {
                        self.advance();
                        let first_value = self.parse_expression()?;
                        let mut pairs = vec![];

                        // Extract key from first_expr
                        let key = match first_expr {
                            Expression::Identifier(k) => k,
                            _ => return Err("Dictionary key must be identifier".to_string()),
                        };
                        pairs.push((key, first_value));

                        // Parse remaining pairs
                        while self.current_token() == &Token::Comma {
                            self.advance();
                            if open_bracket.brackets_match(self.current_token()) {
                                break; // trailing comma
                            }
                            if let Token::Identifier(key) = self.current_token() {
                                let key_name = key.clone();
                                self.advance();
                                self.expect(Token::Colon)?;
                                let value = self.parse_expression()?;
                                pairs.push((key_name, value));
                            } else {
                                return Err("Expected key identifier in dict".to_string());
                            }
                        }

                        // v2.1: Validate bracket matching
                        if !open_bracket.brackets_match(self.current_token()) {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        self.advance();
                        Ok(Expression::Dict(pairs))
                    }
                    // Comma = list
                    Token::Comma => {
                        let mut elements = vec![first_expr];
                        while self.current_token() == &Token::Comma {
                            self.advance();
                            if open_bracket.brackets_match(self.current_token()) {
                                break; // trailing comma
                            }
                            elements.push(self.parse_expression()?);
                        }

                        // v2.1: Validate bracket matching
                        if !open_bracket.brackets_match(self.current_token()) {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        self.advance();
                        Ok(Expression::List(elements))
                    }
                    // Closing bracket = single element (grouping or single-element list)
                    _ if open_bracket.brackets_match(self.current_token()) => {
                        self.advance();
                        // For () this is grouping, for [] and {} this is single-element list
                        if open_bracket == Token::LeftParen {
                            Ok(first_expr) // Grouping
                        } else {
                            Ok(Expression::List(vec![first_expr])) // Single element list
                        }
                    }
                    _ => {
                        // v2.1: Check for mismatched brackets
                        if self.current_token().is_close_bracket() {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        Err(format!(
                            "Unexpected token in collection: {:?}",
                            self.current_token()
                        ))
                    }
                }
            }
            _ => Err(format!(
                "Unexpected token in expression: {:?}",
                self.current_token()
            )),
        }
    }
}
