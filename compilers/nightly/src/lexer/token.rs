// GUL Compiler - Token Definitions
// Complete token types for GUL v3.2 syntax

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
    // Literals
    Integer,
    Float,
    String,
    FString,
    Boolean,
    
    // Identifiers and Keywords
    Identifier,
    
    // Keywords - Declarations
    Let,
    Var,
    Fn,
    Async,
    Struct,
    Enum,
    Impl,
    Trait,
    Type,
    
    // Keywords - Control Flow
    If,
    Elif,
    Else,
    Match,
    While,
    For,
    Loop,
    In,
    Break,
    Continue,
    Return,
    
    // Keywords - Error Handling
    Try,
    Catch,
    Finally,
    
    // Keywords - Imports
    Import,
    From,
    As,
    
    // Keywords - Other
    Mn,  // Main block
    Pass,
    
    // Ownership Keywords
    Ref,
    Own,
    Borrow,
    Move,
    Kept,
    
    // Operators - Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    DoubleStar,  // **
    
    // Operators - Comparison
    EqualEqual,
    NotEqual,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    
    // Operators - Logical
    And,
    Or,
    Not,
    
    // Operators - Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    
    // Operators - Assignment
    Equal,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    
    // Punctuation
    Comma,
    Colon,
    Semicolon,
    Dot,
    Arrow,      // ->
    FatArrow,   // =>
    At,         // @
    Hash,       // #
    Dollar,     // $
    Question,   // ?
    
    // Special
    Newline,
    Indent,
    Dedent,
    Eof,
    
    // Decorators
    Decorator,  // @decorator_name
    
    // Type Annotations
    AtInt,
    AtFloat,
    AtStr,
    AtBool,
    AtList,
    AtDict,
    AtSet,
    AtTuple,
    AtOption,
    AtBox,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            column,
        }
    }
    
    #[allow(dead_code)]
    pub fn is_keyword(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::Let
                | TokenType::Var
                | TokenType::Fn
                | TokenType::Async
                | TokenType::Struct
                | TokenType::Enum
                | TokenType::If
                | TokenType::Elif
                | TokenType::Else
                | TokenType::Match
                | TokenType::While
                | TokenType::For
                | TokenType::Loop
                | TokenType::In
                | TokenType::Break
                | TokenType::Continue
                | TokenType::Return
                | TokenType::Try
                | TokenType::Catch
                | TokenType::Finally
                | TokenType::Import
                | TokenType::From
                | TokenType::As
                | TokenType::Mn
                | TokenType::Pass
                | TokenType::Ref
                | TokenType::Own
                | TokenType::Borrow
                | TokenType::Move
                | TokenType::Kept
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}('{}') at {}:{}",
            self.token_type, self.lexeme, self.line, self.column
        )
    }
}

pub fn get_keyword_type(word: &str) -> Option<TokenType> {
    match word {
        "let" => Some(TokenType::Let),
        "var" => Some(TokenType::Var),
        "fn" => Some(TokenType::Fn),
        "async" => Some(TokenType::Async),
        "struct" => Some(TokenType::Struct),
        "enum" => Some(TokenType::Enum),
        "impl" => Some(TokenType::Impl),
        "trait" => Some(TokenType::Trait),
        "type" => Some(TokenType::Type),
        "if" => Some(TokenType::If),
        "elif" => Some(TokenType::Elif),
        "else" => Some(TokenType::Else),
        "match" => Some(TokenType::Match),
        "while" => Some(TokenType::While),
        "for" => Some(TokenType::For),
        "loop" => Some(TokenType::Loop),
        "in" => Some(TokenType::In),
        "break" => Some(TokenType::Break),
        "continue" => Some(TokenType::Continue),
        "return" => Some(TokenType::Return),
        "try" => Some(TokenType::Try),
        "catch" => Some(TokenType::Catch),
        "finally" => Some(TokenType::Finally),
        "import" => Some(TokenType::Import),
        "from" => Some(TokenType::From),
        "as" => Some(TokenType::As),
        "mn" => Some(TokenType::Mn),
        "pass" => Some(TokenType::Pass),
        "ref" => Some(TokenType::Ref),
        "own" => Some(TokenType::Own),
        "borrow" => Some(TokenType::Borrow),
        "move" => Some(TokenType::Move),
        "kept" => Some(TokenType::Kept),
        "and" => Some(TokenType::And),
        "or" => Some(TokenType::Or),
        "not" => Some(TokenType::Not),
        "true" | "false" => Some(TokenType::Boolean),
        _ => None,
    }
}
