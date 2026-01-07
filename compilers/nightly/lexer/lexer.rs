// GUL v3.2 Compiler - Lexer Implementation
use crate::lexer::token::*;

pub fn tokenize(source: String)  ->  Vec<Token> {
    let mut l_pos = 0usize;
    let mut l_len = (source).len();
    let mut l_tokens = vec![];
    let mut indent_stack = vec![0];
    let mut at_line_start = true;

    while l_pos < l_len {
        let ch = source.chars().nth(l_pos).unwrap().to_string();

        // Handle indentation at start of line
        // Handle indentation at start of line
        // (Removed lines 15-20 which prematurely cleared at_line_start)
        if ch == " ".to_string() || ch == "\t".to_string() || ch == "\r".to_string() {
            if at_line_start {
                let mut indent = 0usize;
                while l_pos < l_len {
                    let c = source.chars().nth(l_pos).unwrap().to_string();
                    if c == " ".to_string() { indent = indent + 1 }
                    else if c == "\t".to_string() { indent = indent + 4 }
                    else if c == " ".to_string() || c == "\t".to_string() || c == "\r".to_string() { indent = indent + 0 }
                    else { break }
                    l_pos = l_pos + 1;

                // After whitespace, check if it's a real line
                }
                if l_pos < l_len {
                    let c2 = source.chars().nth(l_pos).unwrap().to_string();
                    if c2 != "\n".to_string() && c2 != "\x23".to_string() {
                        let s_size = (indent_stack).len();
                        let last_indent = indent_stack[s_size - 1];
                        if indent > last_indent {
                            indent_stack.push(indent);
                            l_tokens.push(Token{token_type: TokenType::Indent, value: "".to_string(), line: 1, column: 1});
                        }
                        else if indent < last_indent {
                            let mut cur_size = (indent_stack).len();
                            while cur_size > 1 && indent_stack[cur_size - 1] > indent {
                                indent_stack.remove(cur_size - 1);
                                l_tokens.push(Token{token_type: TokenType::Dedent, value: "".to_string(), line: 1, column: 1});
                                cur_size = (indent_stack).len();
                            }
                        }
                    }
                }
                at_line_start = false;
            }
            else {
                l_pos = l_pos + 1;
            }
        }
        else if ch == "\n".to_string() {
            l_tokens.push(Token{token_type: TokenType::Newline, value: "\n".to_string(), line: 1, column: 1});
            l_pos = l_pos + 1;
            at_line_start = true;
        }
        else if ch == "\x23".to_string() {
            while l_pos < l_len && source.chars().nth(l_pos).unwrap().to_string() != "\n".to_string() {
                l_pos = l_pos + 1;
            }
        }
        else if ch == "\"".to_string() {
            // String literal
             // If we were at line start with 0 indent
            if at_line_start {
                while (indent_stack).len() > 1 {
                    indent_stack.remove((indent_stack).len()-1);
                    l_tokens.push(Token{token_type: TokenType::Dedent, value: "".to_string(), line: 1, column: 1});
                }
                at_line_start = false;
            }
            
            let mut s_val = "".to_string();
            l_pos = l_pos + 1; // Skip opening quote
            while l_pos < l_len {
                let c = source.chars().nth(l_pos).unwrap();
                if c == '"' {
                    l_pos = l_pos + 1; // Skip closing quote
                    break;
                }
                else if c == '\\' {
                    if l_pos + 1 < l_len {
                        let next = source.chars().nth(l_pos + 1).unwrap();
                        if next == 'n' { s_val.push('\n'); }
                        else if next == 't' { s_val.push('\t'); }
                        else if next == 'r' { s_val.push('\r'); }
                        else if next == '\\' { s_val.push('\\'); }
                        else if next == '"' { s_val.push('"'); }
                        else { s_val.push(c); s_val.push(next); } // Keep raw if unknown? Or just next? For now strict
                        l_pos = l_pos + 2;
                    } else {
                        s_val.push('\\');
                        l_pos = l_pos + 1;
                    }
                }
                else {
                    s_val.push(c);
                    l_pos = l_pos + 1;
                }
            }
            l_tokens.push(Token{token_type: TokenType::String, value: s_val, line: 1, column: 1});
        }
        else if (ch >= "a".to_string() && ch <= "z".to_string()) || (ch >= "A".to_string() && ch <= "Z".to_string()) || ch == "_".to_string() || ch == "@".to_string() {
            // If we were at line start with 0 indent
            if at_line_start {
                let s_size_0 = (indent_stack).len();
                let last_0 = indent_stack[s_size_0 - 1];
                if last_0 > 0 {
                    while (indent_stack).len() > 1 && indent_stack[(indent_stack).len() - 1] > 0 {
                        indent_stack.remove((indent_stack).len() - 1);
                        l_tokens.push(Token{token_type: TokenType::Dedent, value: "".to_string(), line: 1, column: 1});
                    }
                }
                at_line_start = false;

            }
            let mut ident = ch;
            l_pos = l_pos + 1;
            while l_pos < l_len {
                let c = source.chars().nth(l_pos).unwrap().to_string();
                if (c >= "a".to_string() && c <= "z".to_string()) || (c >= "A".to_string() && c <= "Z".to_string()) || (c >= "0".to_string() && c <= "9".to_string()) || c == "_".to_string() || c == "@".to_string() {
                    ident = ident + &c;
                    l_pos = l_pos + 1;
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
            l_tokens.push(Token{token_type: t, value: ident, line: 1, column: 1});
        }
        else if ch >= "0".to_string() && ch <= "9".to_string() {
            // If we were at line start with 0 indent
            if at_line_start {
                while (indent_stack).len() > 1 {
                    indent_stack.remove((indent_stack).len()-1);
                    l_tokens.push(Token{token_type: TokenType::Dedent, value: "".to_string(), line: 1, column: 1});
                }
                at_line_start = false;
            }
            let mut num = ch;
            l_pos = l_pos + 1;
            let mut is_float = false;
            
            while l_pos < l_len {
                let c = source.chars().nth(l_pos).unwrap().to_string();
                if c >= "0".to_string() && c <= "9".to_string() {
                    num = num + &c;
                    l_pos = l_pos + 1;
                }
                else if c == ".".to_string() && !is_float {
                    // Check if next char is digit? Or allow dangling dot (e.g. 2.)?
                    // For GUL, let's require at least one digit after dot for simplicity, or just allow it.
                    // Let's lookahead.
                    let next_c = if l_pos + 1 < l_len { source.chars().nth(l_pos + 1) } else { None };
                    if let Some(nc) = next_c {
                         let ncs = nc.to_string();
                         if ncs >= "0".to_string() && ncs <= "9".to_string() {
                             is_float = true;
                             num = num + &c;
                             l_pos = l_pos + 1;
                             continue;
                         }
                    }
                    // If dot not followed by digit, it might be method call (e.g. 1.to_string())
                    // So we break if we see a dot that doesn't look like a float part.
                    break;
                }
                else { break }
            }
            if is_float {
                l_tokens.push(Token{token_type: TokenType::Float, value: num, line: 1, column: 1});
            } else {
                l_tokens.push(Token{token_type: TokenType::Integer, value: num, line: 1, column: 1});
            }
        }
        else {
            // If we were at line start with 0 indent
            if at_line_start {
                while (indent_stack).len() > 1 {
                    indent_stack.remove((indent_stack).len()-1);
                    l_tokens.push(Token{token_type: TokenType::Dedent, value: "".to_string(), line: 1, column: 1});
                }
                at_line_start = false;
            }
            let mut t_p = TokenType::Error;
            let mut val = ch.clone();
            
            // Lookahead for multi-char operators
            let next_char = if l_pos + 1 < l_len { source.chars().nth(l_pos + 1) } else { None };
            
            if ch == "|".to_string() && next_char == Some('>') {
                t_p = TokenType::Pipeline;
                val = "|>".to_string();
                l_pos = l_pos + 1; // Skip next char
            }
            else if ch == "-".to_string() {
                if next_char == Some('>') {
                     t_p = TokenType::Arrow;
                     val = "->".to_string();
                     l_pos = l_pos + 1;
                } else if next_char == Some('=') {
                     t_p = TokenType::MinusEq;
                     val = "-=".to_string();
                     l_pos = l_pos + 1;
                } else {
                     t_p = TokenType::Minus;
                }
            }
            else if ch == "=".to_string() {
                if next_char == Some('>') {
                     t_p = TokenType::FatArrow;
                     val = "=>".to_string();
                     l_pos = l_pos + 1;
                } else if next_char == Some('=') {
                     t_p = TokenType::EqualEqual;
                     val = "==".to_string();
                     l_pos = l_pos + 1;
                } else {
                     t_p = TokenType::Equal;
                }
            }
            else if ch == "!".to_string() {
                if next_char == Some('=') {
                     t_p = TokenType::NotEqual;
                     val = "!=".to_string();
                     l_pos = l_pos + 1;
                } else {
                     t_p = TokenType::Not;
                }
            }
            else if ch == ">".to_string() {
                if next_char == Some('=') {
                     t_p = TokenType::GreaterEq;
                     val = ">=".to_string();
                     l_pos = l_pos + 1;
                } else {
                     t_p = TokenType::Greater;
                }
            }
            else if ch == "<".to_string() {
                if next_char == Some('=') {
                     t_p = TokenType::LessEq;
                     val = "<=".to_string();
                     l_pos = l_pos + 1;
                } else {
                     t_p = TokenType::Less;
                }
            }
            else if ch == "+".to_string() {
                if next_char == Some('=') {
                     t_p = TokenType::PlusEq;
                     val = "+=".to_string();
                     l_pos = l_pos + 1;
                } else {
                     t_p = TokenType::Plus;
                }
            }
            else if ch == "*".to_string() {
                if next_char == Some('=') {
                     t_p = TokenType::StarEq;
                     val = "*=".to_string();
                     l_pos = l_pos + 1;
                } else if next_char == Some('*') {
                     t_p = TokenType::DoubleStar;
                     val = "**".to_string();
                     l_pos = l_pos + 1;
                } else {
                     t_p = TokenType::Star;
                }
            }
            else if ch == "/".to_string() {
                if next_char == Some('=') {
                     t_p = TokenType::SlashEq;
                     val = "/=".to_string();
                     l_pos = l_pos + 1;
                } else {
                     t_p = TokenType::Slash;
                }
            }
            else if ch == ".".to_string() {
                if next_char == Some('.') {
                    t_p = TokenType::DotDot;
                    val = "..".to_string();
                    l_pos = l_pos + 1;
                    // TODO: Handle ..= ?
                } else {
                    t_p = TokenType::Dot;
                }
            }
            else if ch == "\x3a".to_string() { t_p = TokenType::Colon }
            else if ch == ",".to_string() { t_p = TokenType::Comma }
            else if ch == "(".to_string() { t_p = TokenType::LeftParen }
            else if ch == ")".to_string() { t_p = TokenType::RightParen }
            else if ch == "{".to_string() { t_p = TokenType::LeftBrace }
            else if ch == "}".to_string() { t_p = TokenType::RightBrace }
            else if ch == "[".to_string() { t_p = TokenType::LeftBracket }
            else if ch == "]".to_string() { t_p = TokenType::RightBracket }
            else if ch == "|".to_string() { 
                t_p = TokenType::Error; 
            }
            
            l_tokens.push(Token{token_type: t_p, value: val, line: 1, column: 1});
            l_pos = l_pos + 1;

        }
    }
    while (indent_stack).len() > 1 {
        indent_stack.remove((indent_stack).len()-1);
        l_tokens.push(Token{token_type: TokenType::Dedent, value: "".to_string(), line: 1, column: 1});
    }
    l_tokens.push(Token{token_type: TokenType::Eof, value: "".to_string(), line: 1, column: 1});
    return l_tokens;
}