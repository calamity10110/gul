// GUL v3.2 Compiler - Lexer Implementation
use crate::lexer::token::*;

pub fn tokenize(source: String)  ->  Vec<Token> {
    let mut l_pos = 0usize;
    let chars: Vec<char> = source.chars().collect();
    let l_len = chars.len();
    let mut l_tokens = vec![];
    let mut indent_stack = vec![0];
    let mut at_line_start = true;
    let mut line = 1usize;
    let mut column = 1usize;
    let mut bracket_level = 0usize;

    while l_pos < l_len {
        // Handle indentation at start of line
        if at_line_start {
            let mut indent = 0usize;
            let mut temp_pos = l_pos;
            let mut temp_col = column;
            
            while temp_pos < l_len {
                let c = chars[temp_pos];
                if c == ' ' { 
                    indent += 1;
                    temp_col += 1;
                }
                else if c == '\t' { 
                    indent += 4;
                    temp_col += 4;
                }
                else if c == '\r' { 
                    // Ignore
                }
                else { break }
                temp_pos += 1;
            }
            
            if temp_pos < l_len {
                let next_c = chars[temp_pos];
                // Don't generate indent/dedent for empty lines or comments
                if next_c != '\n' && next_c != '#' && bracket_level == 0 {
                    let last_indent = *indent_stack.last().unwrap();
                    if indent > last_indent {
                        indent_stack.push(indent);
                        l_tokens.push(Token{token_type: TokenType::Indent, value: "".to_string(), line, column: 1});
                    }
                    else if indent < last_indent {
                        while indent_stack.len() > 1 && *indent_stack.last().unwrap() > indent {
                            indent_stack.pop();
                            l_tokens.push(Token{token_type: TokenType::Dedent, value: "".to_string(), line, column: 1});
                        }
                        // Note: Python requires the dedent to match an earlier indent level.
                        // For now we just go down as far as it matches or below.
                    }
                }
            }
            
            l_pos = temp_pos;
            column = temp_col;
            at_line_start = false;
            
            if l_pos >= l_len { break; }
        }

        let ch_char = chars[l_pos];
        let ch = ch_char.to_string();

        if ch_char == ' ' || ch_char == '\t' || ch_char == '\r' {
            l_pos += 1;
            column += 1;
            continue;
        }
        else if ch_char == '\n' {
            l_tokens.push(Token{token_type: TokenType::Newline, value: "\n".to_string(), line, column});
            l_pos += 1;
            line += 1;
            column = 1;
            at_line_start = true;
            continue;
        }
        else if ch_char == '#' {
            while l_pos < l_len && chars[l_pos] != '\n' {
                l_pos += 1;
            }
            // Let the next iteration handle the newline/at_line_start
            at_line_start = true; 
            column = 1; // Wait, actually the comment might start in middle of line
            // But if it ends the line, we are at line start for next line.
            // If we are at EOF, then next iteration will break.
            continue;
        }
        else if ch_char == '"' {
            let start_col = column;
            let mut s_val = "".to_string();
            l_pos += 1; column += 1;
            
            while l_pos < l_len {
                let c = chars[l_pos];
                if c == '"' {
                    l_pos += 1; column += 1;
                    break;
                }
                else if c == '\\' {
                    if l_pos + 1 < l_len {
                        let next = chars[l_pos + 1];
                        match next {
                            'n' => s_val.push('\n'),
                            't' => s_val.push('\t'),
                            'r' => s_val.push('\r'),
                            '\\' => s_val.push('\\'),
                            '\"' => s_val.push('\"'),
                            _ => { s_val.push('\\'); s_val.push(next); }
                        }
                        l_pos += 2; column += 2;
                    } else {
                        s_val.push('\\');
                        l_pos += 1; column += 1;
                    }
                }
                else {
                    s_val.push(c);
                    l_pos += 1; column += 1;
                    if c == '\n' {
                        line += 1;
                        column = 1;
                    }
                }
            }
            l_tokens.push(Token{token_type: TokenType::String, value: s_val, line, column: start_col});
        }
        else if ch_char.is_alphabetic() || ch_char == '_' || ch_char == '@' {
            let start_col = column;
            let mut ident = ch;
            l_pos += 1; column += 1;
            
            while l_pos < l_len {
                let c = chars[l_pos];
                if c.is_alphanumeric() || c == '_' || c == '@' {
                    ident.push(c);
                    l_pos += 1; column += 1;
                }
                else { break }
            }
            
            let mut t = TokenType::Identifier;
            if is_keyword(ident.clone()) {
                t = get_keyword_type(ident.clone());
            } else if is_type_constructor(ident.clone()) {
                t = get_type_constructor_type(ident.clone());
            } else if get_decorator_type(ident.clone()) != TokenType::Error {
                t = get_decorator_type(ident.clone());
            }
            
            l_tokens.push(Token{token_type: t, value: ident, line, column: start_col});
        }
        else if ch_char.is_ascii_digit() {
            let start_col = column;
            let mut num = ch;
            l_pos += 1; column += 1;
            let mut is_float = false;
            
            while l_pos < l_len {
                let c = chars[l_pos];
                if c.is_ascii_digit() {
                    num.push(c);
                    l_pos += 1; column += 1;
                }
                else if c == '.' && !is_float {
                    if l_pos + 1 < l_len && chars[l_pos + 1].is_ascii_digit() {
                        is_float = true;
                        num.push('.');
                        l_pos += 1; column += 1;
                    } else {
                        break;
                    }
                }
                else { break }
            }
            
            if is_float {
                l_tokens.push(Token{token_type: TokenType::Float, value: num, line, column: start_col});
            } else {
                l_tokens.push(Token{token_type: TokenType::Integer, value: num, line, column: start_col});
            }
        }
        else {
            let start_col = column;
            let mut t_p = TokenType::Error;
            let mut val = ch.clone();
            
            let next_char = if l_pos + 1 < l_len { Some(chars[l_pos + 1]) } else { None };
            
            if ch_char == '|' && next_char == Some('>') {
                t_p = TokenType::Pipeline;
                val = "|>".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '-' && next_char == Some('>') {
                t_p = TokenType::Arrow;
                val = "->".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '-' && next_char == Some('=') {
                t_p = TokenType::MinusEq;
                val = "-=".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '=' && next_char == Some('>') {
                t_p = TokenType::FatArrow;
                val = "=>".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '=' && next_char == Some('=') {
                t_p = TokenType::EqualEqual;
                val = "==".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '!' && next_char == Some('=') {
                t_p = TokenType::NotEqual;
                val = "!=".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '>' && next_char == Some('=') {
                t_p = TokenType::GreaterEq;
                val = ">=".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '<' && next_char == Some('=') {
                t_p = TokenType::LessEq;
                val = "<=".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '+' && next_char == Some('=') {
                t_p = TokenType::PlusEq;
                val = "+=".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '*' && next_char == Some('=') {
                t_p = TokenType::StarEq;
                val = "*=".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '*' && next_char == Some('*') {
                t_p = TokenType::DoubleStar;
                val = "**".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '/' && next_char == Some('=') {
                t_p = TokenType::SlashEq;
                val = "/=".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '&' && next_char == Some('&') {
                t_p = TokenType::And;
                val = "&&".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '|' && next_char == Some('|') {
                t_p = TokenType::Or;
                val = "||".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '|' && next_char == Some('>') {
                t_p = TokenType::Pipeline;
                val = "|>".to_string();
                l_pos += 2; column += 2;
            }
            else if ch_char == '.' && next_char == Some('.') {
                t_p = TokenType::DotDot;
                val = "..".to_string();
                l_pos += 2; column += 2;
            }
            else {
                match ch_char {
                    '+' => t_p = TokenType::Plus,
                    '-' => t_p = TokenType::Minus,
                    '*' => t_p = TokenType::Star,
                    '/' => t_p = TokenType::Slash,
                    '=' => t_p = TokenType::Equal,
                    '>' => t_p = TokenType::Greater,
                    '<' => t_p = TokenType::Less,
                    ':' => t_p = TokenType::Colon,
                    ',' => t_p = TokenType::Comma,
                    '.' => t_p = TokenType::Dot,
                    '!' => t_p = TokenType::Not,
                    '%' => t_p = TokenType::Percent,
                    '(' => { t_p = TokenType::LeftParen; bracket_level += 1; }
                    ')' => { t_p = TokenType::RightParen; if bracket_level > 0 { bracket_level -= 1; } }
                    '[' => { t_p = TokenType::LeftBracket; bracket_level += 1; }
                    ']' => { t_p = TokenType::RightBracket; if bracket_level > 0 { bracket_level -= 1; } }
                    '{' => { t_p = TokenType::LeftBrace; bracket_level += 1; }
                    '}' => { t_p = TokenType::RightBrace; if bracket_level > 0 { bracket_level -= 1; } }
                    _ => t_p = TokenType::Error,
                }
                l_pos += 1;
                column += 1;
            }
            
            l_tokens.push(Token{token_type: t_p, value: val, line, column: start_col});
        }
    }
    
    // Final dedents
    while indent_stack.len() > 1 {
        indent_stack.pop();
        l_tokens.push(Token{token_type: TokenType::Dedent, value: "".to_string(), line, column: 1});
    }
    l_tokens.push(Token{token_type: TokenType::Eof, value: "".to_string(), line, column: 1});
    
    l_tokens
}