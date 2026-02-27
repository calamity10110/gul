#![allow(unused_mut)]
// Parser module - builds AST from tokens

mod expressions;
mod statements;
mod types;

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

    pub(crate) fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    pub(crate) fn peek(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.position + offset)
            .unwrap_or(&Token::Eof)
    }

    pub(crate) fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    pub(crate) fn expect(&mut self, expected: Token) -> Result<(), String> {
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

    pub(crate) fn skip_newlines(&mut self) {
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
            Statement::Function { name, is_async, params, outputs, .. } => {
                let p_str = params.iter().map(|p| p.0.clone()).collect::<Vec<_>>().join(", ");
                let o_str = if outputs.is_empty() { "".to_string() } else { format!("({})", outputs.iter().map(|o| o.0.clone()).collect::<Vec<_>>().join(", ")) };
                format!("{}fn {}({}) {} {{ ... }}", if *is_async { "async " } else { "" }, name, p_str, o_str);
            }
            _ => panic!("Expected async function"),
        }
    }

    #[test]
    #[ignore] // Ownership keywords in parameters are deprecated in v3.2
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
        // v3 syntax: just `mn:` without main() or parentheses
        let mut lexer = Lexer::new("mn:\n    print(\"Hello, World!\")\n    ui.print(^&^[tree])\n    data = await fetch(\"https://api.example.com\")\n    print(data)");
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
            Statement::Import(modules) => {
                assert_eq!(modules.len(), 1);
                assert_eq!(modules[0], "std");
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

    // v2.1 Bracket Equivalence Parser Tests
    #[test]
    fn test_v21_function_call_with_brackets() {
        // Function call using [] instead of ()
        let mut lexer = Lexer::new("def result = print[\"hello\"]");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "result");
                match value {
                    Expression::Call { function: _, args } => {
                        assert_eq!(args.len(), 1);
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_function_call_with_braces() {
        // Function call using {} instead of ()
        let mut lexer = Lexer::new("def result = print{\"hello\"}");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "result");
                match value {
                    Expression::Call { function: _, args } => {
                        assert_eq!(args.len(), 1);
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_list_with_parens() {
        // List using () instead of []
        let mut lexer = Lexer::new("def data = (1, 2, 3)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "data");
                match value {
                    Expression::List(elements) => {
                        assert_eq!(elements.len(), 3);
                    }
                    _ => panic!("Expected list"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_list_with_braces() {
        // List using {} instead of []
        let mut lexer = Lexer::new("def data = {1, 2, 3}");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "data");
                match value {
                    Expression::List(elements) => {
                        assert_eq!(elements.len(), 3);
                    }
                    _ => panic!("Expected list"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_dict_with_brackets() {
        // Dict using [] instead of {}
        let mut lexer = Lexer::new("def config = [host: \"localhost\", port: 8080]");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "config");
                match value {
                    Expression::Dict(pairs) => {
                        assert_eq!(pairs.len(), 2);
                        assert_eq!(pairs[0].0, "host");
                        assert_eq!(pairs[1].0, "port");
                    }
                    _ => panic!("Expected dict"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_empty_collections() {
        // Empty list with different brackets
        let mut lexer = Lexer::new("def empty = []");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::Definition { value, .. } => match value {
                Expression::List(elements) => assert_eq!(elements.len(), 0),
                _ => panic!("Expected empty list"),
            },
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_mixed_bracket_nesting() {
        // Nested with different bracket types
        let mut lexer = Lexer::new("def x = func{a, [1, 2], (3, 4)}");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "x");
                match value {
                    Expression::Call { args, .. } => {
                        assert_eq!(args.len(), 3);
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }
}
