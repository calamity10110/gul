// Lexer module - tokenizes source code

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords (legacy)
    Imp,
    Def,
    Fn,
    Asy,
    Cs,
    Mn,
    Own,
    Ref,
    Copy,
    Await,
    Loop,
    If,
    Elif,
    Else,
    For,
    While,
    In,
    Return,
    Break,
    Continue,
    Try,
    Catch,
    Finally,
    Throw,

    // New keywords (v2.0)
    Import, // import (replaces imp)
    Const,  // const (explicit immutable)
    Mut,    // mut (mutable)
    Async,  // async (replaces asy)
    Extern, // extern (replaces cs)
    Main,   // main (optional, can use without mn)
    Struct, // struct definition
    Global, // @global
    Static, // @static
    Local,  // @local

    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),

    // Identifiers
    Identifier(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Dot,
    Arrow,

    // UI Syntax
    UiSprite(String),

    // Scientific Units
    Unit(String), // e.g., "m/s", "m/s^2", "kg"

    // Special
    Newline,
    Indent,
    Dedent,
    Eof,

    // Additional operators
    BangEqual,      // !=
    LessLess,       // <<
    GreaterGreater, // >>
    Ampersand,      // &
    Pipe,           // |
    Semicolon,      // ;
    QuestionMark,   // ?
    At,             // @
}

// v2.1 Bracket Equivalence - Helper functions
impl Token {
    /// Check if this token is any opening bracket (), [], {}
    pub fn is_open_bracket(&self) -> bool {
        matches!(
            self,
            Token::LeftParen | Token::LeftBracket | Token::LeftBrace
        )
    }

    /// Check if this token is any closing bracket ), ], }
    pub fn is_close_bracket(&self) -> bool {
        matches!(
            self,
            Token::RightParen | Token::RightBracket | Token::RightBrace
        )
    }

    /// Get the matching closing bracket for an opening bracket
    pub fn matching_close(&self) -> Option<Token> {
        match self {
            Token::LeftParen => Some(Token::RightParen),
            Token::LeftBracket => Some(Token::RightBracket),
            Token::LeftBrace => Some(Token::RightBrace),
            _ => None,
        }
    }

