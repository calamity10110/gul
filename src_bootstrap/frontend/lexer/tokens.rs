// Token definitions for the GUL lexer

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
    Import,   // import (replaces imp)
    Const,    // const (explicit immutable)
    Mut,      // mut (mutable)
    Async,    // async (replaces asy)
    Extern,   // extern (replaces cs)
    Main,     // main (optional, can use without mn)
    Struct,   // struct definition
    Global,   // @global
    Static,   // @static
    Local,    // @local
    Parallel, // parallel keyword

    // New keywords (v3.0)
    Let, // let (immutable, replaces const/def)
    Var, // var (mutable, replaces mut/?)

    // Data-flow node system (v3.0)
    Node,   // node declaration
    ReIn,   // re_in: required input
    ReOut,  // re_out: required output
    OptIn,  // opt_in: optional input
    OptOut, // opt_out: optional output
    Trait,  // trait definition

    // Ownership modes (GUL 101) - extends legacy Own/Ref/Copy
    Borrow,  // borrow<T> - mutable access, no move
    TakeOwn, // take - ownership transfer
    Gives,   // gives - ownership via copy

    // v3.2 Keywords
    AlsoFor,   // also_for (parallel for)
    AlsoWhile, // also_while (parallel while)

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
    FatArrow, // =>

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
