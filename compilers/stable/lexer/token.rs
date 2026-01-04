// GUL v3.2 Compiler - Token Definitions
// Defines all token types used by the lexer

use std::collections::{HashMap, HashSet};

// Token types enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Literals
    Integer,
    Float,
    String,
    Boolean,
    TrueKeyword,
    FalseKeyword,
    NoneLiteral,

    // Identifiers and Keywords
    Identifier,

    // Keywords
    Let,
    Var,
    Fn,
    Async,
    Struct,
    Enum,
    Match,
    If,
    Elif,
    Else,
    For,
    While,
    Loop,
    In,
    Break,
    Continue,
    Return,
    Try,
    Catch,
    Finally,

    // Import system
    AtImp, // @imp
    AtPython, // @python
    AtRust, // @rust
    AtSql, // @sql
    AtJs, // @js
    AtUi, // @ui

    // Type constructors
    AtInt, // @int
    AtFloat, // @float
    AtStr, // @str
    AtBool, // @bool
    AtList, // @list
    AtTuple, // @tuple
    AtSet, // @set
    AtDict, // @dict
    AtTensor, // @tensor

    // ML decorators
    AtGrad, // @grad

    // Ownership modes
    Borrow,
    Ref,
    Move,
    Kept,

    // Operators
    Plus, // +
    Minus, // -
    Star, // *
    Slash, // /
    Percent, // %
    DoubleStar, // **

    // Comparison
    EqualEqual, // ==
    NotEqual, // !=
    Greater, // >
    GreaterEq, // >=
    Less, // <
    LessEq, // <=

    // Logical
    And, // &&
    Or, // ||
    Not, // not

    // Assignment
    Equal, // =
    PlusEq, // +=
    MinusEq, // -=
    StarEq, // *=
    SlashEq, // /=

    // Delimiters
    LeftParen, // (
    RightParen, // )
    LeftBrace, // {
    RightBrace, // }
    LeftBracket, // [
    RightBracket, // ]

    // Punctuation
    Comma, // ,
    Colon, // :
    Semicolon, // ;
    Dot, // .
    Arrow, // ->
    FatArrow, // =>
    DotDot, // ..
    DotDotEq, // ..=
    Pipeline, // |>

    // Special
    Newline,
    Indent,
    Dedent,
    Eof,
    Comment,
    Error,

    // Entry point
    Mn, // mn:

