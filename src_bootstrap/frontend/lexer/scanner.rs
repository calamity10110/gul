// Lexer scanner - tokenizes source code into tokens

use super::tokens::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.first().copied();

        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut indent_stack: Vec<usize> = vec![0]; // Stack to track indentation levels
        let mut at_line_start = true; // Track if we're at the start of a line
        let mut current_indent = 0; // Current line's indentation

        while let Some(ch) = self.current_char {
            match ch {
                ' ' | '\t' => {
                    if at_line_start {
                        // Count indentation at line start
                        current_indent += if ch == '\t' { 4 } else { 1 };
                    }
                    self.advance();
                }
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                    at_line_start = true;
                    current_indent = 0;
                }
                '#' => {
                    if self.peek(1) == Some('[') {
                        self.skip_multiline_comment();
                    } else {
                        self.skip_comment();
                    }
                }
                _ => {
                    // Process indentation when we encounter first non-whitespace character
                    if at_line_start {
                        at_line_start = false;

                        // Compare current indentation with stack
                        let last_indent = *indent_stack.last().unwrap();

                        if current_indent > last_indent {
                            // Increased indentation - push INDENT
                            indent_stack.push(current_indent);
                            tokens.push(Token::Indent);
                        } else if current_indent < last_indent {
                            // Decreased indentation - pop DEDENT(s)
                            while let Some(&stack_indent) = indent_stack.last() {
                                if stack_indent <= current_indent {
                                    break;
                                }
                                indent_stack.pop();
                                tokens.push(Token::Dedent);
                            }

                            // Verify indentation matches a level in the stack
                            if indent_stack.last() != Some(&current_indent) {
                                // Indentation error - but we'll be lenient and adjust
                                indent_stack.push(current_indent);
                            }
                        }
                        // If current_indent == last_indent, no change needed
                    }

                    // Now process the actual character
                    match ch {
                        '"' => tokens.push(self.read_string()),
                        '0'..='9' => tokens.push(self.read_number()),
                        'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.read_identifier_or_unit()),
                        '^' => {
                            // Check for UI sprite syntax: ^&^[...]
                            if self.peek(1) == Some('&')
                                && self.peek(2) == Some('^')
                                && self.peek(3) == Some('[')
                            {
                                tokens.push(self.read_ui_sprite());
                            } else {
                                tokens.push(Token::Caret);
                                self.advance();
                            }
                        }
                        '+' => {
                            tokens.push(Token::Plus);
                            self.advance();
                        }
                        '-' => {
                            if self.peek(1) == Some('>') {
                                tokens.push(Token::Arrow);
                                self.advance();
                                self.advance();
                            } else {
                                tokens.push(Token::Minus);
                                self.advance();
                            }
                        }
                        '*' => {
                            tokens.push(Token::Star);
                            self.advance();
                        }
                        '/' => {
                            if self.peek(1) == Some('/') {
                                self.skip_comment();
                            } else {
                                tokens.push(Token::Slash);
                                self.advance();
                            }
                        }
                        '%' => {
                            tokens.push(Token::Percent);
                            self.advance();
                        }
                        '(' => {
                            tokens.push(Token::LeftParen);
                            self.advance();
                        }
                        ')' => {
                            tokens.push(Token::RightParen);
                            self.advance();
                        }
                        '[' => {
                            tokens.push(Token::LeftBracket);
                            self.advance();
                        }
                        ']' => {
                            tokens.push(Token::RightBracket);
                            self.advance();
                        }
                        '{' => {
                            tokens.push(Token::LeftBrace);
                            self.advance();
                        }
                        '}' => {
                            tokens.push(Token::RightBrace);
                            self.advance();
                        }
                        ',' => {
                            tokens.push(Token::Comma);
                            self.advance();
                        }
                        ':' => {
                            tokens.push(Token::Colon);
                            self.advance();
                        }
                        ';' => {
                            tokens.push(Token::Semicolon);
                            self.advance();
                        }
                        '.' => {
                            tokens.push(Token::Dot);
                            self.advance();
                        }
                        '=' => {
                            if self.peek(1) == Some('=') {
                                tokens.push(Token::EqualEqual);
                                self.advance();
                                self.advance();
                            } else if self.peek(1) == Some('>') {
                                tokens.push(Token::FatArrow);
                                self.advance();
                                self.advance();
                            } else {
                                tokens.push(Token::Equal);
                                self.advance();
                            }
                        }
                        '!' => {
                            if self.peek(1) == Some('=') {
                                tokens.push(Token::BangEqual);
                                self.advance();
                                self.advance();
                            } else {
                                tokens.push(Token::Not);
                                self.advance();
                            }
                        }
                        '<' => {
                            if self.peek(1) == Some('=') {
                                tokens.push(Token::LessEqual);
                                self.advance();
                                self.advance();
                            } else if self.peek(1) == Some('<') {
                                tokens.push(Token::LessLess);
                                self.advance();
                                self.advance();
                            } else {
                                tokens.push(Token::Less);
                                self.advance();
                            }
                        }
                        '>' => {
                            if self.peek(1) == Some('=') {
                                tokens.push(Token::GreaterEqual);
                                self.advance();
                                self.advance();
                            } else if self.peek(1) == Some('>') {
                                tokens.push(Token::GreaterGreater);
                                self.advance();
                                self.advance();
                            } else {
                                tokens.push(Token::Greater);
                                self.advance();
                            }
                        }
                        '&' => {
                            if self.peek(1) == Some('&') {
                                tokens.push(Token::And);
                                self.advance();
                                self.advance();
                            } else {
                                tokens.push(Token::Ampersand);
                                self.advance();
                            }
                        }
                        '|' => {
                            if self.peek(1) == Some('|') {
                                tokens.push(Token::Or);
                                self.advance();
                                self.advance();
                            } else {
                                tokens.push(Token::Pipe);
                                self.advance();
                            }
                        }
                        '?' => {
                            tokens.push(Token::QuestionMark);
                            self.advance();
                        }
                        '@' => {
                            tokens.push(Token::At);
                            self.advance();
                        }
                        _ => self.advance(),
                    }
                }
            }
        }

        // Add final DEDENT tokens for any remaining indentation
        while indent_stack.len() > 1 {
            indent_stack.pop();
            tokens.push(Token::Dedent);
        }

        tokens.push(Token::Eof);
        tokens
    }

    fn skip_comment(&mut self) {
        while self.current_char.is_some() && self.current_char != Some('\n') {
            self.advance();
        }
    }

    fn skip_multiline_comment(&mut self) {
        // Skip #[
        self.advance();
        self.advance();

        while let Some(ch) = self.current_char {
            if ch == ']' && self.peek(1) == Some('#') {
                self.advance(); // Skip ]
                self.advance(); // Skip #
                break;
            }
            self.advance();
        }
    }

    fn read_ui_sprite(&mut self) -> Token {
        // Skip ^&^[
        self.advance();
        self.advance();
        self.advance();
        self.advance();

        let mut content = String::new();
        let mut depth = 1;

        while let Some(ch) = self.current_char {
            if ch == '[' {
                depth += 1;
                content.push(ch);
                self.advance();
            } else if ch == ']' {
                depth -= 1;
                if depth == 0 {
                    self.advance();
                    break;
                }
                content.push(ch);
                self.advance();
            } else {
                content.push(ch);
                self.advance();
            }
        }

        Token::UiSprite(content)
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // Skip opening quote
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance();
                break;
            }
            value.push(ch);
            self.advance();
        }

        Token::String(value)
    }

    fn read_number(&mut self) -> Token {
        let mut value = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                is_float = true;
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::Float(value.parse().unwrap())
        } else {
            Token::Integer(value.parse().unwrap())
        }
    }

    fn read_identifier_or_unit(&mut self) -> Token {
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for keywords first - v2.0 keywords take priority
        let token = match value.as_str() {
            // v3.0 keywords (highest priority)
            "const" => Token::Let, // const is now the keyword for immutable (was let)
            "let" => {
                eprintln!("Warning: 'let' is deprecated, use 'const' instead");
                Token::Let
            }
            "var" => Token::Var,

            // v2.0 keywords (primary)
            "import" | "use" => Token::Import,
            // "const" removed from here as it's now v3.0 primary
            "mut" => {
                eprintln!("Warning: 'mut' is deprecated, use 'var' instead");
                Token::Mut
            }
            "async" => Token::Async,
            "extern" => {
                eprintln!("Warning: 'extern' is deprecated, use @python/@rust/@sql blocks instead");
                Token::Extern
            }
            "main" => {
                eprintln!("Warning: 'main' is deprecated, use 'mn:' block instead");
                Token::Main
            }
            "struct" => Token::Struct,
            "global" => Token::Global,
            "static" => Token::Static,
            "local" => Token::Local,
            "parallel" => Token::Parallel,
            "also_for" => Token::AlsoFor,
            "also_while" => Token::AlsoWhile,

            // Legacy keywords (backward compatibility with warnings)
            "imp" => {
                eprintln!("Warning: 'imp' is deprecated, use 'import' instead");
                Token::Imp
            }
            "def" => {
                eprintln!("Warning: 'def' is deprecated, use 'const' or 'mut' for variables, 'fn' for functions");
                Token::Def
            }
            "fn" => Token::Fn, // 'fn' is still valid in v2.0
            "asy" => {
                eprintln!("Warning: 'asy' is deprecated, use 'async' instead");
                Token::Asy
            }
            "cs" => {
                eprintln!("Warning: 'cs' is deprecated, use 'extern' instead");
                Token::Cs
            }
            "mn" => Token::Mn, // mn is v3.0 main entry (no deprecation)
            // Legacy ownership (deprecated in GUL 101)
            "own" => {
                eprintln!(
                    "Warning: 'own' is deprecated, use explicit port ownership in node contracts"
                );
                Token::Own
            }
            "ref" => {
                eprintln!(
                    "Warning: 'ref' as standalone keyword is deprecated, use in node contracts"
                );
                Token::Ref
            }
            "copy" => {
                eprintln!("Warning: 'copy' is deprecated, use 'gives' instead");
                Token::Copy
            }
            // GUL 101 ownership modes (current)
            "borrow" => Token::Borrow,
            "take" => Token::TakeOwn,
            "gives" => Token::Gives,
            "await" => Token::Await,
            "loop" => Token::Loop,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "in" => Token::In,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "finally" => Token::Finally,
            "throw" => Token::Throw,

            // Boolean literals
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),

            _ => Token::Identifier(value.clone()),
        };

        // If it's an identifier, check if it's followed by a unit pattern (e.g., m/s, kg)
        if matches!(token, Token::Identifier(_)) {
            if let Token::Identifier(ref id) = token {
                // Check for ui![...] syntax
                if id == "ui" && self.current_char == Some('!') && self.peek(1) == Some('[') {
                    self.advance(); // Skip !
                                    // Now at [, read_ui_sprite typically expects 4 chars upfront (^&^[) so we need to handle manually
                                    // or refactor read_ui_sprite.
                                    // Let's implement reading here for now to avoid breaking read_ui_sprite signature assumptions
                    self.advance(); // Skip [

                    let mut content = String::new();
                    let mut depth = 1;

                    while let Some(ch) = self.current_char {
                        if ch == '[' {
                            depth += 1;
                            content.push(ch);
                            self.advance();
                        } else if ch == ']' {
                            depth -= 1;
                            if depth == 0 {
                                self.advance();
                                break;
                            }
                            content.push(ch);
                            self.advance();
                        } else {
                            content.push(ch);
                            self.advance();
                        }
                    }
                    return Token::UiSprite(content);
                }
            }

            // Check for unit patterns like m/s, m/s^2, kg, etc.
            if self.current_char == Some('/') {
                let mut unit = value.clone();

                // Read unit pattern
                while let Some(ch) = self.current_char {
                    if ch == '/' || ch == '^' || ch.is_alphanumeric() {
                        unit.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                return Token::Unit(unit);
            }
        }

        token
    }
}