    /// Check if two brackets match (same type)
    pub fn brackets_match(&self, other: &Token) -> bool {
        matches!(
            (self, other),
            (Token::LeftParen, Token::RightParen)
                | (Token::LeftBracket, Token::RightBracket)
                | (Token::LeftBrace, Token::RightBrace)
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "Identifier({})", s),
            Token::Integer(n) => write!(f, "Integer({})", n),
            Token::Float(n) => write!(f, "Float({})", n),
            Token::String(s) => write!(f, "String(\"{}\")", s),
            Token::UiSprite(s) => write!(f, "UiSprite({})", s),
            Token::QuestionMark => write!(f, "?"),
            Token::At => write!(f, "@"),
            Token::Global => write!(f, "global"),
            Token::Static => write!(f, "static"),
            Token::Local => write!(f, "local"),
            _ => write!(f, "{:?}", self),
        }
    }
}

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
                            tokens.push(Token::Slash);
                            self.advance();
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
            // v2.0 keywords (primary)
            "import" | "use" => Token::Import,
            "const" => Token::Const,
            "mut" => Token::Mut,
            "async" => Token::Async,
            "extern" => Token::Extern,
            "main" => Token::Main,
            "struct" => Token::Struct,
            "global" => Token::Global,
            "static" => Token::Static,
            "local" => Token::Local,

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
            "mn" => {
                eprintln!("Warning: 'mn' is deprecated, use 'main' instead");
                Token::Mn
            }
            "own" => Token::Own, // ownership keywords still valid
            "ref" => Token::Ref,
            "copy" => Token::Copy,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("def x = 10");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Integer(10));
    }

    #[test]
    fn test_ui_sprite() {
        let mut lexer = Lexer::new("def tree = ^&^[tree]");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("tree".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::UiSprite("tree".to_string()));
    }

    #[test]
    fn test_ui_sprite_with_properties() {
        let mut lexer = Lexer::new("^&^[slider{min=0, max=100}]");
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens[0],
            Token::UiSprite("slider{min=0, max=100}".to_string())
        );
    }

    #[test]
    fn test_multiline_comment() {
        let mut lexer = Lexer::new("#[\nThis is a\nmulti-line comment\n]#\ndef x = 5");
        let tokens = lexer.tokenize();

        // Should skip the multi-line comment
        assert_eq!(tokens[0], Token::Newline);
        assert_eq!(tokens[1], Token::Def);
        assert_eq!(tokens[2], Token::Identifier("x".to_string()));
    }

    #[test]
    fn test_scientific_units() {
        let mut lexer = Lexer::new("def speed = 10 m/s");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("speed".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Integer(10));
        assert_eq!(tokens[4], Token::Unit("m/s".to_string()));
    }

    #[test]
    fn test_scientific_units_complex() {
        let mut lexer = Lexer::new("def accel = 9.81 m/s^2");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("accel".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Float(9.81));
        assert_eq!(tokens[4], Token::Unit("m/s^2".to_string()));
    }

    #[test]
    fn test_comparison_operators() {
        let mut lexer = Lexer::new("a == b != c < d <= e > f >= g");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::EqualEqual);
        assert_eq!(tokens[2], Token::Identifier("b".to_string()));
        assert_eq!(tokens[3], Token::BangEqual);
        assert_eq!(tokens[4], Token::Identifier("c".to_string()));
        assert_eq!(tokens[5], Token::Less);
        assert_eq!(tokens[6], Token::Identifier("d".to_string()));
        assert_eq!(tokens[7], Token::LessEqual);
        assert_eq!(tokens[8], Token::Identifier("e".to_string()));
        assert_eq!(tokens[9], Token::Greater);
        assert_eq!(tokens[10], Token::Identifier("f".to_string()));
        assert_eq!(tokens[11], Token::GreaterEqual);
    }

    #[test]
    fn test_logical_operators() {
        let mut lexer = Lexer::new("a && b || !c");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::And);
        assert_eq!(tokens[2], Token::Identifier("b".to_string()));
        assert_eq!(tokens[3], Token::Or);
        assert_eq!(tokens[4], Token::Not);
        assert_eq!(tokens[5], Token::Identifier("c".to_string()));
    }

    #[test]
    fn test_bitwise_operators() {
        let mut lexer = Lexer::new("a & b | c << d >> e");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::Ampersand);
        assert_eq!(tokens[2], Token::Identifier("b".to_string()));
        assert_eq!(tokens[3], Token::Pipe);
        assert_eq!(tokens[4], Token::Identifier("c".to_string()));
        assert_eq!(tokens[5], Token::LessLess);
        assert_eq!(tokens[6], Token::Identifier("d".to_string()));
        assert_eq!(tokens[7], Token::GreaterGreater);
    }

    #[test]
    fn test_ownership_keywords() {
        let mut lexer = Lexer::new("own x ref y copy z");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Own);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Ref);
        assert_eq!(tokens[3], Token::Identifier("y".to_string()));
        assert_eq!(tokens[4], Token::Copy);
        assert_eq!(tokens[5], Token::Identifier("z".to_string()));
    }

    #[test]
    fn test_async_await() {
        let mut lexer = Lexer::new("asy fetch():\n    res = await http.get(url)");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Asy);
        assert_eq!(tokens[1], Token::Identifier("fetch".to_string()));
        assert_eq!(tokens[2], Token::LeftParen);
        assert_eq!(tokens[3], Token::RightParen);
        assert_eq!(tokens[4], Token::Colon);
        assert_eq!(tokens[5], Token::Newline);
        // ... more tokens
        let await_pos = tokens.iter().position(|t| matches!(t, Token::Await));
        assert!(await_pos.is_some());
    }

    #[test]
    fn test_float_numbers() {
        let mut lexer = Lexer::new("3.14 2.718 0.5");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Float(3.14));
        assert_eq!(tokens[1], Token::Float(2.718));
        assert_eq!(tokens[2], Token::Float(0.5));
    }

    #[test]
    fn test_string_literals() {
        let mut lexer = Lexer::new("\"Hello, World!\" \"test\"");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::String("Hello, World!".to_string()));
        assert_eq!(tokens[1], Token::String("test".to_string()));
    }

    #[test]
    fn test_control_flow_keywords() {
        let mut lexer =
            Lexer::new("if x > 0:\n    return x\nelif x < 0:\n    return -x\nelse:\n    return 0");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::If);
        let elif_pos = tokens.iter().position(|t| matches!(t, Token::Elif));
        assert!(elif_pos.is_some());
        let else_pos = tokens.iter().position(|t| matches!(t, Token::Else));
        assert!(else_pos.is_some());
        let return_count = tokens.iter().filter(|t| matches!(t, Token::Return)).count();
        assert_eq!(return_count, 3);
    }

    #[test]
    fn test_ui_sprite_variations() {
        // Test various UI sprite formats from SYNTAX.md
        // These test the ^&^[component{properties}] syntax for inline UI components
        let test_cases = vec![
            ("^&^[tree]", "tree"),
            (
                "^&^[slider{min=0, max=100, value=50}]",
                "slider{min=0, max=100, value=50}",
            ),
            (
                "^&^[button{text=\"Click Me\"}]",
                "button{text=\"Click Me\"}",
            ),
            ("^&^[image:icon]", "image:icon"),
            (
                "^&^[sprite:player{x=10, y=20, width=32, height=32}]",
                "sprite:player{x=10, y=20, width=32, height=32}",
            ),
        ];

        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();
            assert_eq!(tokens[0], Token::UiSprite(expected.to_string()));
        }
    }

    #[test]
    fn test_ui_macro_syntax() {
        let mut lexer = Lexer::new("ui![button{text=\"Click\"}]");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens[0],
            Token::UiSprite("button{text=\"Click\"}".to_string())
        );
    }

    #[test]
    fn test_foreign_language_blocks() {
        // Test foreign language block syntax from SYNTAX.md
        // This tests the cs language: ... syntax for multi-language integration
        let test_cases = vec![
            ("cs rust:\n    fn sum(a: i32, b: i32) -> i32 { a + b }", "rust"),
            ("cs python:\n    import math\n    def compute(x):\n        return math.sin(x)", "python"),
            ("cs js:\n    export function greet(name) {\n        return `Hello, ${name}!`;\n    }", "js"),
            ("cs ts:\n    export function add(a: number, b: number): number {\n        return a + b;\n    }", "ts"),
            ("cs c:\n    int fibonacci(int n) {\n        return n <= 1 ? n : fibonacci(n-1) + fibonacci(n-2);\n    }", "c"),
            ("cs sql:\n    select * from users where age > 18 order by name;", "sql"),
        ];

        for (input, expected_lang) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();

            // Find the Cs token followed by identifier
            let cs_pos = tokens.iter().position(|t| matches!(t, Token::Cs));
            assert!(cs_pos.is_some(), "Expected Cs token in: {}", input);

            // Check that the next token is the expected language identifier
            if let Some(pos) = cs_pos {
                if pos + 1 < tokens.len() {
                    if let Token::Identifier(lang) = &tokens[pos + 1] {
                        assert_eq!(
                            lang, expected_lang,
                            "Expected language {} but got {}",
                            expected_lang, lang
                        );
                    } else {
                        panic!("Expected identifier after Cs token");
                    }
                }
            }
        }
    }

    #[test]
    fn test_type_annotations() {
        let mut lexer =
            Lexer::new("def name: str = \"Alice\"\ndef age: int = 30\ndef speed: float = 10.5 m/s");
        let tokens = lexer.tokenize();

        // Check for colon tokens (type annotations)
        let colon_count = tokens.iter().filter(|t| matches!(t, Token::Colon)).count();
        assert_eq!(colon_count, 3);

        // Check for type identifiers
        let type_identifiers: Vec<_> = tokens
            .iter()
            .filter_map(|t| {
                if let Token::Identifier(id) = t {
                    if id == "str" || id == "int" || id == "float" {
                        Some(id.as_str())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(type_identifiers, vec!["str", "int", "float"]);
    }

    #[test]
    fn test_loop_constructs() {
        let mut lexer = Lexer::new("loop:\n    print(\"infinite\")\n    break_if(condition)\n\nfor item in collection:\n    process(item)\n\nwhile condition:\n    update()");
        let tokens = lexer.tokenize();

        // Check for loop keywords
        assert!(tokens.iter().any(|t| matches!(t, Token::Loop)));
        assert!(tokens.iter().any(|t| matches!(t, Token::For)));
        assert!(tokens.iter().any(|t| matches!(t, Token::While)));
        assert!(tokens.iter().any(|t| matches!(t, Token::In)));
    }

    #[test]
    fn test_import_statements() {
        let mut lexer =
            Lexer::new("imp std.io\nimp std.http\nimp ai.cv\nimp ui\nimp embedded.gpio");
        let tokens = lexer.tokenize();

        // Check for import keywords
        let import_count = tokens.iter().filter(|t| matches!(t, Token::Imp)).count();
        assert_eq!(import_count, 5);

        // Check for dot notation in imports
        let dot_count = tokens.iter().filter(|t| matches!(t, Token::Dot)).count();
        assert_eq!(dot_count, 4); // std.io, std.http, ai.cv, embedded.gpio
    }

    #[test]
    fn test_main_entry_point() {
        let mut lexer = Lexer::new("mn main():\n    print(\"Hello, World!\")\n    ui.print(^&^[tree])\n    data = await fetch(\"https://api.example.com\")\n    print(data)");
        let tokens = lexer.tokenize();

        // Check for main keyword
        assert!(tokens.iter().any(|t| matches!(t, Token::Mn)));

        // Check for await keyword
        assert!(tokens.iter().any(|t| matches!(t, Token::Await)));

        // Check for UI sprite
        assert!(tokens.iter().any(|t| matches!(t, Token::UiSprite(_))));
    }

    #[test]
    fn test_scientific_expressions() {
        // Test scientific notation and expressions from SYNTAX.md
        // Covers physics formulas, chemistry expressions, and mathematical operations
        let test_cases = vec![
            (
                "def F = m * a",
                vec![
                    Token::Def,
                    Token::Identifier("F".to_string()),
                    Token::Equal,
                    Token::Identifier("m".to_string()),
                    Token::Star,
                    Token::Identifier("a".to_string()),
                ],
            ),
            (
                "def E = m * c^2",
                vec![
                    Token::Def,
                    Token::Identifier("E".to_string()),
                    Token::Equal,
                    Token::Identifier("m".to_string()),
                    Token::Star,
                    Token::Identifier("c".to_string()),
                    Token::Caret,
                    Token::Integer(2),
                ],
            ),
            (
                "def v = d / t",
                vec![
                    Token::Def,
                    Token::Identifier("v".to_string()),
                    Token::Equal,
                    Token::Identifier("d".to_string()),
                    Token::Slash,
                    Token::Identifier("t".to_string()),
                ],
            ),
            (
                "def pH = -log10([H+])",
                vec![
                    Token::Def,
                    Token::Identifier("pH".to_string()),
                    Token::Equal,
                    Token::Minus,
                    Token::Identifier("log10".to_string()),
                    Token::LeftParen,
                    Token::LeftBracket,
                    Token::Identifier("H".to_string()),
                    Token::Plus,
                    Token::RightBracket,
                    Token::RightParen,
                ],
            ),
        ];

        for (input, expected_pattern) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();

            // Check that we have the expected token pattern
            for (i, expected_token) in expected_pattern.iter().enumerate() {
                if i < tokens.len() {
                    match (expected_token, &tokens[i]) {
                        (Token::Identifier(expected), Token::Identifier(actual)) => {
                            assert_eq!(expected, actual)
                        }
                        (expected, actual) => assert_eq!(expected, actual),
                    }
                }
            }
        }
    }

    #[test]
    fn test_v0_11_0_tokens() {
        let mut lexer = Lexer::new("?count = 0 @int @global");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::QuestionMark);
        assert_eq!(tokens[1], Token::Identifier("count".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Integer(0));
        assert_eq!(tokens[4], Token::At);
        assert_eq!(tokens[5], Token::Identifier("int".to_string()));
        assert_eq!(tokens[6], Token::At);
        assert_eq!(tokens[7], Token::Global);
    }

    #[test]
    fn test_indentation_tracking() {
        let code =
            "def func():\n    x = 1\n    if x > 0:\n        print(x)\n        y = 2\n    return x";
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();

        // Find indent and dedent tokens
        let indent_count = tokens.iter().filter(|t| matches!(t, Token::Indent)).count();
        let dedent_count = tokens.iter().filter(|t| matches!(t, Token::Dedent)).count();

        // Should have 2 indents (after func(): and after if:) and 2 dedents
        assert_eq!(indent_count, 2, "Expected 2 INDENT tokens");
        assert_eq!(dedent_count, 2, "Expected 2 DEDENT tokens");

        // Verify token sequence includes proper indentation
        assert!(tokens.contains(&Token::Indent));
        assert!(tokens.contains(&Token::Dedent));
    }

    #[test]
    fn test_nested_indentation() {
        let code = "if a:\n    if b:\n        if c:\n            x = 1\n        y = 2\n    z = 3";
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();

        let indent_count = tokens.iter().filter(|t| matches!(t, Token::Indent)).count();
        let dedent_count = tokens.iter().filter(|t| matches!(t, Token::Dedent)).count();

        // Should have 3 indents and 3 dedents for nested blocks
        assert_eq!(indent_count, 3);
        assert_eq!(dedent_count, 3);
    }

    // v2.1 Bracket Equivalence Tests
    #[test]
    fn test_bracket_helper_is_open() {
        assert!(Token::LeftParen.is_open_bracket());
        assert!(Token::LeftBracket.is_open_bracket());
        assert!(Token::LeftBrace.is_open_bracket());
        assert!(!Token::RightParen.is_open_bracket());
        assert!(!Token::Plus.is_open_bracket());
    }

    #[test]
    fn test_bracket_helper_is_close() {
        assert!(Token::RightParen.is_close_bracket());
        assert!(Token::RightBracket.is_close_bracket());
        assert!(Token::RightBrace.is_close_bracket());
        assert!(!Token::LeftParen.is_close_bracket());
        assert!(!Token::Minus.is_close_bracket());
    }

    #[test]
    fn test_bracket_helper_matching_close() {
        assert_eq!(Token::LeftParen.matching_close(), Some(Token::RightParen));
        assert_eq!(
            Token::LeftBracket.matching_close(),
            Some(Token::RightBracket)
        );
        assert_eq!(Token::LeftBrace.matching_close(), Some(Token::RightBrace));
        assert_eq!(Token::RightParen.matching_close(), None);
        assert_eq!(Token::Plus.matching_close(), None);
    }

    #[test]
    fn test_bracket_helper_brackets_match() {
        assert!(Token::LeftParen.brackets_match(&Token::RightParen));
        assert!(Token::LeftBracket.brackets_match(&Token::RightBracket));
        assert!(Token::LeftBrace.brackets_match(&Token::RightBrace));

        // Mismatched should be false
        assert!(!Token::LeftParen.brackets_match(&Token::RightBracket));
        assert!(!Token::LeftParen.brackets_match(&Token::RightBrace));
        assert!(!Token::LeftBracket.brackets_match(&Token::RightParen));
    }

    #[test]
    fn test_v21_all_bracket_types_tokenized() {
        let mut lexer = Lexer::new("func() func[] func{}");
        let tokens = lexer.tokenize();

        // Should have all bracket types
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftParen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightParen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftBracket)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightBracket)));
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftBrace)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightBrace)));
    }
}
