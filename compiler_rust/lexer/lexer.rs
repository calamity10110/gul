// Auto-generated from GUL source
#![allow(unused_variables, dead_code, unused_mut)]

use std::collections::{HashMap, HashSet, VecDeque};

// GUL v3.2 Compiler - Lexer Implementation
// Tokenizes GUL source code into a stream of tokens

use std::io;
use std::collections;
use compiler::lexer::token;

// Character stream for reading source code
struct CharStream {
    source: @str
    position: @int
    line: @int
    column: @int
    length: @int
    
fn current(&self) -> String {
        \"\"\"Get current character without advancing\"\"\"
        if self.position >= self.length {
            return \"\"
        return self.source[self.position]
    
fn peek(&self, offset { i64) -> String:
        \"\"\"Peek ahead by offset characters\"\"\"
        let pos = self.position + offset;
        if pos >= self.length {
            return \"\"
        return self.source[pos]
    
fn advance(&mut &self) -> String {
        \"\"\"Advance && return current character\"\"\"
        if self.position >= self.length {
            return \"\"
        
        let ch = self.source[self.position];
        self.position = self.position + 1
        
        if ch == \"\\n\" {
            self.line = self.line + 1
            self.column = 1
        else {
            self.column = self.column + 1
        
        return ch
    
fn skip_whitespace(&mut &self) {
        \"\"\"Skip whitespace except newlines\"\"\"
        while self.position < self.length {
            let ch = self.current();
            if ch == \" \" || ch == \"\\t\" || ch == \"\\r\" {
                self.advance()
            else {
                break
    
fn skip_comment(&mut &self) {
        \"\"\"Skip single-line comment\"\"\"
        // Comment starts with //
        while self.position < self.length && self.current() != \"\\n\" {
            self.advance()
    
fn bool is_at_end(&self) -> bool {
        return self.position >= self.length

// Main Lexer struct
struct Lexer {
    stream: CharStream
    tokens: @list[Token]
    indent_stack: @list[@int]
    current_indent: @int
    
fn tokenize(source { String) -> Vec[Token]:
        \"\"\"Main entry point: tokenize source code\"\"\"
        let mut lexer = Lexer{;
            stream: CharStream{
                source: source,
                position: @int(0),
                line: @int(1),
                column: @int(1),
                length: @int(len(source))
            },
            tokens: @list[],
            indent_stack: @list[@int(0)],
            current_indent: @int(0)
        }
        
        lexer.scan_all()
        return lexer.tokens
    
fn scan_all(&mut &self) {
        \"\"\"Scan entire source && produce tokens\"\"\"
        let mut at_line_start = bool(true);
        
        while ! self.stream.is_at_end() {
            // Handle indentation at line start
            if at_line_start {
                self.handle_indentation()
                at_line_start = @bool(false)
            
            // Skip whitespace (not newlines)
            self.stream.skip_whitespace()
            
            if self.stream.is_at_end() {
                break
            
            let ch = self.stream.current();
            
            // Comments
            if ch == \"#\" {
                self.stream.skip_comment()
                continue
            
            // Newlines
            if ch == \"\\n\" {
                self.add_token(TokenType.Newline, \"\\n\")
                self.stream.advance()
                at_line_start = @bool(true)
                continue
            
            // Scan next token
            self.scan_token()
        
        // Emit remaining dedents
        while len(self.indent_stack) > 1 {
            self.indent_stack.remove(-1)
            self.add_token(TokenType.Dedent, \"\")
        
        // EOF token
        self.add_token(TokenType.Eof, \"\")
    
fn handle_indentation(&mut &self) {
        \"\"\"Handle indentation-based scoping\"\"\"
        let mut indent_level = i64(0);
        
        while self.stream.current() == \" \" || self.stream.current() == \"\\t\" {
            if self.stream.current() == \" \" {
                indent_level = indent_level + 1
            else {
                indent_level = indent_level + 4  # Tab = 4 spaces
            self.stream.advance()
        
        // Skip blank lines and comments
        if self.stream.current() == \"\\n\" || self.stream.current() == \"#\" {
            return
        
        let last_indent = self.indent_stack[-1];
        
        if indent_level > last_indent {
            // Indent
            self.indent_stack.add(indent_level)
            self.add_token(TokenType.Indent, \"\")
        else if indent_level < last_indent {
            // Dedent (possibly multiple levels)
            while len(self.indent_stack) > 0 && self.indent_stack[-1] > indent_level {
                self.indent_stack.remove(-1)
                self.add_token(TokenType.Dedent, \"\")

            // BUGFIX: Validate dedent aligns with previous indent level
            if len(self.indent_stack) > 0 && self.indent_stack[-1] != indent_level {
                // Indentation doesn't match any outer level
                print(format!("Warning: Inconsistent indentation at {self.stream.line}:{self.stream.column}"))
                print(format!("  Expected indent level to match {self.indent_stack[-1]}, got {indent_level}"))
    
fn scan_token(&mut &self) {
        \"\"\"Scan a single token\"\"\"
        let ch = self.stream.current();
        
        // Numbers
        if self.is_digit(ch) {
            self.scan_number()
            return
        
        // Identifiers and keywords
        if self.is_alpha(ch) {
            self.scan_identifier()
            return
        
        // Strings
        if ch == \"\\\"\" || ch == \"'\" {
            self.scan_string()
            return
        
        // Operators and punctuation
        self.scan_operator()
    
fn scan_number(&mut &self) {
        \"\"\"Scan integer || float literal\"\"\"
        let mut num_str = String(\"\");
        let mut is_float = bool(false);
        
        // Collect digits
        while self.is_digit(self.stream.current()) {
            num_str = num_str + self.stream.advance()
        
        // Check for decimal point
        if self.stream.current() == \".\" && self.is_digit(self.stream.peek(1)) {
            is_float = @bool(true)
            num_str = num_str + self.stream.advance()  # Add '.'
            
            while self.is_digit(self.stream.current()) {
                num_str = num_str + self.stream.advance()
        
        // Check for scientific notation
        if self.stream.current() == \"e\" || self.stream.current() == \"E\" {
            is_float = @bool(true)
            num_str = num_str + self.stream.advance()
            
            if self.stream.current() == \"+\" || self.stream.current() == \"-\" {
                num_str = num_str + self.stream.advance()
            
            while self.is_digit(self.stream.current()) {
                num_str = num_str + self.stream.advance()
        
        if is_float {
            self.add_token(TokenType.Float, num_str)
        else {
            self.add_token(TokenType.Integer, num_str)
    
fn scan_identifier(&mut &self) {
        \"\"\"Scan identifier || keyword\"\"\"
        let mut ident = String(\"\");
        
        // Handle @ prefix for decorators/type constructors
        if self.stream.current() == \"@\" {
            ident = ident + self.stream.advance()
        
        // Collect alphanumeric and underscore
        while self.is_alphanumeric(self.stream.current()) {
            ident = ident + self.stream.advance()
        
        // Check if it's a decorator
        if is_decorator(ident) {
            self.add_token(get_decorator_type(ident), ident)
            return
        
        // Check if it's a type constructor
        if is_type_constructor(ident) {
            self.add_token(get_type_constructor_type(ident), ident)
            return
        
        // Check if it's a keyword
        if is_keyword(ident) {
            self.add_token(get_keyword_type(ident), ident)
            return
        
        // Otherwise it's an identifier
        self.add_token(TokenType.Identifier, ident)
    
fn scan_string(&mut &self) {
        \"\"\"Scan string literal\"\"\"
        let quote = self.stream.advance()  # Opening quote;
        let mut string_val = String(\"\");
        
        while ! self.stream.is_at_end() && self.stream.current() != quote {
            // BUGFIX: Changed from "\\\\\" to "\\" for single backslash check
            if self.stream.current() == \"\\" {
                // Escape sequence
                self.stream.advance()
                let escaped = self.stream.advance();
                
                match escaped {
                    \"n\" => string_val = string_val + \"\\n\"
                    \"t\" => string_val = string_val + \"\\t\"
                    \"r\" => string_val = string_val + \"\\r\"
                    \"\\\"\" => string_val = string_val + \"\\\"\"
                    \"'\" => string_val = string_val + \"'\"
                    \"\\\\\" => string_val = string_val + \"\\\\\"
                    _ => string_val = string_val + escaped
            else {
                string_val = string_val + self.stream.advance()
        
        // Closing quote
        if ! self.stream.is_at_end() {
            self.stream.advance()
        
        self.add_token(TokenType.String, string_val)
    
fn scan_operator(&mut &self) {
        \"\"\"Scan operators && punctuation\"\"\"
        let ch = self.stream.advance();
        
        match ch {
            \"+\" => {
                if self.stream.current() == \"=\" {
                    self.stream.advance()
                    self.add_token(TokenType.PlusEq, \"+=\")
                else {
                    self.add_token(TokenType.Plus, \"+\")
            }
            \"-\" => {
                if self.stream.current() == \"=\" {
                    self.stream.advance()
                    self.add_token(TokenType.MinusEq, \"-=\")
                else if self.stream.current() == \">\" {
                    self.stream.advance()
                    self.add_token(TokenType.Arrow, \"->\")
                else {
                    self.add_token(TokenType.Minus, \"-\")
            }
            \"*\" => {
                if self.stream.current() == \"=\" {
                    self.stream.advance()
                    self.add_token(TokenType.StarEq, \"*=\")
                else if self.stream.current() == \"*\" {
                    self.stream.advance()
                    self.add_token(TokenType.DoubleStar, \"**\")
                else {
                    self.add_token(TokenType.Star, \"*\")
            }
            \"/\" => {
                if self.stream.current() == \"=\" {
                    self.stream.advance()
                    self.add_token(TokenType.SlashEq, \"/=\")
                else {
                    self.add_token(TokenType.Slash, \"/\")
            }
            \"%\" => self.add_token(TokenType.Percent, \"%\")
            \"=\" => {
                if self.stream.current() == \"=\" {
                    self.stream.advance()
                    self.add_token(TokenType.EqualEqual, \"==\")
                else if self.stream.current() == \">\" {
                    self.stream.advance()
                    self.add_token(TokenType.FatArrow, \"=>\")
                else {
                    self.add_token(TokenType.Equal, \"=\")
            }
            \"!\" => {
                if self.stream.current() == \"=\" {
                    self.stream.advance()
                    self.add_token(TokenType.NotEqual, \"!=\")
            }
            \">\" => {
                if self.stream.current() == \"=\" {
                    self.stream.advance()
                    self.add_token(TokenType.GreaterEq, \">=\")
                else {
                    self.add_token(TokenType.Greater, \">\")
            }
            \"<\" => {
                if self.stream.current() == \"=\" {
                    self.stream.advance()
                    self.add_token(TokenType.LessEq, \"<=\")
                else {
                    self.add_token(TokenType.Less, \"<\")
            }
            \"&\" => {
                if self.stream.current() == \"&\" {
                    self.stream.advance()
                    self.add_token(TokenType.And, \"&&\")
            }
            \"|\" => {
                if self.stream.current() == \"|\" {
                    self.stream.advance()
                    self.add_token(TokenType.Or, \"||\")
            }
            \"(\" => self.add_token(TokenType.LeftParen, \"(\")
            \")\" => self.add_token(TokenType.RightParen, \")\")
            \"{\" => self.add_token(TokenType.LeftBrace, \"{\")
            \"}\" => self.add_token(TokenType.RightBrace, \"}\")
            \"[\" => self.add_token(TokenType.LeftBracket, \"[\")
            \"]\" => self.add_token(TokenType.RightBracket, \"]\")
            \",\" => self.add_token(TokenType.Comma, \",\")
            \":\" => self.add_token(TokenType.Colon, \":\")
            \";\" => self.add_token(TokenType.Semicolon, \";\")
            \".\" => {
                if self.stream.current() == \".\" {
                    self.stream.advance()
                    if self.stream.current() == \"=\" {
                        self.stream.advance()
                        self.add_token(TokenType.DotDotEq, \"..=\")
                    else {
                        self.add_token(TokenType.DotDot, \"..\")
                else {
                    self.add_token(TokenType.Dot, \".\")
            }
    
fn add_token(&mut &self, type { TokenType, value: String):
        \"\"\"Add token to token list\"\"\"
        let token = create_token(type, value, self.stream.line, self.stream.column);
        self.tokens.add(token)
    
fn bool is_digit(&self, ch { String) -> bool:
        if len(ch) == 0 {
            return @bool(false)
        return ch >= \"0\" && ch <= \"9\"
    
fn bool is_alpha(&self, ch { String) -> bool:
        if len(ch) == 0 {
            return @bool(false)
        return (ch >= \"a\" && ch <= \"z\") || (ch >= \"A\" && ch <= \"Z\") || ch == \"_\" || ch == \"@\"
    
fn bool is_alphanumeric(&self, ch { String) -> bool:
        return self.is_alpha(ch) || self.is_digit(ch)

// Public API
fn tokenize(source { String) -> Vec[Token]:
    \"\"\"Tokenize GUL source code into tokens\"\"\"
    return Lexer.tokenize(source)