// Token struct representing a single lexical token
}
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,

}
impl Token {
    pub fn to_string(&mut self)  ->  String {
        return format!("{:?}('{}') at {}:{}", self.token_type, self.value, self.line, self.column);

    }
    pub fn is_keyword(&self) -> bool {
        // Check if token is a keyword// 
        let keywords = HashSet::from([TokenType::Let, TokenType::Var, TokenType::Fn,
            TokenType::Async, TokenType::Struct, TokenType::Enum,
            TokenType::Match, TokenType::If, TokenType::Elif,
            TokenType::Else, TokenType::For, TokenType::While,
            TokenType::Loop, TokenType::In, TokenType::Break,
            TokenType::Continue, TokenType::Return, TokenType::Try,
            TokenType::Catch, TokenType::Finally, TokenType::Mn]);
        return keywords.contains(&self.token_type);

    }
    pub fn is_operator(&self) -> bool {
        // Check if token is an operator// 
        let operators = HashSet::from([TokenType::Plus, TokenType::Minus, TokenType::Star,
            TokenType::Slash, TokenType::Percent, TokenType::DoubleStar,
            TokenType::EqualEqual, TokenType::NotEqual,
            TokenType::Greater, TokenType::GreaterEq,
            TokenType::Less, TokenType::LessEq,
            TokenType::And, TokenType::Or, TokenType::Not,
            TokenType::Pipeline]);
        return operators.contains(&self.token_type);

    }
    pub fn is_literal(&self) -> bool {
        // Check if token is a literal value// 
        let literals = HashSet::from([TokenType::Integer, TokenType::Float,
            TokenType::String, TokenType::Boolean,
            TokenType::NoneLiteral]);
        return literals.contains(&self.token_type);

// Helper function to create tokens
}
}
pub fn create_token(token_type: TokenType, value: String, line: usize, column: usize)  ->  Token {
    let t = Token{
        token_type: token_type,
        value: value,
        line: line,
        column: column,
    };
    return t;

}
pub fn get_keyword_type(word: String)  ->  TokenType {
    // Get the token type for a keyword// 
    if word == "let".to_string() {
        return TokenType::Let;
    }
    else if word == "var".to_string() {
        return TokenType::Var;
    }
    else if word == "fn".to_string() {
        return TokenType::Fn;
    }
    else if word == "async".to_string() {
        return TokenType::Async;
    }
    else if word == "struct".to_string() {
        return TokenType::Struct;
    }
    else if word == "enum".to_string() {
        return TokenType::Enum;
    }
    else if word == "match".to_string() {
        return TokenType::Match;
    }
    else if word == "if".to_string() {
        return TokenType::If;
    }
    else if word == "else if".to_string() {
        return TokenType::Elif;
    }
    else if word == "else".to_string() {
        return TokenType::Else;
    }
    else if word == "for".to_string() {
        return TokenType::For;
    }
    else if word == "while".to_string() {
        return TokenType::While;
    }
    else if word == "loop".to_string() {
        return TokenType::Loop;
    }
    else if word == "in".to_string() {
        return TokenType::In;
    }
    else if word == "break".to_string() {
        return TokenType::Break;
    }
    else if word == "continue".to_string() {
        return TokenType::Continue;
    }
    else if word == "return".to_string() {
        return TokenType::Return;
    }
    else if word == "try".to_string() {
        return TokenType::Try;
    }
    else if word == "catch".to_string() {
        return TokenType::Catch;
    }
    else if word == "finally".to_string() {
        return TokenType::Finally;
    }
    else if word == "mn".to_string() {
        return TokenType::Mn;
    }
    else if word == "borrow".to_string() {
        return TokenType::Borrow;
    }
    else if word == "ref".to_string() {
        return TokenType::Ref;
    }
    else if word == "move".to_string() {
        return TokenType::Move;
    }
    else if word == "kept".to_string() {
        return TokenType::Kept;
    }
    else if word == "&&".to_string() {
        return TokenType::And;
    }
    else if word == "||".to_string() {
        return TokenType::Or;
    }
    else if word == "!".to_string() {
        return TokenType::Not;
    }
    else if word == "true".to_string() {
        return TokenType::TrueKeyword;
    }
    else if word == "false".to_string() {
        return TokenType::FalseKeyword;
    }
    return TokenType::Error;

}
pub fn is_keyword(word: String)  ->  bool {
    // Check if word is a reserved keyword// 
    return get_keyword_type(word) != TokenType::Error;

}
pub fn get_type_constructor_type(text: String)  ->  TokenType {
    // Get the token type for a type constructor// 
    // Check for @ prefix (byte 64)
    if text.len() < 2 || text.as_bytes()[0] != 64 {
        return TokenType::Error;

    }
    let suffix = &text[1..];
    if suffix == "int".to_string() {
        return TokenType::AtInt;
    }
    else if suffix == "float".to_string() {
        return TokenType::AtFloat;
    }
    else if suffix == "str".to_string() {
        return TokenType::AtStr;
    }
    else if suffix == "bool".to_string() {
        return TokenType::AtBool;
    }
    else if suffix == "list".to_string() {
        return TokenType::AtList;
    }
    else if suffix == "tuple".to_string() {
        return TokenType::AtTuple;
    }
    else if suffix == "set".to_string() {
        return TokenType::AtSet;
    }
    else if suffix == "dict".to_string() {
        return TokenType::AtDict;
    }
    else if suffix == "tensor".to_string() {
        return TokenType::AtTensor;
    }
    return TokenType::Error;

}
pub fn is_type_constructor(text: String)  ->  bool {
    // Check if text is a type constructor// 
    return get_type_constructor_type(text) != TokenType::Error;

}
pub fn get_decorator_type(text: String)  ->  TokenType {
    // Get the token type for a decorator// 
    // Check for @ prefix (byte 64)
    if text.len() < 2 || text.as_bytes()[0] != 64 {
        return TokenType::Error;

    }
    let suffix = &text[1..];
    if suffix == "imp".to_string() {
        return TokenType::AtImp;
    }
    else if suffix == "python".to_string() {
        return TokenType::AtPython;
    }
    else if suffix == "rust".to_string() {
        return TokenType::AtRust;
    }
    else if suffix == "sql".to_string() {
        return TokenType::AtSql;
    }
    else if suffix == "js".to_string() {
        return TokenType::AtJs;
    }
    else if suffix == "ui".to_string() {
        return TokenType::AtUi;
    }
    else if suffix == "grad".to_string() {
        return TokenType::AtGrad;
    }
    return TokenType::Error;

}
pub fn is_decorator(text: String)  ->  bool {
    // Check if text is a decorator/annotation// 
    return get_decorator_type(text) != TokenType::Error;
}