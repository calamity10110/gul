// GUL Compiler - Complete Lexer Implementation
// Tokenizes GUL v3.2 source code

pub mod token;
pub use token::{get_keyword_type, Token, TokenType};

use anyhow::{anyhow, Result};

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
    indent_stack: Vec<usize>,
    pending_dedents: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
            indent_stack: vec![0],
            pending_dedents: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace_except_newline();
            
            if self.is_at_end() {
                break;
            }
            
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }
        
        // Emit remaining dedents
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.push(Token::new(TokenType::Dedent, String::new(), self.line, self.column));
        }
        
        tokens.push(Token::new(TokenType::Eof, String::new(), self.line, self.column));
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return Ok(Some(Token::new(TokenType::Dedent, String::new(), self.line, 1)));
        }

        let start_line = self.line;
        let start_column = self.column;
        
        let ch = self.peek();
        
        // Handle newlines and indentation
        if ch == '\n' {
            self.advance();
            let token = Token::new(TokenType::Newline, "\n".to_string(), start_line, start_column);
            
            // Check indentation on next line
            if !self.is_at_end() && self.peek() != '\n' {
                let indent_level = self.count_indentation();

                if self.is_at_end() || self.peek() == '\n' || self.peek() == '#' {
                    return Ok(None);
                }

                let current_indent = *self.indent_stack.last().unwrap();
                
                if indent_level > current_indent {
                    self.indent_stack.push(indent_level);
                    return Ok(Some(Token::new(TokenType::Indent, String::new(), self.line, 1)));
                } else if indent_level < current_indent {
                    let mut dedents = 0;
                    while self.indent_stack.len() > 1 && *self.indent_stack.last().unwrap() > indent_level {
                        self.indent_stack.pop();
                        dedents += 1;
                    }
                    if dedents > 0 {
                        self.pending_dedents = dedents - 1;
                        return Ok(Some(Token::new(TokenType::Dedent, String::new(), self.line, 1)));
                    }
                }
            }
            
            return Ok(Some(token));
        }
        
        // Comments
        if ch == '#' {
            self.skip_comment();
            return Ok(None);
        }
        
        // String literals
        if ch == '"' || ch == '\'' {
            return Ok(Some(self.scan_string()?));
        }
        
        // F-Strings
        if ch == 'f' && (self.peek_next() == '"' || self.peek_next() == '\'') {
             self.advance(); // consume 'f'
             let mut token = self.scan_string()?;
             token.token_type = TokenType::FString;
             return Ok(Some(token));
        }

        // Numbers
        if ch.is_ascii_digit() {
            return Ok(Some(self.scan_number()?));
        }
        
        // Identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            return Ok(Some(self.scan_identifier()));
        }
        
        // Type annotations starting with @
        if ch == '@' {
            return Ok(Some(self.scan_type_annotation_or_decorator()?));
        }
        
        // Operators and delimiters
        let token = match ch {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '[' => self.make_token(TokenType::LeftBracket),
            ']' => self.make_token(TokenType::RightBracket),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ',' => self.make_token(TokenType::Comma),
            ';' => self.make_token(TokenType::Semicolon),
            '?' => self.make_token(TokenType::Question),
            '$' => self.make_token(TokenType::Dollar),
            
            ':' => self.make_token(TokenType::Colon),
            '.' => self.make_token(TokenType::Dot),
            
            '+' => {
                if self.peek_next() == '=' {
                    self.advance();
                    self.make_token(TokenType::PlusEq)
                } else {
                    self.make_token(TokenType::Plus)
                }
            }
            
            '-' => {
                if self.peek_next() == '=' {
                    self.advance();
                    self.make_token(TokenType::MinusEq)
                } else if self.peek_next() == '>' {
                    self.advance();
                    self.make_token(TokenType::Arrow)
                } else {
                    self.make_token(TokenType::Minus)
                }
            }
            
            '*' => {
                if self.peek_next() == '*' {
                    self.advance();
                    self.make_token(TokenType::DoubleStar)
                } else if self.peek_next() == '=' {
                    self.advance();
                    self.make_token(TokenType::StarEq)
                } else {
                    self.make_token(TokenType::Star)
                }
            }
            
            '/' => {
                if self.peek_next() == '=' {
                    self.advance();
                    self.make_token(TokenType::SlashEq)
                } else {
                    self.make_token(TokenType::Slash)
                }
            }
            
            '%' => {
                if self.peek_next() == '=' {
                    self.advance();
                    self.make_token(TokenType::PercentEq)
                } else {
                    self.make_token(TokenType::Percent)
                }
            }
            
            '=' => {
                if self.peek_next() == '=' {
                    self.advance();
                    self.make_token(TokenType::EqualEqual)
                } else if self.peek_next() == '>' {
                    self.advance();
                    self.make_token(TokenType::FatArrow)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            
            '!' => {
                if self.peek_next() == '=' {
                    self.advance();
                    self.make_token(TokenType::NotEqual)
                } else {
                    return Err(anyhow!("Unexpected character '!' at {}:{}", start_line, start_column));
                }
            }
            
            '<' => {
                if self.peek_next() == '=' {
                    self.advance();
                    self.make_token(TokenType::LessEq)
                } else if self.peek_next() == '>' {
                    self.advance();
                    self.make_token(TokenType::LeftShift)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            
            '>' => {
                if self.peek_next() == '=' {
                    self.advance();
                    self.make_token(TokenType::GreaterEq)
               } else if self.peek_next() == '>' {
                    self.advance();
                    self.make_token(TokenType::RightShift)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            
            '&' => self.make_token(TokenType::BitwiseAnd),
            '|' => self.make_token(TokenType::BitwiseOr),
            '^' => self.make_token(TokenType::BitwiseXor),
            '~' => self.make_token(TokenType::BitwiseNot),
            
            _ => return Err(anyhow!("Unexpected character '{}' at {}:{}", ch, start_line, start_column)),
        };
        
        Ok(Some(token))
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let ch = self.advance();
        Token::new(token_type, ch.to_string(), self.line, self.column - 1)
    }

    fn scan_string(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let quote = self.advance();
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != quote {
            if self.peek() == '\\' {
                self.advance();
                if !self.is_at_end() {
                    let escaped = match self.peek() {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '\'' => '\'',
                        '"' => '"',
                        _ => self.peek(),
                    };
                    value.push(escaped);
                    self.advance();
                }
            } else {
                value.push(self.advance());
            }
        }
        
        if self.is_at_end() {
            return Err(anyhow!("Unterminated string at {}:{}", start_line, start_column));
        }
        
        self.advance(); // closing quote
        Ok(Token::new(TokenType::String, value, start_line, start_column))
    }

    fn scan_number(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let mut value = String::new();
        let mut is_float = false;
        
        while !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek() == '.') {
            if self.peek() == '.' {
                if is_float {
                    break; // Second dot, not part of number
                }
                is_float = true;
            }
            value.push(self.advance());
        }
        
        let token_type = if is_float {
            TokenType::Float
        } else {
            TokenType::Integer
        };
        
        Ok(Token::new(token_type, value, start_line, start_column))
    }

    fn scan_identifier(&mut self) -> Token {
        let start_line = self.line;
        let start_column = self.column;
        let mut value = String::new();
        
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            value.push(self.advance());
        }
        
        let token_type = get_keyword_type(&value).unwrap_or(TokenType::Identifier);
        Token::new(token_type, value, start_line, start_column)
    }

    fn scan_type_annotation_or_decorator(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        self.advance(); // consume @
        
        if self.is_at_end() || (!self.peek().is_alphabetic() && self.peek() != '_') {
            return Ok(Token::new(TokenType::At, "@".to_string(), start_line, start_column));
        }
        
        let mut value = String::from("@");
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            value.push(self.advance());
        }
        
        let token_type = match value.as_str() {
            "@int" => TokenType::AtInt,
            "@float" => TokenType::AtFloat,
            "@str" => TokenType::AtStr,
            "@bool" => TokenType::AtBool,
            "@list" => TokenType::AtList,
            "@dict" => TokenType::AtDict,
            "@set" => TokenType::AtSet,
            "@tuple" => TokenType::AtTuple,
            "@option" => TokenType::AtOption,
            "@box" => TokenType::AtBox,
            _ => TokenType::Decorator,
        };
        
        Ok(Token::new(token_type, value, start_line, start_column))
    }

    fn count_indentation(&mut self) -> usize {
        let mut count = 0;
        while !self.is_at_end() && (self.peek() == ' ' || self.peek() == '\t') {
            if self.peek() == '\t' {
                count += 4; // Tab = 4 spaces
            } else {
                count += 1;
            }
            self.advance();
        }
        count
    }

    fn skip_whitespace_except_newline(&mut self) {
        while !self.is_at_end() && (self.peek() == ' ' || self.peek() == '\t' || self.peek() == '\r') {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        
        ch
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("let x = 42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5); // let, x, =, 42, EOF
        assert_eq!(tokens[0].token_type, TokenType::Let);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::Integer);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new(r#""hello world""#);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].lexeme, "hello world");
    }
}
