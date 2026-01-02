// Enhanced Parser with indentation handling and full expression support

use crate::lexer::{Token, TokenType};
use crate::ast::*;
use anyhow::{anyhow, Result};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            self.skip_newlines();
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }
        
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>> {
        self.skip_newlines();
        
        if self.is_at_end() {
            return Ok(None);
        }
        
        match &self.peek().token_type {
            TokenType::Let => Ok(Some(self.parse_let()?)),
            TokenType::Var => Ok(Some(self.parse_var()?)),
            TokenType::Fn => Ok(Some(self.parse_function()?)),
            TokenType::Return => Ok(Some(self.parse_return()?)),
            TokenType::If => Ok(Some(self.parse_if()?)),
            TokenType::While => Ok(Some(self.parse_while()?)),
            TokenType::Loop => Ok(Some(self.parse_loop()?)),
            TokenType::Break => Ok(Some(self.parse_break()?)),
            TokenType::Continue => Ok(Some(self.parse_continue()?)),
            TokenType::Match => Ok(Some(self.parse_match()?)),
            _ => Ok(Some(self.parse_expression_or_assignment()?)),
        }
    }

    fn parse_match(&mut self) -> Result<Statement> {
        self.advance(); // consume 'match'
        let value = self.parse_expression()?;
        self.expect(TokenType::Colon)?;
        self.skip_newlines();
        
        // Expect block indent
        self.expect(TokenType::Indent)?;
        
        let mut arms = Vec::new();
        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenType::Dedent) {
                break;
            }
            
            let pattern = self.parse_pattern()?;
            self.expect(TokenType::FatArrow)?;
             
            // Check if block or inline
            let body = if self.check(&TokenType::Newline) {
                self.advance(); // consume newline
                self.parse_block()?
            } else if self.check(&TokenType::Indent) {
                self.parse_block()?
            } else {
                if let Some(stmt) = self.parse_statement()? {
                     vec![stmt]
                } else {
                     return Err(anyhow!("Expected statement in match arm"));
                }
            };
            
            arms.push(MatchArm { pattern, body });
        }
        
        self.expect(TokenType::Dedent)?;
        
        Ok(Statement::Match { value, arms })
    }

    fn parse_pattern(&mut self) -> Result<Pattern> {
        match self.peek().token_type {
            TokenType::Integer => {
                let val = self.advance().lexeme.parse().unwrap();
                Ok(Pattern::Literal(ExpressionKind::Integer(val)))
            }
            TokenType::Boolean => {
                 let val = self.advance().lexeme == "true";
                 Ok(Pattern::Literal(ExpressionKind::Boolean(val)))
            }
            TokenType::String => {
                let val = self.advance().lexeme.clone();
                Ok(Pattern::Literal(ExpressionKind::String(val)))
            }
            TokenType::Identifier => {
                let name = self.advance().lexeme.clone();
                if name == "_" {
                    Ok(Pattern::Wildcard)
                } else {
                    Ok(Pattern::Identifier(name))
                }
            }
            _ => Err(anyhow!("Unexpected token in pattern: {:?}", self.peek())),
        }
    }

    fn parse_let(&mut self) -> Result<Statement> {
        self.advance(); // consume 'let'
        let name = self.expect_identifier()?;
        self.expect(TokenType::Equal)?;
        let value = self.parse_expression()?;
        self.skip_newlines();
        Ok(Statement::Let { name, value })
    }

    fn parse_var(&mut self) -> Result<Statement> {
        self.advance(); // consume 'var'
        let name = self.expect_identifier()?;
        self.expect(TokenType::Equal)?;
        let value = self.parse_expression()?;
        self.skip_newlines();
        Ok(Statement::Var { name, value })
    }

    fn parse_function(&mut self) -> Result<Statement> {
        self.advance(); // consume 'fn'
        let name = self.expect_identifier()?;
        self.expect(TokenType::LeftParen)?;
        
        let mut params = Vec::new();
        while !self.check(&TokenType::RightParen) && !self.is_at_end() {
            params.push(self.expect_identifier()?);
            if !self.check(&TokenType::RightParen) {
                self.expect(TokenType::Comma)?;
            }
        }
        
        self.expect(TokenType::RightParen)?;
        self.expect(TokenType::Colon)?;
        self.skip_newlines();
        
        // Parse function body with indentation
        let body = self.parse_block()?;
        
        Ok(Statement::Function { name, params, body })
    }

    fn parse_if(&mut self) -> Result<Statement> {
        self.advance(); // consume 'if'
        let condition = self.parse_expression()?;
        self.expect(TokenType::Colon)?;
        self.skip_newlines();
        
        let then_branch = self.parse_block()?;
        let mut else_branch = None;
        
        // Handle else/elif
        self.skip_newlines();
        if self.check(&TokenType::Else) {
            self.advance(); // consume 'else'
            self.expect(TokenType::Colon)?;
            self.skip_newlines();
            else_branch = Some(self.parse_block()?);
        } else if self.check(&TokenType::Elif) {
            self.advance(); // consume 'elif'
            let elif_condition = self.parse_expression()?;
            self.expect(TokenType::Colon)?;
            self.skip_newlines();
            let elif_then = self.parse_block()?;
            
            let else_stmt = Statement::If {
                condition: elif_condition,
                then_branch: elif_then,
                else_branch: self.parse_elif_continuation()?,
            };
            else_branch = Some(vec![else_stmt]);
        }
        
        Ok(Statement::If { condition, then_branch, else_branch })
    }
    
    // Helper for elif chains
    fn parse_elif_continuation(&mut self) -> Result<Option<Vec<Statement>>> {
        self.skip_newlines();
        if self.check(&TokenType::Else) {
            self.advance();
            self.expect(TokenType::Colon)?;
            self.skip_newlines();
            return Ok(Some(self.parse_block()?));
        } else if self.check(&TokenType::Elif) {
            self.advance();
             let condition = self.parse_expression()?;
            self.expect(TokenType::Colon)?;
            self.skip_newlines();
            let then_branch = self.parse_block()?;
            let else_branch = self.parse_elif_continuation()?;
            
            let stmt = Statement::If { condition, then_branch, else_branch };
            return Ok(Some(vec![stmt]));
        }
        Ok(None)
    }

    fn parse_while(&mut self) -> Result<Statement> {
        self.advance(); // consume 'while'
        let condition = self.parse_expression()?;
        self.expect(TokenType::Colon)?;
        self.skip_newlines();
        let body = self.parse_block()?;
        Ok(Statement::While { condition, body })
    }

    fn parse_loop(&mut self) -> Result<Statement> {
        self.advance(); // consume 'loop'
        self.expect(TokenType::Colon)?;
        self.skip_newlines();
        let body = self.parse_block()?;
        Ok(Statement::Loop { body })
    }

    fn parse_break(&mut self) -> Result<Statement> {
        self.advance(); // consume 'break'
        self.skip_newlines();
        Ok(Statement::Break)
    }

    fn parse_continue(&mut self) -> Result<Statement> {
        self.advance(); // consume 'continue'
        self.skip_newlines();
        Ok(Statement::Continue)
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        
        // Expect INDENT token
        if self.check(&TokenType::Indent) {
            self.advance();
        } else {
             self.skip_newlines();
             if !self.check(&TokenType::Indent) {
                 return Ok(statements); 
             }
             self.advance(); // consume indent we just checked
        }
        
        // Parse statements until DEDENT
        while !self.check(&TokenType::Dedent) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenType::Dedent) {
                break;
            }
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }
        
        // Consume DEDENT token
        if self.check(&TokenType::Dedent) {
            self.advance();
        }
        
        Ok(statements)
    }

    fn parse_return(&mut self) -> Result<Statement> {
        self.advance(); // consume 'return'
        let value = if self.check(&TokenType::Newline) || self.is_at_end() || self.check(&TokenType::Dedent) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.skip_newlines();
        Ok(Statement::Return { value })
    }

    fn parse_expression_or_assignment(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;
        
        if self.check(&TokenType::Equal) {
            self.advance(); // consume '='
            let value = self.parse_expression()?;
            self.skip_newlines();
            
            return match expr.kind {
                ExpressionKind::Identifier(name) => Ok(Statement::Assignment { name, value }),
                ExpressionKind::Index { target, index } => Ok(Statement::SetIndex { 
                    target: *target, 
                    index: *index, 
                    value 
                }),
                _ => Err(anyhow!("Invalid assignment target: {:?}", expr)),
            }
        }

        self.skip_newlines();
        Ok(Statement::Expression { expr })
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut left = self.parse_additive()?;
        
        while matches!(
            self.peek().token_type,
            TokenType::EqualEqual
                | TokenType::NotEqual
                | TokenType::Less
                | TokenType::LessEq
                | TokenType::Greater
                | TokenType::GreaterEq
        ) {
            let op_token = self.advance();
            let op = match op_token.token_type {
                TokenType::EqualEqual => BinaryOperator::Equal,
                TokenType::NotEqual => BinaryOperator::NotEqual,
                TokenType::Less => BinaryOperator::Less,
                TokenType::LessEq => BinaryOperator::LessEq,
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEq => BinaryOperator::GreaterEq,
                _ => BinaryOperator::Equal,
            };
            
            let right = self.parse_additive()?;
            left = Expression {
                kind: ExpressionKind::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                },
                ty: Type::Unknown
            };
        }
        
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplicative()?;
        
        while matches!(self.peek().token_type, TokenType::Plus | TokenType::Minus) {
            let op_token = self.advance();
            let op = match op_token.token_type {
                TokenType::Plus => BinaryOperator::Add,
                TokenType::Minus => BinaryOperator::Subtract,
                _ => BinaryOperator::Add,
            };
            let right = self.parse_multiplicative()?;
            left = Expression { 
                kind: ExpressionKind::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                },
                ty: Type::Unknown
            };
        }
        
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression> {
        let mut left = self.parse_call()?;
        
        while matches!(
            self.peek().token_type,
            TokenType::Star | TokenType::Slash | TokenType::Percent
        ) {
            let op_token = self.advance();
            let op = match op_token.token_type {
                TokenType::Star => BinaryOperator::Multiply,
                TokenType::Slash => BinaryOperator::Divide,
                _ => BinaryOperator::Multiply,
            };
            let right = self.parse_call()?;
            left = Expression { 
                kind: ExpressionKind::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                },
                ty: Type::Unknown 
            };
        }
        
        Ok(left)
    }

    fn parse_call(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.check(&TokenType::LeftParen) {
                self.advance();
                let mut args = Vec::new();
                while !self.check(&TokenType::RightParen) && !self.is_at_end() {
                    args.push(self.parse_expression()?);
                    if !self.check(&TokenType::RightParen) {
                        self.expect(TokenType::Comma)?;
                    }
                }
                self.expect(TokenType::RightParen)?;
                expr = Expression {
                    kind: ExpressionKind::Call {
                        callee: Box::new(expr),
                        args,
                    },
                    ty: Type::Unknown
                };
            } else if self.check(&TokenType::LeftBracket) {
                self.advance();
                let index = self.parse_expression()?;
                self.expect(TokenType::RightBracket)?;
                expr = Expression {
                    kind: ExpressionKind::Index {
                        target: Box::new(expr),
                        index: Box::new(index),
                    },
                    ty: Type::Unknown
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        match &self.peek().token_type {
            TokenType::Integer => {
                let value = self.advance().lexeme.parse()
                    .map_err(|e| anyhow!("Failed to parse integer: {}", e))?;
                Ok(Expression { kind: ExpressionKind::Integer(value), ty: Type::Integer })
            }
            TokenType::Float => {
                let value = self.advance().lexeme.parse()
                    .map_err(|e| anyhow!("Failed to parse float: {}", e))?;
                Ok(Expression { kind: ExpressionKind::Float(value), ty: Type::Float })
            }
            TokenType::String => {
                let value = self.advance().lexeme.clone();
                Ok(Expression { kind: ExpressionKind::String(value), ty: Type::String })
            }
            TokenType::FString => {
                let text = self.advance().lexeme.clone();
                self.parse_f_string(text)
            }
            TokenType::Boolean => {
                let value = self.advance().lexeme == "true";
                Ok(Expression { kind: ExpressionKind::Boolean(value), ty: Type::Boolean })
            }
            TokenType::Identifier => {
                let name = self.advance().lexeme.clone();
                Ok(Expression { kind: ExpressionKind::Identifier(name), ty: Type::Unknown })
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut items = Vec::new();
                while !self.check(&TokenType::RightBracket) && !self.is_at_end() {
                    items.push(self.parse_expression()?);
                    if !self.check(&TokenType::RightBracket) {
                        self.expect(TokenType::Comma)?;
                    }
                }
                self.expect(TokenType::RightBracket)?;
                Ok(Expression { kind: ExpressionKind::List(items), ty: Type::Unknown })
            }
            _ => Err(anyhow!("Unexpected token in expression: {:?}", self.peek())),
        }
    }

    fn parse_f_string(&mut self, text: String) -> Result<Expression> {
        let mut components = Vec::new();
        let mut last_pos = 0;
        let chars: Vec<char> = text.chars().collect();
        
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '{' {
                // Push previous text
                if i > last_pos {
                    let s: String = chars[last_pos..i].iter().collect();
                    components.push(Expression{ kind: ExpressionKind::String(s), ty: Type::String });
                }
                
                // Find closing }
                let start = i + 1;
                i += 1;
                while i < chars.len() && chars[i] != '}' {
                    i += 1;
                }
                if i >= chars.len() {
                    return Err(anyhow!("Unclosed brace in f-string"));
                }
                
                let expr_text: String = chars[start..i].iter().collect();
                
                // Recursive parse
                let mut lexer = crate::lexer::Lexer::new(&expr_text);
                let tokens = lexer.tokenize()?;
                let mut parser = Parser::new(tokens);
                let expr = parser.parse_expression()?;
                components.push(expr);
                
                last_pos = i + 1;
            }
            i += 1;
        }
        
        if last_pos < chars.len() {
            let s: String = chars[last_pos..].iter().collect();
            components.push(Expression{ kind: ExpressionKind::String(s), ty: Type::String });
        }
        
        let mut expr = Expression { 
             kind: ExpressionKind::String("".to_string()), 
             ty: Type::String 
        };
        
        for component in components {
            expr = Expression {
                kind: ExpressionKind::BinaryOp {
                    left: Box::new(expr),
                    op: BinaryOperator::Add,
                    right: Box::new(component),
                },
                ty: Type::String,
            };
        }
        
        Ok(expr)
    }

    fn expect(&mut self, token_type: TokenType) -> Result<()> {
        if self.check(&token_type) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow!(
                "Expected {:?}, got {:?} at line {}",
                token_type,
                self.peek().token_type,
                self.peek().line
            ))
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        if self.check(&TokenType::Identifier) {
            Ok(self.advance().lexeme.clone())
        } else {
            Err(anyhow!("Expected identifier, got {:?}", self.peek()))
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_at_end()
            && std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn skip_newlines(&mut self) {
        while !self.is_at_end()
            && matches!(
                self.peek().token_type,
                TokenType::Newline | TokenType::Indent | TokenType::Dedent
            )
        {
            if matches!(self.peek().token_type, TokenType::Indent | TokenType::Dedent) {
                break;
            }
            self.advance();
        }
    }
}
