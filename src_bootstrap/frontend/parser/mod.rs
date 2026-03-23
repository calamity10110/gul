#![allow(unused_mut)]
// Parser module - builds AST from tokens

mod expressions;
mod statements;
mod types;

use crate::frontend::ast::{Type, *};
use crate::frontend::lexer::Token;

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
mod tests;
