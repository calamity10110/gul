// Type annotation parsing methods for Parser

use crate::ast::{Type, *};
use crate::lexer::Token;
use super::Parser;

impl Parser {
    pub(crate) fn parse_type(&mut self) -> Result<Type, String> {
        // Handle optional @ prefix (v3.2)
        if self.current_token() == &Token::At {
            self.advance();
        }

        match self.current_token() {
            Token::Identifier(name) => {
                let type_name = name.clone();
                self.advance();

                match type_name.as_str() {
                    "int" => Ok(Type::Int),
                    "float" | "flt" => Ok(Type::Float),
                    "str" => Ok(Type::String),
                    "bool" => Ok(Type::Bool),
                    "any" => Ok(Type::Any),
                    "list" => {
                        if self.current_token() == &Token::Less {
                             self.advance();
                             let inner = self.parse_type()?;
                             self.expect(Token::Greater)?;
                             Ok(Type::List(Box::new(inner)))
                        } else {
                             Ok(Type::List(Box::new(Type::Any)))
                        }
                    }
                    "dict" => Ok(Type::Dict(Box::new(Type::Any), Box::new(Type::Any))),
                    _unit_name => {
                        // Check for unit types
                        if let Token::Unit(unit) = self.current_token() {
                            let unit_name = unit.clone();
                            self.advance();
                            Ok(Type::Unit(unit_name))
                        } else {
                            // Treat as custom/struct type?
                            Ok(Type::Any)
                        }
                    }
                }
            }
            _ => Err(format!("Expected type name, found {:?}", self.current_token())),
        }
    }

    pub(crate) fn parse_type_annotation(&mut self) -> Result<String, String> {
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
}
