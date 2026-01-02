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
        if at_line_start && ch != " ".to_string() && ch != "\t".to_string() && ch != "\n".to_string() && ch != "\x23".to_string() {
            // We reached a non-space character at the start of a line (after potential spaces)
            // Wait, this logic needs to know how many spaces were there!
            at_line_start = false; // Should have handled it before

        }
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
                            l_tokens.push(Token{token_type: TokenType::Indent, value: "", line: 1, column: 1});
                        }
                        else if indent < last_indent {
                            let mut cur_size = (indent_stack).len();
                            while cur_size > 1 && indent_stack[cur_size - 1] > indent {
                                indent_stack.remove(cur_size - 1);
                                l_tokens.push(Token{token_type: TokenType::Dedent, value: "", line: 1, column: 1});
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
            l_tokens.push(Token{token_type: TokenType::Newline, value: "\n", line: 1, column: 1});
            l_pos = l_pos + 1;
            at_line_start = true;
        }
        else if ch == "\x23".to_string() {
            while l_pos < l_len && source.chars().nth(l_pos).unwrap().to_string() != "\n".to_string() {
                l_pos = l_pos + 1;
            }
        }
        else if (ch >= "a".to_string() && ch <= "z".to_string()) || (ch >= "A".to_string() && ch <= "Z".to_string()) || ch == "_".to_string() || ch == "@".to_string() {
            // If we were at line start with 0 indent
            if at_line_start {
                let s_size_0 = (indent_stack).len();
                let last_0 = indent_stack[s_size_0 - 1];
                if last_0 > 0 {
                    while (indent_stack).len() > 1 && indent_stack[(indent_stack).len() - 1] > 0 {
                        indent_stack.remove((indent_stack).len() - 1);
                        l_tokens.push(Token{token_type: TokenType::Dedent, value: "", line: 1, column: 1});
                    }
                }
                at_line_start = false;

            }
            let mut ident = ch;
            l_pos = l_pos + 1;
            while l_pos < l_len {
                let c = source.chars().nth(l_pos).unwrap().to_string();
                if (c >= "a".to_string() && c <= "z".to_string()) || (c >= "A".to_string() && c <= "Z".to_string()) || (c >= "0".to_string() && c <= "9".to_string()) || c == "_".to_string() || c == "@".to_string() {
                    ident = ident + c;
                    l_pos = l_pos + 1;
                }
                else { break }
            }
            let mut t = TokenType::Identifier;
            if token.is_keyword(ident) {
                t = token.get_keyword_type(ident);
            }
            l_tokens.push(Token{token_type: t, value: ident, line: 1, column: 1});
        }
        else if ch >= "0".to_string() && ch <= "9".to_string() {
            // If we were at line start with 0 indent
            if at_line_start {
                while (indent_stack).len() > 1 {
                    indent_stack.remove((indent_stack).len()-1);
                    l_tokens.push(Token{token_type: TokenType::Dedent, value: "", line: 1, column: 1});
                }
                at_line_start = false;
            }
            let mut num = ch;
            l_pos = l_pos + 1;
            while l_pos < l_len {
                let c = source.chars().nth(l_pos).unwrap().to_string();
                if c >= "0".to_string() && c <= "9".to_string() {
                    num = num + c;
                    l_pos = l_pos + 1;
                }
                else { break }
            }
            l_tokens.push(Token{token_type: TokenType::Integer, value: num, line: 1, column: 1});
        }
        else {
            // If we were at line start with 0 indent
            if at_line_start {
                while (indent_stack).len() > 1 {
                    indent_stack.remove((indent_stack).len()-1);
                    l_tokens.push(Token{token_type: TokenType::Dedent, value: "", line: 1, column: 1});
                }
                at_line_start = false;
            }
            let mut t_p = TokenType::Error;
            if ch == "+".to_string() { t_p = TokenType::Plus }
            else if ch == "-".to_string() { t_p = TokenType::Minus }
            else if ch == "*".to_string() { t_p = TokenType::Star }
            else if ch == "/".to_string() { t_p = TokenType::Slash }
            else if ch == "=".to_string() { t_p = TokenType::Equal }
            else if ch == "\x3a".to_string() { t_p = TokenType::Colon }
            else if ch == ",".to_string() { t_p = TokenType::Comma }
            else if ch == "(".to_string() { t_p = TokenType::LeftParen }
            else if ch == ")".to_string() { t_p = TokenType::RightParen }
            l_tokens.push(Token{token_type: t_p, value: ch, line: 1, column: 1});
            l_pos = l_pos + 1;

        }
    }
    while (indent_stack).len() > 1 {
        indent_stack.remove((indent_stack).len()-1);
        l_tokens.push(Token{token_type: TokenType::Dedent, value: "", line: 1, column: 1});
    }
    l_tokens.push(Token{token_type: TokenType::Eof, value: "", line: 1, column: 1});
    return l_tokens;
}