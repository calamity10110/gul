// Parser module - builds AST from tokens

use crate::ast::*;
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
        while self.current_token() == &Token::Newline {
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

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token() {
            Token::Imp => self.parse_import(),
            Token::Def => self.parse_definition(),
            Token::Fn => self.parse_function(false),
            Token::Asy => self.parse_function(true),
            Token::Cs => self.parse_custom_block(),
            Token::Mn => self.parse_main(),
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
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_import(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'imp'

        if let Token::Identifier(module) = self.current_token() {
            let module_name = module.clone();
            self.advance();
            self.skip_newlines();
            Ok(Statement::Import(module_name))
        } else {
            Err("Expected module name after 'imp'".to_string())
        }
    }

    fn parse_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'def'

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

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

        // Simple block parsing - collect statements until we hit a dedent or EOF
        // For now, we'll parse until we see a top-level keyword or EOF
        loop {
            self.skip_newlines();

            match self.current_token() {
                Token::Eof => break,
                Token::Def | Token::Fn | Token::Asy | Token::Cs | Token::Mn => break,
                _ => {
                    // Check if this looks like end of block (next line is dedented)
                    // For now, just parse the statement
                    if self.is_block_end() {
                        break;
                    }
                    statements.push(self.parse_statement()?);
                }
            }
        }

        Ok(statements)
    }

    fn is_block_end(&self) -> bool {
        // Simple heuristic: if we see certain keywords at the start, it's likely a new block
        matches!(self.current_token(), Token::Elif | Token::Else | Token::Eof)
    }

    fn parse_custom_block(&mut self) -> Result<Statement, String> {
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
                    Token::Def | Token::Fn | Token::Asy | Token::Cs | Token::Mn | Token::Imp
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
                        Token::Def | Token::Fn | Token::Asy | Token::Cs | Token::Mn | Token::Imp
                    ) {
                        break;
                    }
                }
            }

            Ok(Statement::CustomBlock {
                language,
                code: code_lines.join("\n"),
            })
        } else {
            Err("Expected language name after 'cs'".to_string())
        }
    }

    fn parse_main(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'mn'

        if let Token::Identifier(name) = self.current_token() {
            if name != "main" {
                return Err("Expected 'main' after 'mn'".to_string());
            }
            self.advance();

            self.expect(Token::LeftParen)?;
            self.expect(Token::RightParen)?;
            self.expect(Token::Colon)?;
            self.skip_newlines();

            let body = self.parse_block()?;

            Ok(Statement::Main { body })
        } else {
            Err("Expected 'main' after 'mn'".to_string())
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

                // Parse sprite content (simplified)
                // Format: sprite_type{prop1=val1, prop2=val2} or just sprite_type
                let parts: Vec<&str> = sprite_content.splitn(2, '{').collect();
                let sprite_type = parts[0].to_string();
                let properties = Vec::new(); // TODO: Parse properties

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
                        break;
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
        let mut lexer = Lexer::new("asy fetch(url):\n    res = await http.get(url)\n    return res.text()");
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
        let mut lexer = Lexer::new("def slider = ^÷^[slider{min=0, max=100, value=50}]");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "slider");
                // Check if the value is a UiSprite expression
                match value {
                    Expression::UiSprite { .. } => {}, // Success
                    _ => panic!("Expected UiSprite expression"),
                }
            }
            _ => panic!("Expected definition with UI sprite"),
        }
    }

    #[test]
    fn test_parse_custom_block() {
        let mut lexer = Lexer::new("cs rust:\n    fn sum(a: i32, b: i32) -> i32 {\n        a + b\n    }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::CustomBlock { language, .. } => {
                assert_eq!(language, "rust");
            }
            _ => panic!("Expected custom block"),
        }
    }

    #[test]
    fn test_parse_if_elif_else() {
        let mut lexer = Lexer::new("if x > 0:\n    return x\nelif x < 0:\n    return -x\nelse:\n    return 0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::If { then_body, else_body, .. } => {
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
        let mut lexer = Lexer::new("mn main():\n    print(\"Hello, World!\")\n    ui.print(^÷^[tree])\n    data = await fetch(\"https://api.example.com\")\n    print(data)");
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
        let mut lexer = Lexer::new("imp std.io");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Import(module) => {
                assert_eq!(module, "std.io");
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
