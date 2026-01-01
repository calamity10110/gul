// Auto-generated from GUL source
#![allow(unused_variables, dead_code, unused_mut)]

use std::collections::{HashMap, HashSet, VecDeque};

// GUL v3.2 Compiler - Token Definitions
// Defines all token types used by the lexer

use std::collections;

// Token types enumeration
enum TokenType {
    // Literals
    Integer
    Float
    String
    Boolean
    
    // Identifiers and Keywords
    Identifier
    
    // Keywords
    Let
    Var
    Fn
    Async
    Struct
    Enum
    Match
    If
    Elif
    Else
    For
    While
    Loop
    In
    Break
    Continue
    Return
    Try
    Catch
    Finally
    
    // Import system
    AtImp          # @imp
    AtPython       # @python
    AtRust         # @rust
    AtSql          # @sql
    AtJs           # @js
    AtUi           # @ui
    
    // Type constructors
    AtInt          # @int
    AtFloat        # @float
    AtStr          # @str
    AtBool         # @bool
    AtList         # @list
    AtTuple        # @tuple
    AtSet          # @set
    AtDict         # @dict
    
    // Ownership modes
    Borrow
    Ref
    Move
    Kept
    
    // Operators
    Plus           # +
    Minus          # -
    Star           # *
    Slash          # /
    Percent        # %
    DoubleStar     # **
    
    // Comparison
    EqualEqual     # ==
    NotEqual       # !=
    Greater        # >
    GreaterEq      # >=
    Less           # <
    LessEq         # <=
    
    // Logical
    And            # &&
    Or             # ||
    Not            # !
    
    // Assignment
    Equal          # =
    PlusEq         # +=
    MinusEq        # -=
    StarEq         # *=
    SlashEq        # /=
    
    // Delimiters
    LeftParen      # (
    RightParen     # )
    LeftBrace      # {
    RightBrace     # }
    LeftBracket    # [
    RightBracket   # ]
    
    // Punctuation
    Comma          # ,
    Colon          #  {
    Semicolon      # ;
    Dot            # .
    Arrow          # ->
    FatArrow       # =>
    DotDot         # ..
    DotDotEq       # ..=
    
    // Special
    Newline
    Indent
    Dedent
    Eof
    Comment
    
    // Entry point
    Mn             # mn {

// Token struct representing a single lexical token
struct Token {
    type: TokenType
    value: @str
    line: @int
    column: @int
    
fn String to_string(&self) -> String {
        return @str(f\"{self.type}('{self.value}') at {self.line}:{self.column}\")
    
fn bool is_keyword(&self) -> bool {
        \"\"\"Check if token is a keyword\"\"\"
        let keywords = HashSet{;
            TokenType.Let, TokenType.Var, TokenType.Fn,
            TokenType.Async, TokenType.Struct, TokenType.Enum,
            TokenType.Match, TokenType.If, TokenType.Elif,
            TokenType.Else, TokenType.For, TokenType.While,
            TokenType.Loop, TokenType.In, TokenType.Break,
            TokenType.Continue, TokenType.Return, TokenType.Try,
            TokenType.Catch, TokenType.Finally, TokenType.Mn
        }
        return self.type in keywords
    
fn bool is_operator(&self) -> bool {
        \"\"\"Check if token is an operator\"\"\"
        let operators = HashSet{;
            TokenType.Plus, TokenType.Minus, TokenType.Star,
            TokenType.Slash, TokenType.Percent, TokenType.DoubleStar,
            TokenType.EqualEqual, TokenType.NotEqual,
            TokenType.Greater, TokenType.GreaterEq,
            TokenType.Less, TokenType.LessEq,
            TokenType.And, TokenType.Or, TokenType.Not
        }
        return self.type in operators
    
fn bool is_literal(&self) -> bool {
        \"\"\"Check if token is a literal value\"\"\"
        let literals = HashSet{;
            TokenType.Integer, TokenType.Float,
            TokenType.String, TokenType.Boolean
        }
        return self.type in literals

// Helper function to create tokens
fn HashMap create_token(type { TokenType, value: String, line: i64, column: i64) -> Token:
    return Token{
        type: type,
        value: value,
        line: line,
        column: column
    }

// Keyword mapping
let KEYWORDS = HashMap{;
    \"let\": TokenType.Let,
    \"var\": TokenType.Var,
    \"fn\": TokenType.Fn,
    \"async\": TokenType.Async,
    \"struct\": TokenType.Struct,
    \"enum\": TokenType.Enum,
    \"match\": TokenType.Match,
    \"if\": TokenType.If,
    \"elif\": TokenType.Elif,
    \"else\": TokenType.Else,
    \"for\": TokenType.For,
    \"while\": TokenType.While,
    \"loop\": TokenType.Loop,
    \"in\": TokenType.In,
    \"break\": TokenType.Break,
    \"continue\": TokenType.Continue,
    \"return\": TokenType.Return,
    \"try\": TokenType.Try,
    \"catch\": TokenType.Catch,
    \"finally\": TokenType.Finally,
    \"mn\": TokenType.Mn,
    \"borrow\": TokenType.Borrow,
    \"ref\": TokenType.Ref,
    \"move\": TokenType.Move,
    \"kept\": TokenType.Kept,
    \"&&\": TokenType.And,
    \"||\": TokenType.Or,
    \"!\": TokenType.Not,
    \"true\": TokenType.Boolean,
    \"false\": TokenType.Boolean
}

// Type constructor prefix mapping
let TYPE_CONSTRUCTORS = HashMap{;
    \"@int\": TokenType.AtInt,
    \"@float\": TokenType.AtFloat,
    \"@str\": TokenType.AtStr,
    \"@bool\": TokenType.AtBool,
    \"@list\": TokenType.AtList,
    \"@tuple\": TokenType.AtTuple,
    \"@set\": TokenType.AtSet,
    \"@dict\": TokenType.AtDict
}

// Decorator/annotation mapping
let DECORATORS = HashMap{;
    \"@imp\": TokenType.AtImp,
    \"@python\": TokenType.AtPython,
    \"@rust\": TokenType.AtRust,
    \"@sql\": TokenType.AtSql,
    \"@js\": TokenType.AtJs,
    \"@ui\": TokenType.AtUi
}

fn bool is_keyword(word { String) -> bool:
    \"\"\"Check if a word is a GUL keyword\"\"\"
    return word in KEYWORDS

fn get_keyword_type(word { String) -> TokenType:
    \"\"\"Get the token type for a keyword\"\"\"
    return KEYWORDS[word]

fn bool is_type_constructor(text { String) -> bool:
    \"\"\"Check if text is a type constructor\"\"\"
    return text in TYPE_CONSTRUCTORS

fn get_type_constructor_type(text { String) -> TokenType:
    \"\"\"Get the token type for a type constructor\"\"\"
    return TYPE_CONSTRUCTORS[text]

fn bool is_decorator(text { String) -> bool:
    \"\"\"Check if text is a decorator/annotation\"\"\"
    return text in DECORATORS

fn get_decorator_type(text { String) -> TokenType:
    \"\"\"Get the token type for a decorator\"\"\"
    return DECORATORS[text]
